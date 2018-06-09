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

#[macro_use]
extern crate log;
extern crate csv;
extern crate evmobserver;
extern crate simple_logger;

use std::env::args;
use std::fmt::Write;

use log::Level;
use csv::ByteRecord;
use evmobserver::csvfiles::PriceReader;
use evmobserver::evminst;
use evmobserver::evmtrace;
use evmobserver::prices::Candlestick;

const DIVISOR: f64 = 1e9;

fn visitor(candle: &Candlestick, trace: &ByteRecord) {
    let ts = evmtrace::get_field_u64(trace, evmtrace::TS_IDX);
    let block_num = evmtrace::get_field_u32(trace, evmtrace::BLOCK_NUM_IDX);
    let txn_index = evmtrace::get_field_u16(trace, evmtrace::TXN_INDEX_IDX);
    let addr_from = evmtrace::get_field_str(trace, evmtrace::ADDR_FROM_IDX);
    let gas_px_gwei = evmtrace::get_field_u64(trace, evmtrace::GAS_PX_IDX) as f64 / DIVISOR;

    let mid_px_fiat = candle.mid_price();
    let gas_px_eth = gas_px_gwei / DIVISOR;
    let gas_px_fiat = gas_px_eth * mid_px_fiat;

    let mut block_total_gas = 0u64;
    let mut block_total_px_eth = 0f64;
    let mut block_total_px_fiat = 0f64;

    let mut output = String::with_capacity(2048);

    for (i, inst) in evminst::VALUES.iter().enumerate() {
        let (count, gas) = evmtrace::get_inst_fields(trace, i);
        if count == 0 || gas == 0 {
            continue;
        };

        block_total_gas += gas;

        let gas_px_used_eth = gas_px_eth * gas as f64;
        let gas_px_used_fiat = gas_px_fiat * gas as f64;

        block_total_px_eth += gas_px_used_eth;
        block_total_px_fiat += gas_px_used_fiat;

        write!(output, "{}:{} {} = ${:.9}\n", inst, count, gas, gas_px_used_fiat).unwrap();
    }

    info!(
        "ts {}, block {}, from {}, txn {}, gas_total_count {}, gas_px_eth {:.12} \
        ({:.3} gwei) * mid ${:.3} = ${:.9} TOTAL=${:.6} ({:.9} eth)\n{}",
        ts, block_num, addr_from, txn_index, block_total_gas, gas_px_eth, gas_px_gwei,
        mid_px_fiat, gas_px_fiat, block_total_px_fiat, block_total_px_eth, output
    );
}

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let mut argv: Vec<String> = args().collect();

    if argv.len() < 3 {
        info!("Usage: price_load PRICES.CSV [COUNTS.CSV ...]");
        std::process::exit(1);
    }

    let prices = PriceReader::new(argv.get(1).unwrap());

    info!("Loaded {} prices", prices.len());

    prices.process(argv.split_off(2), visitor);
}
