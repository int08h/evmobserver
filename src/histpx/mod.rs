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
//! Exchanges and data px
//!

use serde_json;
use std::fmt::Display;
use std::fmt::Error;
use std::fmt::Formatter;

//! Implementations download from specific market data sources
pub trait PriceDl {
    fn download(&self, start_ts: u64, market: &Exchange) -> Vec<Candlestick>;
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Exchange {
    Kraken,
    Bitstamp,
    Poloniex,
    Gdax,
    Coinbase,
    Btce,
    Gemini,
    Binance,
    Bitfinex,
}

impl Display for Exchange {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let val = serde_json::to_value(self).unwrap();
        write!(f, "{}", val.as_str().unwrap())
    }
}

pub static EXCHANGES: [Exchange; 9] = [
    Exchange::Kraken,
    Exchange::Bitstamp,
    Exchange::Poloniex,
    Exchange::Gdax,
    Exchange::Coinbase,
    Exchange::Btce,
    Exchange::Gemini,
    Exchange::Binance,
    Exchange::Bitfinex,
];

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DataSource {
    Poloniex,
    Cryptowatch,
    Coinapi,
}

impl Display for DataSource {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        let val = serde_json::to_value(self).unwrap();
        write!(f, "{}", val.as_str().unwrap())
    }
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum FxMethod {
    EthUsd,
    EthUsdt,
    EthBtcUsd,
}
