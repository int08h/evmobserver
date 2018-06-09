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
extern crate json;
extern crate log;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate simple_logger;

use std::u64;
use prices::Candlestick;
use histpx::{Exchange, PriceDl, DataSource, FxMethod};

pub struct Poloniex {}

impl PriceDl for Poloniex {
    fn download(&self, start_ts: u64, _: &Exchange) -> Vec<Candlestick> {
        Poloniex::download(start_ts)
    }
}

impl Poloniex {
    pub fn new() -> Self {
        Poloniex {}
    }

    pub fn download(start_ts: u64) -> Vec<Candlestick> {
        let start_date_str = start_ts.to_string();

        let params = Vec::from(
            [
                ("command", "returnChartData"),
                ("currencyPair", "USDT_ETH"),
                ("end", "9999999999"),
                ("period", "300"),
                ("start", start_date_str.as_ref()),
            ].as_ref(),
        );

        let client = reqwest::Client::new();
        let mut response = client
            .get("https://poloniex.com/public")
            .query(&params)
            .send()
            .unwrap();

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

            results.push(Candlestick {
                market: Exchange::Poloniex,
                source: DataSource::Poloniex,
                fx_method: FxMethod::EthUsdt,
                end_ts,
                open,
                high,
                low,
                close,
                volume: Some(volume),
            });
        }

        results
    }
}
