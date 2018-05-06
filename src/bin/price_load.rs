extern crate csv;
extern crate evmobserver;
#[macro_use]
extern crate log;
extern crate simple_logger;

use evmobserver::evmtrace::EvmTrace;
use evmobserver::prices::BestPrice;
use log::Level;
use std::env::args;
use std::time::Duration;

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let argv: Vec<String> = args().collect();

    if argv.len() < 3 {
        info!("Usage: price_load PRICES.CSV [COUNTS.CSV ...]");
        std::process::exit(1);
    }

    let mut prices = BestPrice::new();

    prices.load_csv(argv.get(1).unwrap());
    info!("Loaded {} prices", prices.len());

    let tolerance = Duration::from_secs(300);

    let count_file = argv.get(2).unwrap();
    let mut reader = csv::Reader::from_path(count_file).unwrap();

    for record in reader.deserialize() {
        if record.is_err() {
            warn!("Record error {:?}", record);
            continue;
        }

        let trace: EvmTrace = record.unwrap();
        let candle = prices.nearest_record(trace.ts, tolerance.as_secs()).unwrap();

        info!("block {}, txn {}, px {}, vol {}", trace.block_num, trace.txn_index, candle.low,
              candle.volume.unwrap_or(0.0));
    }

}
