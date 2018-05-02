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
//! Ethereum historical prices
//!

extern crate csv;
extern crate log;
extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate simple_logger;

use serde::Serialize;
use sources::{DataSource, Exchange, FxMethod};
use std::collections::Bound::{Excluded, Included, Unbounded};
use std::collections::BTreeMap;
use std::u64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Candlestick {
    pub market: Exchange,
    pub source: DataSource,
    pub fx_method: FxMethod,
    pub end_ts: u64,
    pub open: f64,
    pub high: f64,
    pub low: f64,
    pub close: f64,
    pub volume: Option<f64>,
}

pub struct BestPrice {
    prices: BTreeMap<u64, Candlestick>,
}

impl BestPrice {
    pub fn new() -> Self {
        BestPrice {
            prices: BTreeMap::new()
        }
    }

    pub fn load_csv(&mut self, file_name: &str) {
        let mut reader = csv::Reader::from_path(file_name).unwrap();
        let mut count = 0u64;

        for record in reader.deserialize() {
            count += 1;
            let candle: Candlestick = record.unwrap();

            if self.prices.get(&candle.end_ts).is_none() {
               self.prices.insert(candle.end_ts, candle);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.prices.len()
    }

    pub fn nearest_price(&self, ts: u64) -> Option<&Candlestick> {
        match self.prices.range((Included(ts), Unbounded)).next() {
            Some((_, v)) => Some(v),
            None => None
        }
    }
}


