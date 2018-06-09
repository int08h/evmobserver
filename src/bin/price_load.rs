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

extern crate csv;
extern crate evmobserver;
#[macro_use]
extern crate log;
extern crate simple_logger;

use csv::ByteRecord;
use evmobserver::csvfiles::PriceReader;
use evmobserver::evminst;
use evmobserver::evmtrace;
use evmobserver::prices::Candlestick;
use log::Level;
use std::env::args;
use std::fmt::Write;

fn visitor(candle: &Candlestick, trace: &ByteRecord) {
    let mid_px = candle.mid_price();
    let ts = evmtrace::get_field_u64(trace, evmtrace::TS_IDX);
    let block_num = evmtrace::get_field_u32(trace, evmtrace::BLOCK_NUM_IDX);
    let txn_index = evmtrace::get_field_u16(trace, evmtrace::TXN_INDEX_IDX);
    let gas_px = evmtrace::get_field_u64(trace, evmtrace::GAS_PX_IDX);

    let gas_fiat_px = (gas_px as f64 / 1_000_000_000.0) * mid_px / 1_000_000_000.0;
    let mut output = String::with_capacity(2048);
    let mut gas_block_total = 0f64;

    for (i, inst) in evminst::VALUES.iter().enumerate() {
        let (count, gas) = evmtrace::get_inst_fields(trace, i);
        if count == 0 || gas == 0 {
            continue;
        };

        let gas_px_used = gas_fiat_px * gas as f64;
        gas_block_total += gas_px_used;

        write!(output, "{}:{} {} = ${:9}\n", inst, count, gas, gas_px_used).unwrap();
    }

    info!(
        "ts {}, block {}, txn {}, gas {:.3} * mid ${:.3} = ${:.9} TOTAL={:9};\n{}",
        ts, block_num, txn_index, gas_px, mid_px, gas_fiat_px, gas_block_total, output
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
