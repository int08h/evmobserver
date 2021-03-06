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

use csv;
use csv::ByteRecord;
use evminst;
use gethrpc::{BlockInfo, TxnInfo};
use instcount::InstCount;
use prices::{BestPrice, Candlestick};
use std::fs::File;
use std::io;
use std::io::BufWriter;
use std::io::prelude::*;
use std::result::Result::Ok;
use std::str;

///
/// CSV output of counts
///
pub struct TraceOutFile {
    out_writer: BufWriter<File>,

    pub last_block: u64,
    pub last_time_stamp: u64,
    pub total_written: u64,
    pub total_txns: u64,
    pub total_inst: u64,
    pub total_gas: u64,
}

pub struct PriceReader {
    pub prices: BestPrice,
}

impl PriceReader {
    pub fn new(prices_file: &str) -> Self {
        let mut prices = BestPrice::new();
        prices.load_csv(prices_file);

        PriceReader { prices }
    }

    // visit_fn is intentionally a function pointer to prevent painfully long recompilation
    // whenever visit_fn is changed; cargo workspaces might help, if I knew how to use them :/
    pub fn process(&self, count_files: Vec<String>, visit_fn: fn(&Candlestick, &ByteRecord) -> ()) {
        for count_file in count_files {
            let mut reader = csv::Reader::from_path(count_file.clone())
                .expect(&format!("Opening {}", count_file));

            for record in reader.byte_records() {
                if record.is_err() {
                    warn!("file {} record {:?}", count_file, record);
                    continue;
                }

                let trace = record.unwrap();
                let tmp = trace.get(0).unwrap();
                let ts: u64 = unsafe { str::from_utf8_unchecked(tmp).parse().unwrap() };
                let candle = self.prices.nearest_record(ts).unwrap();

                visit_fn(candle, &trace);
            }
        }
    }

    pub fn len(&self) -> usize {
        self.prices.len()
    }
}

impl TraceOutFile {
    const LINE_LIMIT: u64 = 10_000;

    pub fn new(starting_block: u64) -> Self {
        TraceOutFile {
            out_writer: Self::create_outfile(starting_block),
            last_block: 0,
            last_time_stamp: 0,
            total_written: 0,
            total_txns: 0,
            total_inst: 0,
            total_gas: 0,
        }
    }

    pub fn write_count(
        &mut self,
        txn_count: &InstCount,
        txn_info: &TxnInfo,
        block_info: &BlockInfo,
    ) -> io::Result<()> {
        let mut written: usize = 0;

        written += self.out_writer
            .write(block_info.time_stamp.to_string().as_ref())?;
        written += self.out_writer.write(b",")?;
        written += self.out_writer
            .write(block_info.block_num.to_string().as_ref())?;
        written += self.out_writer.write(b",")?;
        written += self.out_writer
            .write(txn_info.block_index.to_string().as_ref())?;
        written += self.out_writer.write(b",")?;
        written += self.out_writer.write(txn_info.from.as_ref())?;
        written += self.out_writer.write(b",")?;
        written += self.out_writer.write(txn_info.to.as_ref())?;
        written += self.out_writer.write(b",")?;
        written += self.out_writer
            .write(txn_info.gas_price.to_string().as_ref())?;
        written += self.out_writer.write(b",")?;

        for i in 0..evminst::VALUES.len() {
            let op = evminst::VALUES[i];

            match txn_count.get_count(op) {
                0 => written += self.out_writer.write(b"0,0,")?,
                count => {
                    let gas = txn_count.get_gas(op);

                    self.total_inst += count;
                    self.total_gas += gas;

                    written += self.out_writer.write(count.to_string().as_ref())?;
                    written += self.out_writer.write(b",")?;
                    written += self.out_writer.write(gas.to_string().as_ref())?;
                    written += self.out_writer.write(b",")?
                }
            };
        }

        written += self.out_writer.write(b"\n")?;
        self.out_writer.flush()?;

        self.total_txns += 1;
        self.last_block = block_info.block_num;
        self.last_time_stamp = block_info.time_stamp;
        self.total_written += written as u64;

        if self.total_txns % Self::LINE_LIMIT == 0 {
            self.rotate_file();
        }

        Ok(())
    }

    fn rotate_file(&mut self) {
        self.out_writer.flush().unwrap();
        self.out_writer = Self::create_outfile(self.last_block);
    }

    fn create_outfile(block_num: u64) -> BufWriter<File> {
        let file_name = format!("counts.{}.csv", block_num);
        let outfile = File::create(&file_name).expect("Failed to create file");
        info!("Writing to {}", &file_name);

        let mut writer = BufWriter::new(outfile);

        writer
            .write("ts,block_num,txn_index,addr_from,addr_to,gas_px,".as_ref())
            .unwrap();

        for i in 0..evminst::VALUES.len() {
            let op = evminst::as_str(&evminst::VALUES[i]).as_ref();
            writer.write(op).unwrap();
            writer.write("_count,".as_ref()).unwrap();
            writer.write(op).unwrap();
            writer.write("_gas,".as_ref()).unwrap();
        }
        writer.write(b"\n").unwrap();
        writer.flush().unwrap();

        writer
    }
}
