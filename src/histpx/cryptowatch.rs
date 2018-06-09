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

extern crate chrono;
extern crate csv;
extern crate json;
extern crate log;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate simple_logger;

use std::u64;

use histpx::{Exchange, FxMethod, DataSource, PriceDl};
use prices::Candlestick;

struct CryptoWatch {}

impl CryptoWatch {
    pub fn new() -> Self {
        CryptoWatch {}
    }
}

impl PriceDl for CryptoWatch {
    fn download(&self, start_ts: u64, market: &Exchange) -> Vec<Candlestick> {
        let period = "300";
        let start_date_str = start_ts.to_string();

        let params = Vec::from([("after", start_date_str.as_ref()), ("periods", period)].as_ref());

        let url = format!("https://api.cryptowat.ch/markets/{}/ethusd/ohlc", market);
        let client = reqwest::Client::new();
        let mut response = client.get(&url).query(&params).send().unwrap();

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

            results.push(Candlestick {
                market: *market,
                source: DataSource::Cryptowatch,
                fx_method: FxMethod::EthUsd,
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
