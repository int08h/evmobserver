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

use chrono::{DateTime, TimeZone, Utc};
use csv::Writer;
use evmobserver::prices::Candlestick;
use evmobserver::sources::{DataSource, Exchange, EXCHANGES, FxMethod};
use reqwest::header::Headers;
use std::io;
use std::u64;

#[allow(dead_code)]
fn coinapi_prices(start_ts: u64, market: &Exchange) -> Vec<Candlestick> {
    let url = format!(
        "https://rest.coinapi.io/v1/ohlcv/{}_SPOT_ETH_USD/history",
        market.to_string().to_ascii_uppercase()
    );

    let start_date = Utc.timestamp(start_ts as i64, 0).to_rfc3339();

    let mut headers = Headers::new();
    headers.set_raw("X-CoinAPI-Key", "");

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

#[allow(dead_code)]
fn cryptowatch_prices(start_ts: u64, market: &Exchange) -> Vec<Candlestick> {
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

#[allow(dead_code)]
fn poloniex_prices(start_ts: u64) -> Vec<Candlestick> {
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

    //    let prices = poloniex_prices(start_ts);
    //    write_prices(&mut writer, &prices);

      for market in EXCHANGES.iter() {
//    for market in vec![Exchange::Gdax, Exchange::Gemini].iter() {
        // let prices = cryptowatch_prices(start_ts, market);
        let prices = coinapi_prices(start_ts, market);
        println!(
            "{}:{} from {} has {} prices",
            DataSource::Coinapi,
            market,
            Utc.timestamp(start_ts as i64, 0),
            prices.len()
        );

        write_prices(&mut writer, &prices);
    }

    writer.flush().unwrap();
}
