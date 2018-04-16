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
//! Provides Geth IPC calls
//!

extern crate bytesize;
extern crate json;
extern crate log;
extern crate simple_logger;
extern crate stopwatch;

use json::JsonValue;
use log::Level::Debug;
use std::io::{Read, Write};
use std::io::ErrorKind::{TimedOut, WouldBlock};
use std::os::unix::net::UnixStream;
use std::str;
use std::time::Duration;
use std::u64;
use stopwatch::Stopwatch;

///
/// Geth IPC interactions
///
pub struct GethRpc {
    /// Unix Domain Socket for Geth IPC
    stream: UnixStream,

    /// Accumulate partial JSON reads from IPC socket for parsing
    results: Vec<u8>,

    /// For timing internal operations
    sw: Stopwatch
}

impl GethRpc {
    /// Open the IPC socket at `ipc_path`
    pub fn new(ipc_path: &str) -> Self {
        let evm = GethRpc {
            stream: UnixStream::connect(ipc_path).unwrap(),
            results: Vec::with_capacity((16 * bytesize::MIB) as usize),
            sw: Stopwatch::new()
        };

        evm.stream.set_read_timeout(Some(Duration::new(10, 0))).expect("Couldn't set read timeout");

        return evm;
    }

    /// Call Geth `eth.syncing`
    pub fn syncing(&mut self) -> json::Result<JsonValue> {
        let rpc = r#"{"jsonrpc":"2.0","method":"eth_syncing","params":[],"id":1}"#;

        self.stream.write_all(rpc.as_bytes()).expect("write error");

        let (_, parsed_json) = self.consume_response("eth.syncing");

        return parsed_json;
    }

    /// Call `eth.getTransactionByBlockNumberAndIndex`
    pub fn txn_by_block_idx(&mut self, block_num: u64, txn_index: u64) -> json::Result<JsonValue> {
        let rpc = format!(
            "{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getTransactionByBlockNumberAndIndex\",\
            \"params\":[\"{:#x}\",\"{:#x}\"],\"id\":1}}", block_num, txn_index
        );

        self.stream.write_all(rpc.as_bytes()).expect("write error");

        let (_, parsed_json) = self.consume_response("txn_by_block_idx");

        return parsed_json;
    }

    /// Call Geth `debug.traceBlockByNumber`
    pub fn trace_block(&mut self, block_num: u64) -> (u64, json::Result<JsonValue>) {
        let rpc = format!(
            "{{\"jsonrpc\":\"2.0\",\"method\":\"debug_traceBlockByNumber\",\
            \"params\":[\"{:#x}\",{{\"disableStorage\":true,\"disableStack\":true,\"disableMemory\":true}}],\
            \"id\":1}}", block_num
        );

        self.stream.write_all(rpc.as_bytes()).expect("write error");

        return self.consume_response("trace_block");
    }

    fn consume_response(&mut self, note: &str) -> (u64, json::Result<JsonValue>) {
        self.results.clear();
        let mut buf = [0u8; 1024 * 1024];
        let mut total_read = 0u64;

        loop {
            self.sw.start();

            match self.stream.read(&mut buf) {
                Ok(size) => {
                    total_read += size as u64;

                    if log_enabled!(Debug) {
                        debug!("{}: loop read {} in {} ms, total {}",
                               note, size, self.sw.elapsed_ms(), total_read);
                    }

                    let chunk = &buf[..size];
                    self.results.extend(chunk);
                    if chunk[size - 1] == b'\n' {
                        break
                    }
                },

                Err(ref e) if e.kind() == TimedOut || e.kind() == WouldBlock =>
                    warn!("Read timeout in {}, continuing", note),

                Err(e) => {
                    panic!("Failed read in {}: {:?}", note, e);
                }
            }
        }

        // we don't need no stinking validation
        let payload = unsafe { str::from_utf8_unchecked(&self.results) };

        (total_read, json::parse(payload))
    }
}
