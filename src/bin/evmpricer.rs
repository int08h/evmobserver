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

extern crate evmobserver;
extern crate json;
#[macro_use]
extern crate log;
extern crate reqwest;
extern crate simple_logger;

use json::JsonValue;
use std::u64;

static MARKETS: &[&str] = &[
    "kraken",
    "bitstamp",
    "poloniex",
    "gdax",
    "btce",
    "gemini",
    "binance",
    "bitfinex",
];

#[derive(Debug, Clone, Copy)]
struct Candlestick {
    market: &'static str,
    source: &'static str,
    end_ts: u64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: Option<f64>,
    vwap: Option<f64>,
}

fn cryptowatch_prices(start_ts: u64, market: &'static str) -> Vec<Candlestick> {
    let period = "300";
    let start_date_str = start_ts.to_string();

    let params = Vec::from([
        ("after", start_date_str.as_ref()),
        ("periods", period)
    ].as_ref());

    let url = format!("https://api.cryptowat.ch/markets/{}/ethusd/ohlc", market);
    let client = reqwest::Client::new();
    let mut response = client.get(&url)
        .query(&params)
        .send().unwrap();

    let text = response.text().expect("text conversion");
    let json = &json::parse(&text).unwrap()["result"][period];

    let mut results = Vec::with_capacity(json.len());

    // [ CloseTime, OpenPrice, HighPrice, LowPrice, ClosePrice, Volume ]
    for entry in json.members() {
        let end_ts = entry[0].as_u64().expect("end_ts");
        let open = entry[1].as_f64().expect("open");
        let high = entry[2].as_f64().expect("high");
        let low = entry[3].as_f64().expect("low");
        let close = entry[4].as_f64().expect("close");
        let volume = entry[5].as_f64().expect("volume");

        results.push(
            Candlestick {
                market: market,
                source: "cryptowat.ch",
                end_ts,
                open,
                high,
                low,
                close,
                volume: Some(volume),
                vwap: None,
            }
        );
    }

    results
}

fn poloniex_prices(start_ts: u64) -> Vec<Candlestick> {
    let start_date_str = start_ts.to_string();

    let params = Vec::from([
        ("command", "returnChartData"),
        ("currencyPair", "USDT_ETH"),
        ("end", "9999999999"),
        ("period", "1800"),
        ("start", start_date_str.as_ref())
    ].as_ref());

    let client = reqwest::Client::new();
    let mut response = client.get("https://poloniex.com/public")
        .query(&params)
        .send().unwrap();

    let text = response.text().expect("text conversion");
    let json = json::parse(&text).expect("Couldn't parse response");
    let mut results = Vec::with_capacity(json.len());

    for entry in json.members() {
        let end_ts = entry["date"].as_u64().expect("date");
        let open = entry["open"].as_f64().expect("open");
        let high = entry["high"].as_f64().expect("high");
        let low = entry["low"].as_f64().expect("low");
        let close = entry["close"].as_f64().expect("close");
        let volume = entry["volume"].as_f64().expect("volume");
        let vwap = entry["weightedAverage"].as_f64().expect("vwap");

        results.push(
            Candlestick {
                market: "poloniex",
                source: "poloniex",
                end_ts,
                open,
                high,
                low,
                close,
                volume: Some(volume),
                vwap: Some(vwap),
            }
        );
    }

    results
}

fn main() {
    use log::Level;
    use std::env::args;

    simple_logger::init_with_level(Level::Info).unwrap();

    let argv: Vec<String> = args().collect();

//    let prices = poloniex_prices(1438922534);

    for market in MARKETS {
        println!("Getting cryptowat.ch {}", market);
//        let prices = cryptowatch_prices(1438922534, market);
        let prices = cryptowatch_prices(1524199386, market);
        println!("  ... {} prices", prices.len());

        for px in &prices {
            info!("{:?}", px);
        }
    }

}
