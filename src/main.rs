// Copyright 2018 int08h, LLC all rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//!
//! Very early work to obtain EVM traces from Geth IPC. Work in progress, etc.
//!

extern crate bytesize;
extern crate json;
#[macro_use]
extern crate log;
extern crate separator;
extern crate simple_logger;
extern crate stopwatch;

use bytesize::ByteSize;
use evminst::EvmInst;
use gethrpc::GethRpc;
use instcount::InstCount;
use json::JsonValue;
use separator::Separatable;
use std::str;
use std::u64;
use stopwatch::Stopwatch;

mod gethrpc;
mod evminst;
mod instcount;
mod util;


///
/// Collect EVM statistics
///
struct EvmExtract {
    rpc: GethRpc,
    current_block: u64,
    txn_count: InstCount,
    block_count: InstCount,
    total_count: InstCount,
    pretty_string: String,
}

impl EvmExtract {
    fn new(starting_block: u64, rpc_path: &str) -> Self {
        EvmExtract {
            rpc: GethRpc::new(rpc_path),
            current_block: starting_block,
            txn_count: InstCount::new(),
            block_count: InstCount::new(),
            total_count: InstCount::new(),
            pretty_string: String::with_capacity(4096),
        }
    }

    fn catchup(&mut self) {
        let latest_block = match self.rpc.get_latest_block() {
            Some(v) => v,
            None => return
        };

        if latest_block <= self.current_block {
            return;
        }

        info!("{} blocks to catch-up on (current {}, latest {})",
              (latest_block - self.current_block).separated_string(),
              self.current_block.separated_string(), latest_block.separated_string()
        );

        let mut sw = Stopwatch::new();

        for block_num in self.current_block..latest_block {
            sw.start();

            let block_info = self.rpc.block_info(block_num);
            let (total, trace_resp) = self.rpc.trace_block(block_num);
            let trace = &trace_resp.unwrap_or(JsonValue::Null)["result"];

            if trace.is_empty() {
                continue;
            }

            self.block_count.clear();
            let num_traces = trace.len();

            for idx in 0..num_traces {
                self.txn_count.clear();

                let inner_result = &trace[idx]["result"];
                let total_trace_gas = &inner_result["gas"];
                let trace_logs = &inner_result["structLogs"];

                self.count_instructions(block_num, idx as u32, trace_logs);
            };

            info!("Block {} ({}) read {} with {} traces in {}ms",
                  block_num.separated_string(), block_info.time_stamp, ByteSize::b(total),
                  num_traces, sw.elapsed_ms()
            );
        }

        self.current_block = latest_block;
    }

    fn count_instructions(&mut self, block_num: u64, txn_idx: u32, trace_logs: &JsonValue) {
        if trace_logs.is_empty() {
            return;
        }

        let txn_info = self.rpc.txn_info(block_num, txn_idx);

        for trace in trace_logs.members() {
            let op = EvmInst::from_opt_str(trace["op"].as_str());
            let gas_cost = trace["gasCost"].as_u64().expect("gasCost extract failed");

            self.txn_count.inc_count(op);
            self.txn_count.add_gas(op, gas_cost);

            self.block_count.inc_count(op);
            self.block_count.add_gas(op, gas_cost);

            self.total_count.inc_count(op);
            self.total_count.add_gas(op, gas_cost);
        }

        info!("{}:{} from:{}, to:{}, px:{}, counts {}",
              block_num, txn_idx, txn_info.from, txn_info.to,
              txn_info.gas_price, self.pretty_counts()
        );
    }

    fn pretty_counts(&mut self) -> &String {
        self.pretty_string.clear();
        let spacing: &str = ", ";

        for i in 0..evminst::VALUES.len() {
            let op = evminst::VALUES[i];

            match self.txn_count.get_count(op) {
                0 => (),
                count => {
                    let gas = self.txn_count.get_gas(op);

                    self.pretty_string.push_str(evminst::as_str(&op));
                    self.pretty_string.push(':');
                    self.pretty_string.push_str(count.to_string().as_ref());
                    self.pretty_string.push('(');
                    self.pretty_string.push_str(gas.to_string().as_ref());
                    self.pretty_string.push(')');
                    self.pretty_string.push_str(spacing);
                }
            }
        }

        &self.pretty_string
    }
}

fn main() {
    use log::Level;
    use std::env::args;

    simple_logger::init_with_level(Level::Info).unwrap();

    let argv: Vec<String> = args().collect();

    if argv.len() != 3 {
        info!("Usage: evmobs STARTING_BLOCK IPC_PATH");
        std::process::exit(1);
    }

    let starting_block = argv[1].parse::<u64>().expect("Couldn't parse starting block");
    let ipc_path = &argv[2];

    let mut evm = EvmExtract::new(starting_block, &ipc_path);

    loop {
        evm.catchup();
    }
}
