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

use evminst::EvmInst;

///
/// Organize counts of EVM instructions and the gas consumed by them
///
pub struct InstCount {
    evm_counts: [u64; 256],
    gas_counts: [u64; 256],
    gas_total: u64,
}

impl InstCount {
    pub fn new() -> Self {
        InstCount {
            evm_counts: [0; 256],
            gas_counts: [0; 256],
            gas_total: 0,
        }
    }

    pub fn inc_count(&mut self, evm_inst: EvmInst) {
        self.evm_counts[evm_inst as usize] += 1;
    }

    pub fn add_gas(&mut self, evm_inst: EvmInst, gas_used: u64) {
        self.gas_counts[evm_inst as usize] += gas_used;
        self.gas_total += gas_used;
    }

    pub fn get_count(&self, evm_inst: EvmInst) -> u64 {
        self.evm_counts[evm_inst as usize]
    }

    pub fn get_gas(&self, evm_inst: EvmInst) -> u64 {
        self.gas_counts[evm_inst as usize]
    }

    pub fn get_gas_total(&self) -> u64 {
        self.gas_total
    }

    pub fn clear(&mut self) {
        // wtf Rust, no array::fill or equivalent?
        for i in 0..self.evm_counts.len() {
            self.evm_counts[i] = 0;
            self.gas_counts[i] = 0;
        }
        self.gas_total = 0;
    }
}
