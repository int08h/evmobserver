extern crate csv;
extern crate evmobserver;
#[macro_use]
extern crate log;
extern crate simple_logger;

use csv::ByteRecord;
use csv::StringRecord;
use evmobserver::csvfiles::PriceReader;
use evmobserver::evminst;
use evmobserver::evmtrace;
use evmobserver::evmtrace::EvmTrace;
use evmobserver::prices;
use evmobserver::prices::Candlestick;
use log::Level;
use std::env::args;
use std::time::Duration;

fn visitor(candle: &Candlestick, trace: &ByteRecord) {
    let ts = evmtrace::get_field_u64(trace, evmtrace::TS_IDX);
    let block_num = evmtrace::get_field_u32(trace, evmtrace::BLOCK_NUM_IDX);
    let txn_index = evmtrace::get_field_u16(trace, evmtrace::TXN_INDEX_IDX);
    let gas_px = evmtrace::get_field_u64(trace, evmtrace::GAS_PX_IDX);
    let mid_px = prices::BestPrice::mid_price(candle);

    let gas_fiat = (gas_px as f64 / 1_000_000_000.0) * mid_px / 1_000_000_000.0;

    info!("ts {}, block {}, txn {}, gas {:.3} * mid ${:.3} = ${:.9}",
          ts, block_num, txn_index, gas_px, mid_px, gas_fiat);

    for (i, inst) in evminst::VALUES.iter().enumerate() {
        let (count, gas) = evmtrace::get_inst_fields(trace, i);
        if count == 0 {
           continue;
        };
//        info!("  {} {}, gas {}", evminst::as_str(inst), count, gas);
    }
}

struct Pricer {}

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let mut argv: Vec<String> = args().collect();

    if argv.len() < 3 {
        info!("Usage: price_load PRICES.CSV [COUNTS.CSV ...]");
        std::process::exit(1);
    }

    let mut prices = PriceReader::new(argv.get(1).unwrap());

    info!("Loaded {} prices", prices.len());

    prices.process(argv.split_off(2), visitor);
}
