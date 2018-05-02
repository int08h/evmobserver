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
//! Dumps EVM per-instruction counts and gas consumption.
//!

extern crate bytesize;
extern crate evmobserver;
extern crate json;
#[macro_use]
extern crate log;
extern crate separator;
extern crate simple_logger;

use bytesize::ByteSize;
use evmobserver::csvoutfile::CsvOutFile;
use evmobserver::evminst::EvmInst;
use evmobserver::gethrpc::BlockInfo;
use evmobserver::gethrpc::GethRpc;
use evmobserver::instcount::InstCount;
use json::JsonValue;
use separator::Separatable;
use std::str;
use std::time::{Duration, Instant};
use std::u64;

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

    fn catchup_latest(&mut self) {
        let latest_block = match self.rpc.get_latest_block() {
            Some(v) => v,
            None => panic!("Failed to read latest block from geth, can't proceed")
        };

        info!("Latest block from geth: {}", latest_block.separated_string());

        self.catchup(latest_block)
    }

    fn catchup(&mut self, target_block: u64) {
        if self.current_block >= target_block {
            info!("Catch-up complete: current {}, target {}",
                  self.current_block.separated_string(), target_block.separated_string()
            );
            return;
        }

        info!("{} blocks to catch-up on (current {}, target {})",
              (target_block - self.current_block).separated_string(),
              self.current_block.separated_string(), target_block.separated_string()
        );

        let ten_seconds = Duration::from_secs(10);
        let mut last_update_block = self.current_block;

        for block_num in self.current_block..(target_block + 1) {

            if self.last_update.elapsed() > ten_seconds {
                let block_delta = block_num - last_update_block;
                self.log_update(block_num, target_block, block_delta);
                self.last_update = Instant::now();
                last_update_block = block_num;
            };

            let block_info = self.rpc.block_info(block_num);
            let trace_resp = self.rpc.trace_block(block_num);
            let trace = &trace_resp.unwrap_or(JsonValue::Null)["result"];

            if trace.is_empty() {
                continue;
            };

            for idx in 0..trace.len() {
                let trace_logs = &trace[idx]["result"]["structLogs"];
                if !trace_logs.is_empty() {
                    self.count_instructions(idx as u32, trace_logs, &block_info);
                };
            };

            self.current_block = block_num;
        };
    }

    fn log_update(&self, curr_block: u64, max_block: u64, block_delta: u64) {
        let elapsed = {
            let tmp = self.last_update.elapsed();
            tmp.as_secs() as f64 + (tmp.subsec_nanos() as f64 * 1e-9)
        };
        let blocks_per_sec = block_delta as f64 / elapsed;

        info!("Wrote block {} ({}) of {} (geth {}): {:.1} blks/s, txns {}, minsts {:.1}, mgas {}, written {}",
              self.out_file.last_block.separated_string(),
              self.out_file.last_time_stamp,
              max_block.separated_string(),
              curr_block.separated_string(),
              blocks_per_sec,
              self.out_file.total_txns.separated_string(),
              self.out_file.total_inst as f64 / 1_000_000.0,
              self.out_file.total_gas / 1_000_000,
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

    let starting_block;
    let ending_block;
    let ipc_path;

    match argv.len() {
        3 => {
            starting_block = argv[1].parse::<u64>().expect("Couldn't parse starting block");
            ending_block = None;
            ipc_path = &argv[2];
        }
        4 => {
            starting_block = argv[1].parse::<u64>().expect("Couldn't parse starting block");
            ending_block = Some(argv[2].parse::<u64>().expect("Couldn't parse ending block"));
            ipc_path = &argv[3];
        }
        _ => {
            info!("Usage: evmextract STARTING_BLOCK [END_BLOCK] IPC_PATH");
            std::process::exit(1);
        }
    }

    let mut evm = EvmExtract::new(starting_block, &ipc_path);

    if ending_block.is_some() {
        evm.catchup(ending_block.unwrap())
    } else {
        info!("Continuous update loop");
        loop {
            evm.catchup_latest();
        }
    }

    info!("Done.");
}
