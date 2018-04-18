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
use json::JsonValue;
use separator::Separatable;
use std::str;
use std::u64;
use stopwatch::Stopwatch;

mod gethrpc;
mod evminst;

///
/// Collect EVM statistics
///
struct EvmExtract {
    rpc: GethRpc,
    current_block: u64,
    txn_counts: [u64; 256],
    block_counts: [u64; 256],
    total_counts: [u64; 256],
    txn_gas: [u64; 256],
    block_gas: [u64; 256],
    total_gas: [u64; 256],
    pretty_string: String
}

impl EvmExtract {
    fn new(rpc_path: &str) -> Self {
        EvmExtract {
            rpc: GethRpc::new(rpc_path),
            current_block: 1_000_000,
            txn_counts: [0; 256],
            block_counts: [0; 256],
            total_counts: [0; 256],
            txn_gas: [0; 256],
            block_gas: [0; 256],
            total_gas: [0; 256],
            pretty_string: String::with_capacity(2048)
        }
    }

    /// Latest synchronized block number from Geth `eth.syncing` call
    fn get_latest_block(&mut self) -> Option<u64> {
        let data = match self.rpc.syncing() {
            Ok(v) => v,
            Err(e) => {
                println!("Error syncing(): {:?}", e);
                return None;
            }
        };

        match data["result"]["currentBlock"].as_str() {
            Some(v) => EvmExtract::hex_to_u64(v),
            _ => None
        }
    }

    fn catchup(&mut self) {
        let latest_block = match self.get_latest_block() {
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

            let (total, trace) = self.rpc.trace_block(block_num);
            let result = &trace.unwrap()["result"];

            if result.is_empty() {
                continue;
            }

            self.clear_block_counts();
            let num_traces = result.len();

            for idx in 0..num_traces {
                self.clear_txn_counts();

                let inner_result = &result[idx]["result"];
                let total_trace_gas = &inner_result["gas"];
                let trace_logs = &inner_result["structLogs"];

                self.count_instructions(block_num, idx as u64, trace_logs);
            }

            info!("Block {} read {} with {} traces in {}ms",
                  block_num.separated_string(), ByteSize::b(total), num_traces, sw.elapsed_ms()
            );
        }

        self.current_block = latest_block;
    }

    fn count_instructions(&mut self, block_num: u64, txn_idx: u64, trace_logs: &JsonValue) {
        if trace_logs.is_empty() {
            return;
        }

        let txn_info = &self.rpc.txn_by_block_idx(block_num, txn_idx).unwrap_or(JsonValue::Null)["result"];

        let from = txn_info["from"].as_str().unwrap_or("");
        let to = txn_info["to"].as_str().unwrap_or("");
        let gas_price = {
            let px = txn_info["gasPrice"].as_str().unwrap_or("0x0");
            EvmExtract::hex_to_u64(px).unwrap_or(0)
        };

        for trace in trace_logs.members() {
            let op = EvmInst::from_opt_str(trace["op"].as_str());
            let gas_cost = trace["gasCost"].as_u64().expect("gasCost extract failed");

            self.txn_counts[op as usize] += 1;
            self.block_counts[op as usize] += 1;
            self.total_counts[op as usize] += 1;
            self.txn_gas[op as usize] += gas_cost;
            self.block_gas[op as usize] += gas_cost;
            self.total_gas[op as usize] += gas_cost;
        }

        info!("{}:{} from:{}, to:{}, px:{}, counts {}",
              block_num, txn_idx, from, to, gas_price, self.pretty_counts());
    }

    fn clear_txn_counts(&mut self) {
        // wtf Rust, no array::fill or equivalent?
        for i in 0..self.txn_counts.len() {
            self.txn_counts[i] = 0;
            self.txn_gas[i] = 0;
        }
    }

    fn clear_block_counts(&mut self) {
        // wtf Rust, no array::fill or equivalent?
        for i in 0..self.block_counts.len() {
            self.block_counts[i] = 0;
            self.block_gas[i] = 0;
        }
    }

    fn pretty_counts(&mut self) -> &String {
        self.pretty_string.clear();
        let spacing: &str = ", ";

        for i in 0..evminst::VALUES.len() {
            let op = evminst::VALUES[i];

            match self.txn_counts[op as usize] {
                0 => (),
                count => {
                    self.pretty_string.push_str(evminst::as_str(&op));
                    self.pretty_string.push(':');
                    self.pretty_string.push_str(count.to_string().as_ref());
                    self.pretty_string.push_str(spacing);
                }
            }
        }

        &self.pretty_string
    }

    fn hex_to_u64(s: &str) -> Option<u64> {
        if s.len() > 2 {
            // skip leading 0x
            match u64::from_str_radix(&s[2..], 16) {
                Ok(val) => Some(val),
                Err(_) => None
            }
        } else {
            None
        }
    }
}


fn main() {
    use log::Level;

    simple_logger::init_with_level(Level::Info).unwrap();

    let mut evm = EvmExtract::new("/home/stuart/.ethereum/geth.ipc");

    loop {
        evm.catchup();
    }
}
