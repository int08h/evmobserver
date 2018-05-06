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

use bytesize;
use json;
use json::JsonValue;
use std::io::{Read, Write};
use std::io::ErrorKind::{TimedOut, WouldBlock};
use std::os::unix::net::UnixStream;
use std::str;
use std::time::Duration;
use std::u64;
use util::hex_to_u64;

///
/// Geth IPC interactions
///
pub struct GethRpc {
    /// Unix Domain Socket for Geth IPC
    stream: UnixStream,

    /// Accumulate partial JSON reads from IPC socket for parsing
    results: Vec<u8>,
}

/// Interesting Block information
#[derive(Debug)]
pub struct BlockInfo {
    pub block_num: u64,
    pub time_stamp: u64,
    pub gas_limit: u64,
}

/// Interesting transaction info
#[derive(Debug)]
pub struct TxnInfo {
    pub block_num: u64,
    pub block_index: u32,
    pub gas_price: u64,
    pub from: String,
    pub to: String,
}

impl GethRpc {
    /// Open the IPC socket at `ipc_path`
    pub fn new(ipc_path: &str) -> Self {
        let evm = GethRpc {
            stream: UnixStream::connect(ipc_path).expect("could not connect to socket"),
            results: Vec::with_capacity((16 * bytesize::MIB) as usize),
        };

        evm.stream.set_read_timeout(Some(Duration::new(10, 0))).expect("Couldn't set read timeout");

        return evm;
    }

    /// Latest synchronized block number from Geth `eth.syncing` call
    pub fn get_latest_block(&mut self) -> Option<u64> {
        let data = match self.block_number() {
            Ok(v) => v,
            Err(e) => {
                println!("Error block_number(): {:?}", e);
                return None;
            }
        };

        match data["result"].as_str() {
            Some(v) => hex_to_u64(v),
            _ => None
        }
    }

    /// Obtain `BlockInfo` for the provided block number
    pub fn block_info(&mut self, block_num: u64) -> BlockInfo {
        let block_info = &self.get_block(block_num).unwrap_or(JsonValue::Null)["result"];

        let ts = block_info["timestamp"].as_str().unwrap_or("0x0");
        let gas_limit = block_info["gas_limit"].as_str().unwrap_or("0x0");

        BlockInfo {
            block_num,
            time_stamp: hex_to_u64(ts).unwrap_or(0),
            gas_limit: hex_to_u64(gas_limit).unwrap_or(0),
        }
    }

    /// Obtain transaction information for transaction in the given block at the given
    /// transaction index.
    pub fn txn_info(&mut self, block_num: u64, block_index: u32) -> TxnInfo {
        let mut txn_info = self.txn_by_block_idx(block_num, block_index)
            .unwrap_or(JsonValue::Null);

        let from = txn_info["result"]["from"].take_string().unwrap_or_else(|| String::new());
        let to = txn_info["result"]["to"].take_string().unwrap_or_else(|| String::new());
        let gas_price = {
            let px = txn_info["result"]["gasPrice"].as_str().unwrap_or("0x0");
            hex_to_u64(px).unwrap_or(0)
        };

        TxnInfo {
            block_num,
            block_index,
            gas_price,
            from,
            to,
        }
    }

    /// Call Geth `debug.traceBlockByNumber`
    pub fn trace_block(&mut self, block_num: u64) -> json::Result<JsonValue> {
        let rpc = format!(
            "{{\"jsonrpc\":\"2.0\",\"method\":\"debug_traceBlockByNumber\",\
            \"params\":[\"{:#x}\",{{\"disableStorage\":true,\"disableStack\":true,\"disableMemory\":true}}],\
            \"id\":1}}", block_num
        );

        self.stream.write_all(rpc.as_bytes()).expect("write error");

        let (_, parsed_json) = self.consume_response("trace_block");

        return parsed_json;
    }

    /// Call Geth `eth.blockNumber`
    fn block_number(&mut self) -> json::Result<JsonValue> {
        let rpc = r#"{"jsonrpc":"2.0","method":"eth_blockNumber","params":[],"id":1}"#;

        self.stream.write_all(rpc.as_bytes()).expect("write error");

        let (_, parsed_json) = self.consume_response("eth.blockNumber");

        return parsed_json;
    }

    /// Call `eth.getTransactionByBlockNumberAndIndex`
    fn txn_by_block_idx(&mut self, block_num: u64, txn_index: u32) -> json::Result<JsonValue> {
        let rpc = format!(
            "{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getTransactionByBlockNumberAndIndex\",\
            \"params\":[\"{:#x}\",\"{:#x}\"],\"id\":1}}", block_num, txn_index
        );

        self.stream.write_all(rpc.as_bytes()).expect("write error");

        let (_, parsed_json) = self.consume_response("txn_by_block_idx");

        return parsed_json;
    }

    /// Call Geth `eth.getBlockByNumber`
    fn get_block(&mut self, block_num: u64) -> json::Result<JsonValue> {
        let rpc = format!(
            "{{\"jsonrpc\":\"2.0\",\"method\":\"eth_getBlockByNumber\",\
            \"params\":[\"{:#x}\",false],\"id\":1}}", block_num
        );

        self.stream.write_all(rpc.as_bytes()).expect("write error");

        let (_, parsed_json) = self.consume_response("get_block");

        return parsed_json;
    }

    fn consume_response(&mut self, note: &str) -> (u64, json::Result<JsonValue>) {
        self.results.clear();
        let mut buf = [0u8; 1024 * 1024];
        let mut total_read = 0u64;

        loop {
            match self.stream.read(&mut buf) {
                Ok(size) => {
                    total_read += size as u64;

                    debug!("{}: loop read {}, total {}", note, size, total_read);

                    let chunk = &buf[..size];
                    self.results.extend(chunk);
                    if chunk[size - 1] == b'\n' {
                        break;
                    }
                },

                Err(ref e) if e.kind() == TimedOut || e.kind() == WouldBlock =>
                    warn!("Read timeout in {}, continuing", note),

                Err(e) =>
                    panic!("Failed read in {}: {:?}", note, e)
            }
        }

        // we don't need no stinking validation; assume geth produces valid utf8
        let payload = unsafe { str::from_utf8_unchecked(&self.results) };

        (total_read, json::parse(payload))
    }
}
