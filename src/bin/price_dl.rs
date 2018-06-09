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
//! Obtain historical ethereum prices and persist them
//!

extern crate chrono;
extern crate csv;
extern crate evmobserver;
extern crate json;
extern crate log;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate simple_logger;

use csv::Writer;
use evmobserver::prices::Candlestick;
use std::io;
use std::u64;

use evmobserver::histpx::PriceDl;
use evmobserver::histpx::Exchange;
use evmobserver::histpx::poloniex::Poloniex;

fn write_prices<W: io::Write>(writer: &mut Writer<W>, prices: &Vec<Candlestick>) {
    for px in prices {
        writer.serialize(px).unwrap();
    }
}

fn main() {
    use log::Level;
    use std::env::args;

    simple_logger::init_with_level(Level::Info).unwrap();

    let argv: Vec<String> = args().collect();
    if argv.len() != 2 {
        println!("usage: price_dl START_EPOCH");
        std::process::exit(1);
    }

    let start_ts: u64 = argv.get(1).unwrap().parse().unwrap();

    let mut writer = csv::Writer::from_path("prices.csv").unwrap();

    let poloniex = Poloniex::new();
    let prices = poloniex.download(start_ts, &Exchange::Poloniex);

    write_prices(&mut writer, &prices);

//    for market in EXCHANGES.iter() {
//    for market in vec![Exchange::Gdax, Exchange::Gemini].iter() {
//        let prices = cryptowatch_prices(start_ts, market);
//        let prices = coinapi_prices(start_ts, market);
//        println!(
//            "{}:{} from {} has {} prices",
//            DataSource::Coinapi,
//            market,
//            Utc.timestamp(start_ts as i64, 0),
//            prices.len()
//        );
//
//        write_prices(&mut writer, &prices);
//    }

    writer.flush().unwrap();
}
