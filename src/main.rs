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

///
/// Very early work to obtain EVM traces from Geth IPC. Work in progress, etc.
///

#[macro_use]
extern crate log;
#[macro_use]
extern crate json;
extern crate bytesize;
extern crate simple_logger;

use bytesize::ByteSize;

use json::JsonValue;

use std::os::unix::net::UnixStream;
use std::io::{Read, Write};
use std::{thread, time, str};
use std::time::Duration;
use std::io::ErrorKind;
use std::u64;

struct EvmExtract {}

impl EvmExtract {

    /// Latest synchronized block number from Geth `eth.syncing` call
    fn parse_latest_block(data: &JsonValue) -> u64 {
        let maybe_curr_block = &data["result"]["currentBlock"].as_str();

        if maybe_curr_block.is_some() {
            let hex_str = &maybe_curr_block.unwrap();
            if hex_str.len() > 2 {
                // skip leading 0x
                return u64::from_str_radix(&hex_str[2..], 16).expect("illegal hex value");
            }
        }

        return 0;
    }
}

/// Geth IPC interactions
struct Rpc {
    /// Unix Domain Socket for Geth IPC
    stream: UnixStream,

    /// Accumulate partial JSON reads from IPC socket for parsing
    results: Vec<u8>
}

impl Rpc {
    /// Open the IPC socket at `ipc_path`
    fn new(ipc_path: &str) -> Self {
        let evm = Rpc {
            stream: UnixStream::connect(ipc_path).unwrap(),
            results: Vec::with_capacity((16 * bytesize::MIB) as usize)
        };

        evm.stream.set_read_timeout(Some(Duration::new(10, 0))).expect("Couldn't set read timeout");

        return evm;
    }

    /// Call Geth `eth.syncing`
    fn syncing(&mut self) -> json::Result<JsonValue> {
        let rpc = r#"{"jsonrpc":"2.0","method":"eth_syncing","params":[],"id":1}"#;

        self.stream.write_all(rpc.as_bytes()).expect("write error");

        let mut buf = [0u8; 4096];
        match self.stream.read(&mut buf) {
            Ok(size) => {
                let payload = str::from_utf8(&buf[..size]).unwrap();
                json::parse(payload)
            },
            Err(ref e) if e.kind() == ErrorKind::TimedOut => Err(json::Error::WrongType("timeout".into())),
            Err(e) => panic!("ERROR: failed to read {:?}", e),
        }
    }

    /// Call Geth `debug.traceBlockByNumber`
    fn trace_block(&mut self, block_num: u64) -> (u64, json::Result<JsonValue>) {
        let rpc = format!(
            "{{\"jsonrpc\":\"2.0\",\"method\":\"debug_traceBlockByNumber\",\
            \"params\":[\"{:#x}\",{{\"disableStorage\":true,\"disableStack\":true,\"disableMemory\":true}}],\
            \"id\":1}}", block_num
        );

        self.stream.write_all(rpc.as_bytes()).expect("write error");

        self.results.clear();
        let mut buf = [0u8; 1024 * 1024];
        let mut total = 0u64;

        loop {
            match self.stream.read(&mut buf) {
                Ok(size) => {
                    total += size as u64;
                    debug!("Block {} loop read {} total {}", block_num, size, total);

                    let slice = &buf[..size];
                    self.results.extend(slice);
                    if slice[size - 1] == b'\n' {
                        break
                    }
                },
                _ => break
            }
        }

        // we don't need no stinking validation
        let payload = unsafe { str::from_utf8_unchecked(&self.results) };

        (total, json::parse(payload))
    }
}

fn main() {
    use log::Level;

    simple_logger::init_with_level(Level::Info).unwrap();

    let evm = EvmExtract::new();
    let mut rpc = Rpc::new("/home/stuart/.ethereum/geth.ipc");
    let mut current_block = 1_000_000;

    loop {
        let sync_json = rpc.syncing().expect("Couldn't get json");
        let latest_block = EvmExtract::parse_latest_block(&sync_json);

        for block_num in current_block..latest_block {
            let (total, trace) = rpc.trace_block(block_num);
            let result = &trace.unwrap()["result"];
            let num_traces = result.len();

            info!("Block {} read {} with {} traces", block_num, ByteSize::b(total), num_traces);
            if num_traces > 0 {
                info!("Trace: {}", result[0])
            }
        }
        current_block = latest_block;

        let millis = time::Duration::from_millis(1000);
        thread::sleep(millis);
    }
}
