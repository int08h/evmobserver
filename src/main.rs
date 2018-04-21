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

use bytesize::ByteSize;
use csvoutfile::CsvOutFile;
use evminst::EvmInst;
use gethrpc::BlockInfo;
use gethrpc::GethRpc;
use instcount::InstCount;
use json::JsonValue;
use separator::Separatable;
use std::str;
use std::time::{Duration, Instant};
use std::u64;

mod gethrpc;
mod evminst;
mod instcount;
mod util;
mod csvoutfile;

///
/// Collect EVM statistics
///
struct EvmExtract {
    rpc: GethRpc,
    current_block: u64,
    txn_count: InstCount,
    total_count: InstCount,
    out_file: CsvOutFile,
    last_update: Instant,
}

static TEN_SECONDS: &'static Duration = &Duration::from_secs(10);

impl EvmExtract {

    fn new(starting_block: u64, rpc_path: &str) -> Self {
        EvmExtract {
            rpc: GethRpc::new(rpc_path),
            current_block: starting_block,
            txn_count: InstCount::new(),
            total_count: InstCount::new(),
            out_file: CsvOutFile::new(starting_block),
            last_update: Instant::now(),
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

        for block_num in self.current_block..latest_block {
            let block_info = self.rpc.block_info(block_num);
            let trace_resp = self.rpc.trace_block(block_num);
            let trace = &trace_resp.unwrap_or(JsonValue::Null)["result"];

            if self.last_update.elapsed() > *TEN_SECONDS {
                self.log_update(block_num, latest_block);
                self.last_update = Instant::now();
            }

            if trace.is_empty() {
                continue;
            }

            let num_traces = trace.len();

            for idx in 0..num_traces {
                let trace_logs = &trace[idx]["result"]["structLogs"];
                if !trace_logs.is_empty() {
                    self.count_instructions(idx as u32, trace_logs, &block_info);
                }
            };

            self.current_block = block_num;
        }
    }

    fn log_update(&self, curr_block: u64, latest_block: u64) {
        info!("Wrote block {} ({}) of {} (geth {}): txns {}, minsts {:.1}, mgas {:.1}, written {}",
              self.out_file.last_block.separated_string(),
              self.out_file.last_time_stamp,
              latest_block.separated_string(),
              curr_block.separated_string(),
              self.out_file.total_txns.separated_string(),
              self.out_file.total_inst as f64 / 1_000_000.0,
              self.out_file.total_gas as f64 / 1_000_000.0,
              ByteSize::b(self.out_file.total_written)
        );
    }

    fn count_instructions(&mut self, txn_idx: u32, trace_logs: &JsonValue, block_info: &BlockInfo) {
        self.txn_count.clear();
        let txn_info = self.rpc.txn_info(block_info.block_num, txn_idx);

        for trace in trace_logs.members() {
            let op = EvmInst::from_opt_str(trace["op"].as_str());
            let gas_cost = trace["gasCost"].as_u64().expect("gasCost extract failed");

            self.txn_count.inc_count(op);
            self.txn_count.add_gas(op, gas_cost);

            self.total_count.inc_count(op);
            self.total_count.add_gas(op, gas_cost);
        }

        self.out_file.write_count(&self.txn_count, &txn_info, &block_info)
            .expect("write_count failed");
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
