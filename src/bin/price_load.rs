extern crate evmobserver;
#[macro_use]
extern crate log;
extern crate simple_logger;

use evmobserver::prices::BestPrice;
use log::Level;
use std::env::args;

fn main() {
    simple_logger::init_with_level(Level::Info).unwrap();

    let argv: Vec<String> = args().collect();

    if argv.len() != 2 {
        info!("Usage: price_load PRICES.CSV");
        std::process::exit(1);
    }

    let mut best_px = BestPrice::new();

    best_px.load_csv(argv.get(1).unwrap());

    info!("Records {}", best_px.len());

    let ts = 1519956698;
    let px = best_px.nearest_price(ts);
    info!("{} -> {:?}", ts, px);
}
