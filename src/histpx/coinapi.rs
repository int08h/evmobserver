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

use chrono::{DateTime, TimeZone, Utc};
use histpx::{DataSource, FxMethod, Exchange, PriceDl};
use prices::Candlestick;

use self::reqwest::header::Headers;

struct CoinApi {}

impl CoinApi {
    pub fn new() -> Self {
        CoinApi {}
    }
}

impl PriceDl for CoinApi {
    fn download(&self, start_ts: u64, market: &Exchange) -> Vec<Candlestick> {
        let url = format!(
            "https://rest.coinapi.io/v1/ohlcv/{}_SPOT_ETH_USD/history",
            market.to_string().to_ascii_uppercase()
        );

        let start_date = Utc.timestamp(start_ts as i64, 0).to_rfc3339();

        let mut headers = Headers::new();
        headers.set_raw("X-CoinAPI-Key", "<your key here>");

        let params = Vec::from(
            [
                ("period_id", "5MIN"),
                ("time_start", &start_date),
                ("include_empty_items", "false"),
                ("limit", "1000"),
            ].as_ref(),
        );

        let client = reqwest::Client::new();
        let mut response = client
            .get(&url)
            .query(&params)
            .headers(headers)
            .send()
            .unwrap();

        let text = response.text().expect("text conversion");
        let json = json::parse(&text).expect("Couldn't parse response");
        let mut results = Vec::with_capacity(json.len());

        for entry in json.members() {
            let tp_end = entry["time_period_end"].as_str().expect("time_period_end");
            let open = entry["price_open"].as_f64().expect("price_open");
            let high = entry["price_high"].as_f64().expect("price_high");
            let low = entry["price_low"].as_f64().expect("price_low");
            let close = entry["price_close"].as_f64().expect("price_close");
            let volume = entry["volume_traded"].as_f64().expect("volume_traded");

            let end_ts = DateTime::parse_from_rfc3339(tp_end)
                .expect("parsing time_period_end")
                .timestamp() as u64;

            results.push(Candlestick {
                market: *market,
                source: DataSource::Coinapi,
                fx_method: FxMethod::EthUsd,
                end_ts,
                open,
                high,
                low,
                close,
                volume: Some(volume),
            });
        }

        // The rate-limit values returned from the API call
        let mut rl_limit = -1i64;
        let mut rl_remain = -1i64;
        let mut rl_cost = -1i64;
        let mut rl_reset = "unknown".to_string();

        for item in response.headers().iter() {
            match item.name().to_ascii_lowercase().as_str() {
                "x-ratelimit-limit" => rl_limit = item.value_string().parse().expect("limit"),
                "x-ratelimit-remaining" => rl_remain = item.value_string().parse().expect("remaining"),
                "x-ratelimit-request-cost" => rl_cost = item.value_string().parse().expect("cost"),
                "x-ratelimit-reset" => rl_reset = item.value_string(),
                _ => (),
            }
        }
        println!(
            "CoinAPI limit {}, remaining {}, request cost {}, reset at {}",
            rl_limit, rl_remain, rl_cost, rl_reset
        );

        results
    }
}
