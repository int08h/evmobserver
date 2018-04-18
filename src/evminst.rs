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
//! EVM Instructions
//!
//!

use std::fmt;
use std::fmt::Formatter;

#[repr(u8)]
#[derive(Debug, PartialEq, Eq, PartialOrd, Clone, Copy)]
pub enum EvmInst {
    STOP = 0x00,
    ADD = 0x01,
    MUL = 0x02,
    SUB = 0x03,
    DIV = 0x04,
    SDIV = 0x05,
    MOD = 0x06,
    SMOD = 0x07,
    ADDMOD = 0x08,
    MULMOD = 0x09,
    EXP = 0x0a,
    SIGNEXTEND = 0x0b,
    LT = 0x10,
    GT = 0x11,
    SLT = 0x12,
    SGT = 0x13,
    EQ = 0x14,
    ISZERO = 0x15,
    AND = 0x16,
    OR = 0x17,
    XOR = 0x18,
    NOT = 0x19,
    BYTE = 0x1a,
    SHA3 = 0x20,
    ADDRESS = 0x30,
    BALANCE = 0x31,
    ORIGIN = 0x32,
    CALLER = 0x33,
    CALLVALUE = 0x34,
    CALLDATALOAD = 0x35,
    CALLDATASIZE = 0x36,
    CALLDATACOPY = 0x37,
    CODESIZE = 0x38,
    CODECOPY = 0x39,
    GASPRICE = 0x3a,
    EXTCODESIZE = 0x3b,
    EXTCODECOPY = 0x3c,
    RETURNDATASIZE = 0x3d,
    RETURNDATACOPY = 0x3e,
    BLOCKHASH = 0x40,
    COINBASE = 0x41,
    TIMESTAMP = 0x42,
    NUMBER = 0x43,
    DIFFICULTY = 0x44,
    GASLIMIT = 0x45,
    POP = 0x50,
    MLOAD = 0x51,
    MSTORE = 0x52,
    MSTORE8 = 0x53,
    SLOAD = 0x54,
    SSTORE = 0x55,
    JUMP = 0x56,
    JUMPI = 0x57,
    PC = 0x58,
    MSIZE = 0x59,
    GAS = 0x5a,
    JUMPDEST = 0x5b,
    PUSH1 = 0x60,
    PUSH2 = 0x61,
    PUSH3 = 0x62,
    PUSH4 = 0x63,
    PUSH5 = 0x64,
    PUSH6 = 0x65,
    PUSH7 = 0x66,
    PUSH8 = 0x67,
    PUSH9 = 0x68,
    PUSH10 = 0x69,
    PUSH11 = 0x6a,
    PUSH12 = 0x6b,
    PUSH13 = 0x6c,
    PUSH14 = 0x6d,
    PUSH15 = 0x6e,
    PUSH16 = 0x6f,
    PUSH17 = 0x70,
    PUSH18 = 0x71,
    PUSH19 = 0x72,
    PUSH20 = 0x73,
    PUSH21 = 0x74,
    PUSH22 = 0x75,
    PUSH23 = 0x76,
    PUSH24 = 0x77,
    PUSH25 = 0x78,
    PUSH26 = 0x79,
    PUSH27 = 0x7a,
    PUSH28 = 0x7b,
    PUSH29 = 0x7c,
    PUSH30 = 0x7d,
    PUSH31 = 0x7e,
    PUSH32 = 0x7f,
    DUP1 = 0x80,
    DUP2 = 0x81,
    DUP3 = 0x82,
    DUP4 = 0x83,
    DUP5 = 0x84,
    DUP6 = 0x85,
    DUP7 = 0x86,
    DUP8 = 0x87,
    DUP9 = 0x88,
    DUP10 = 0x89,
    DUP11 = 0x8a,
    DUP12 = 0x8b,
    DUP13 = 0x8c,
    DUP14 = 0x8d,
    DUP15 = 0x8e,
    DUP16 = 0x8f,
    SWAP1 = 0x90,
    SWAP2 = 0x91,
    SWAP3 = 0x92,
    SWAP4 = 0x93,
    SWAP5 = 0x94,
    SWAP6 = 0x95,
    SWAP7 = 0x96,
    SWAP8 = 0x97,
    SWAP9 = 0x98,
    SWAP10 = 0x99,
    SWAP11 = 0x9a,
    SWAP12 = 0x9b,
    SWAP13 = 0x9c,
    SWAP14 = 0x9d,
    SWAP15 = 0x9e,
    SWAP16 = 0x9f,
    LOG0 = 0xa0,
    LOG1 = 0xa1,
    LOG2 = 0xa2,
    LOG3 = 0xa3,
    LOG4 = 0xa4,
    CREATE = 0xf0,
    CALL = 0xf1,
    CALLCODE = 0xf2,
    RETURN = 0xf3,
    DELEGATECALL = 0xf4,
    CREATE2 = 0xfb,
    REVERT = 0xfd,
    STATICCALL = 0xfa,
    SUICIDE = 0xff,
}

impl EvmInst {
    pub fn from_str(s: &str) -> EvmInst {
        match s {
            "STOP" => EvmInst::STOP,
            "ADD" => EvmInst::ADD,
            "MUL" => EvmInst::MUL,
            "SUB" => EvmInst::SUB,
            "DIV" => EvmInst::DIV,
            "SDIV" => EvmInst::SDIV,
            "MOD" => EvmInst::MOD,
            "SMOD" => EvmInst::SMOD,
            "ADDMOD" => EvmInst::ADDMOD,
            "MULMOD" => EvmInst::MULMOD,
            "EXP" => EvmInst::EXP,
            "SIGNEXTEND" => EvmInst::SIGNEXTEND,
            "LT" => EvmInst::LT,
            "GT" => EvmInst::GT,
            "SLT" => EvmInst::SLT,
            "SGT" => EvmInst::SGT,
            "EQ" => EvmInst::EQ,
            "ISZERO" => EvmInst::ISZERO,
            "AND" => EvmInst::AND,
            "OR" => EvmInst::OR,
            "XOR" => EvmInst::XOR,
            "NOT" => EvmInst::NOT,
            "BYTE" => EvmInst::BYTE,
            "SHA3" => EvmInst::SHA3,
            "ADDRESS" => EvmInst::ADDRESS,
            "BALANCE" => EvmInst::BALANCE,
            "ORIGIN" => EvmInst::ORIGIN,
            "CALLER" => EvmInst::CALLER,
            "CALLVALUE" => EvmInst::CALLVALUE,
            "CALLDATALOAD" => EvmInst::CALLDATALOAD,
            "CALLDATASIZE" => EvmInst::CALLDATASIZE,
            "CALLDATACOPY" => EvmInst::CALLDATACOPY,
            "CODESIZE" => EvmInst::CODESIZE,
            "CODECOPY" => EvmInst::CODECOPY,
            "GASPRICE" => EvmInst::GASPRICE,
            "EXTCODESIZE" => EvmInst::EXTCODESIZE,
            "EXTCODECOPY" => EvmInst::EXTCODECOPY,
            "RETURNDATASIZE" => EvmInst::RETURNDATASIZE,
            "RETURNDATACOPY" => EvmInst::RETURNDATACOPY,
            "BLOCKHASH" => EvmInst::BLOCKHASH,
            "COINBASE" => EvmInst::COINBASE,
            "TIMESTAMP" => EvmInst::TIMESTAMP,
            "NUMBER" => EvmInst::NUMBER,
            "DIFFICULTY" => EvmInst::DIFFICULTY,
            "GASLIMIT" => EvmInst::GASLIMIT,
            "POP" => EvmInst::POP,
            "MLOAD" => EvmInst::MLOAD,
            "MSTORE" => EvmInst::MSTORE,
            "MSTORE8" => EvmInst::MSTORE8,
            "SLOAD" => EvmInst::SLOAD,
            "SSTORE" => EvmInst::SSTORE,
            "JUMP" => EvmInst::JUMP,
            "JUMPI" => EvmInst::JUMPI,
            "PC" => EvmInst::PC,
            "MSIZE" => EvmInst::MSIZE,
            "GAS" => EvmInst::GAS,
            "JUMPDEST" => EvmInst::JUMPDEST,
            "PUSH1" => EvmInst::PUSH1,
            "PUSH2" => EvmInst::PUSH2,
            "PUSH3" => EvmInst::PUSH3,
            "PUSH4" => EvmInst::PUSH4,
            "PUSH5" => EvmInst::PUSH5,
            "PUSH6" => EvmInst::PUSH6,
            "PUSH7" => EvmInst::PUSH7,
            "PUSH8" => EvmInst::PUSH8,
            "PUSH9" => EvmInst::PUSH9,
            "PUSH10" => EvmInst::PUSH10,
            "PUSH11" => EvmInst::PUSH11,
            "PUSH12" => EvmInst::PUSH12,
            "PUSH13" => EvmInst::PUSH13,
            "PUSH14" => EvmInst::PUSH14,
            "PUSH15" => EvmInst::PUSH15,
            "PUSH16" => EvmInst::PUSH16,
            "PUSH17" => EvmInst::PUSH17,
            "PUSH18" => EvmInst::PUSH18,
            "PUSH19" => EvmInst::PUSH19,
            "PUSH20" => EvmInst::PUSH20,
            "PUSH21" => EvmInst::PUSH21,
            "PUSH22" => EvmInst::PUSH22,
            "PUSH23" => EvmInst::PUSH23,
            "PUSH24" => EvmInst::PUSH24,
            "PUSH25" => EvmInst::PUSH25,
            "PUSH26" => EvmInst::PUSH26,
            "PUSH27" => EvmInst::PUSH27,
            "PUSH28" => EvmInst::PUSH28,
            "PUSH29" => EvmInst::PUSH29,
            "PUSH30" => EvmInst::PUSH30,
            "PUSH31" => EvmInst::PUSH31,
            "PUSH32" => EvmInst::PUSH32,
            "DUP1" => EvmInst::DUP1,
            "DUP2" => EvmInst::DUP2,
            "DUP3" => EvmInst::DUP3,
            "DUP4" => EvmInst::DUP4,
            "DUP5" => EvmInst::DUP5,
            "DUP6" => EvmInst::DUP6,
            "DUP7" => EvmInst::DUP7,
            "DUP8" => EvmInst::DUP8,
            "DUP9" => EvmInst::DUP9,
            "DUP10" => EvmInst::DUP10,
            "DUP11" => EvmInst::DUP11,
            "DUP12" => EvmInst::DUP12,
            "DUP13" => EvmInst::DUP13,
            "DUP14" => EvmInst::DUP14,
            "DUP15" => EvmInst::DUP15,
            "DUP16" => EvmInst::DUP16,
            "SWAP1" => EvmInst::SWAP1,
            "SWAP2" => EvmInst::SWAP2,
            "SWAP3" => EvmInst::SWAP3,
            "SWAP4" => EvmInst::SWAP4,
            "SWAP5" => EvmInst::SWAP5,
            "SWAP6" => EvmInst::SWAP6,
            "SWAP7" => EvmInst::SWAP7,
            "SWAP8" => EvmInst::SWAP8,
            "SWAP9" => EvmInst::SWAP9,
            "SWAP10" => EvmInst::SWAP10,
            "SWAP11" => EvmInst::SWAP11,
            "SWAP12" => EvmInst::SWAP12,
            "SWAP13" => EvmInst::SWAP13,
            "SWAP14" => EvmInst::SWAP14,
            "SWAP15" => EvmInst::SWAP15,
            "SWAP16" => EvmInst::SWAP16,
            "LOG0" => EvmInst::LOG0,
            "LOG1" => EvmInst::LOG1,
            "LOG2" => EvmInst::LOG2,
            "LOG3" => EvmInst::LOG3,
            "LOG4" => EvmInst::LOG4,
            "CREATE" => EvmInst::CREATE,
            "CALL" => EvmInst::CALL,
            "CALLCODE" => EvmInst::CALLCODE,
            "RETURN" => EvmInst::RETURN,
            "DELEGATECALL" => EvmInst::DELEGATECALL,
            "CREATE2" => EvmInst::CREATE2,
            "REVERT" => EvmInst::REVERT,
            "STATICCALL" => EvmInst::STATICCALL,
            "SUICIDE" => EvmInst::SUICIDE,
            _ => panic!("unknown instruction {}", s)
        }
    }

    pub fn from_opt_str(s: Option<&str>) -> EvmInst {
        EvmInst::from_str(s.expect("missing envinst?"))
    }
}

impl fmt::Display for EvmInst {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

pub fn as_str(op: &EvmInst) -> &str {
    match *op {
        EvmInst::STOP => "STOP",
        EvmInst::ADD => "ADD",
        EvmInst::MUL => "MUL",
        EvmInst::SUB => "SUB",
        EvmInst::DIV => "DIV",
        EvmInst::SDIV => "SDIV",
        EvmInst::MOD => "MOD",
        EvmInst::SMOD => "SMOD",
        EvmInst::ADDMOD => "ADDMOD",
        EvmInst::MULMOD => "MULMOD",
        EvmInst::EXP => "EXP",
        EvmInst::SIGNEXTEND => "SIGNEXTEND",
        EvmInst::LT => "LT",
        EvmInst::GT => "GT",
        EvmInst::SLT => "SLT",
        EvmInst::SGT => "SGT",
        EvmInst::EQ => "EQ",
        EvmInst::ISZERO => "ISZERO",
        EvmInst::AND => "AND",
        EvmInst::OR => "OR",
        EvmInst::XOR => "XOR",
        EvmInst::NOT => "NOT",
        EvmInst::BYTE => "BYTE",
        EvmInst::SHA3 => "SHA3",
        EvmInst::ADDRESS => "ADDRESS",
        EvmInst::BALANCE => "BALANCE",
        EvmInst::ORIGIN => "ORIGIN",
        EvmInst::CALLER => "CALLER",
        EvmInst::CALLVALUE => "CALLVALUE",
        EvmInst::CALLDATALOAD => "CALLDATALOAD",
        EvmInst::CALLDATASIZE => "CALLDATASIZE",
        EvmInst::CALLDATACOPY => "CALLDATACOPY",
        EvmInst::CODESIZE => "CODESIZE",
        EvmInst::CODECOPY => "CODECOPY",
        EvmInst::GASPRICE => "GASPRICE",
        EvmInst::EXTCODESIZE => "EXTCODESIZE",
        EvmInst::EXTCODECOPY => "EXTCODECOPY",
        EvmInst::RETURNDATASIZE => "RETURNDATASIZE",
        EvmInst::RETURNDATACOPY => "RETURNDATACOPY",
        EvmInst::BLOCKHASH => "BLOCKHASH",
        EvmInst::COINBASE => "COINBASE",
        EvmInst::TIMESTAMP => "TIMESTAMP",
        EvmInst::NUMBER => "NUMBER",
        EvmInst::DIFFICULTY => "DIFFICULTY",
        EvmInst::GASLIMIT => "GASLIMIT",
        EvmInst::POP => "POP",
        EvmInst::MLOAD => "MLOAD",
        EvmInst::MSTORE => "MSTORE",
        EvmInst::MSTORE8 => "MSTORE8",
        EvmInst::SLOAD => "SLOAD",
        EvmInst::SSTORE => "SSTORE",
        EvmInst::JUMP => "JUMP",
        EvmInst::JUMPI => "JUMPI",
        EvmInst::PC => "PC",
        EvmInst::MSIZE => "MSIZE",
        EvmInst::GAS => "GAS",
        EvmInst::JUMPDEST => "JUMPDEST",
        EvmInst::PUSH1 => "PUSH1",
        EvmInst::PUSH2 => "PUSH2",
        EvmInst::PUSH3 => "PUSH3",
        EvmInst::PUSH4 => "PUSH4",
        EvmInst::PUSH5 => "PUSH5",
        EvmInst::PUSH6 => "PUSH6",
        EvmInst::PUSH7 => "PUSH7",
        EvmInst::PUSH8 => "PUSH8",
        EvmInst::PUSH9 => "PUSH9",
        EvmInst::PUSH10 => "PUSH10",
        EvmInst::PUSH11 => "PUSH11",
        EvmInst::PUSH12 => "PUSH12",
        EvmInst::PUSH13 => "PUSH13",
        EvmInst::PUSH14 => "PUSH14",
        EvmInst::PUSH15 => "PUSH15",
        EvmInst::PUSH16 => "PUSH16",
        EvmInst::PUSH17 => "PUSH17",
        EvmInst::PUSH18 => "PUSH18",
        EvmInst::PUSH19 => "PUSH19",
        EvmInst::PUSH20 => "PUSH20",
        EvmInst::PUSH21 => "PUSH21",
        EvmInst::PUSH22 => "PUSH22",
        EvmInst::PUSH23 => "PUSH23",
        EvmInst::PUSH24 => "PUSH24",
        EvmInst::PUSH25 => "PUSH25",
        EvmInst::PUSH26 => "PUSH26",
        EvmInst::PUSH27 => "PUSH27",
        EvmInst::PUSH28 => "PUSH28",
        EvmInst::PUSH29 => "PUSH29",
        EvmInst::PUSH30 => "PUSH30",
        EvmInst::PUSH31 => "PUSH31",
        EvmInst::PUSH32 => "PUSH32",
        EvmInst::DUP1 => "DUP1",
        EvmInst::DUP2 => "DUP2",
        EvmInst::DUP3 => "DUP3",
        EvmInst::DUP4 => "DUP4",
        EvmInst::DUP5 => "DUP5",
        EvmInst::DUP6 => "DUP6",
        EvmInst::DUP7 => "DUP7",
        EvmInst::DUP8 => "DUP8",
        EvmInst::DUP9 => "DUP9",
        EvmInst::DUP10 => "DUP10",
        EvmInst::DUP11 => "DUP11",
        EvmInst::DUP12 => "DUP12",
        EvmInst::DUP13 => "DUP13",
        EvmInst::DUP14 => "DUP14",
        EvmInst::DUP15 => "DUP15",
        EvmInst::DUP16 => "DUP16",
        EvmInst::SWAP1 => "SWAP1",
        EvmInst::SWAP2 => "SWAP2",
        EvmInst::SWAP3 => "SWAP3",
        EvmInst::SWAP4 => "SWAP4",
        EvmInst::SWAP5 => "SWAP5",
        EvmInst::SWAP6 => "SWAP6",
        EvmInst::SWAP7 => "SWAP7",
        EvmInst::SWAP8 => "SWAP8",
        EvmInst::SWAP9 => "SWAP9",
        EvmInst::SWAP10 => "SWAP10",
        EvmInst::SWAP11 => "SWAP11",
        EvmInst::SWAP12 => "SWAP12",
        EvmInst::SWAP13 => "SWAP13",
        EvmInst::SWAP14 => "SWAP14",
        EvmInst::SWAP15 => "SWAP15",
        EvmInst::SWAP16 => "SWAP16",
        EvmInst::LOG0 => "LOG0",
        EvmInst::LOG1 => "LOG1",
        EvmInst::LOG2 => "LOG2",
        EvmInst::LOG3 => "LOG3",
        EvmInst::LOG4 => "LOG4",
        EvmInst::CREATE => "CREATE",
        EvmInst::CALL => "CALL",
        EvmInst::CALLCODE => "CALLCODE",
        EvmInst::RETURN => "RETURN",
        EvmInst::DELEGATECALL => "DELEGATECALL",
        EvmInst::CREATE2 => "CREATE2",
        EvmInst::REVERT => "REVERT",
        EvmInst::STATICCALL => "STATICCALL",
        EvmInst::SUICIDE => "SUICIDE",
    }
}

pub static VALUES: [EvmInst; 135] = [
    EvmInst::STOP,
    EvmInst::ADD,
    EvmInst::MUL,
    EvmInst::SUB,
    EvmInst::DIV,
    EvmInst::SDIV,
    EvmInst::MOD,
    EvmInst::SMOD,
    EvmInst::ADDMOD,
    EvmInst::MULMOD,
    EvmInst::EXP,
    EvmInst::SIGNEXTEND,
    EvmInst::LT,
    EvmInst::GT,
    EvmInst::SLT,
    EvmInst::SGT,
    EvmInst::EQ,
    EvmInst::ISZERO,
    EvmInst::AND,
    EvmInst::OR,
    EvmInst::XOR,
    EvmInst::NOT,
    EvmInst::BYTE,
    EvmInst::SHA3,
    EvmInst::ADDRESS,
    EvmInst::BALANCE,
    EvmInst::ORIGIN,
    EvmInst::CALLER,
    EvmInst::CALLVALUE,
    EvmInst::CALLDATALOAD,
    EvmInst::CALLDATASIZE,
    EvmInst::CALLDATACOPY,
    EvmInst::CODESIZE,
    EvmInst::CODECOPY,
    EvmInst::GASPRICE,
    EvmInst::EXTCODESIZE,
    EvmInst::EXTCODECOPY,
    EvmInst::RETURNDATASIZE,
    EvmInst::RETURNDATACOPY,
    EvmInst::BLOCKHASH,
    EvmInst::COINBASE,
    EvmInst::TIMESTAMP,
    EvmInst::NUMBER,
    EvmInst::DIFFICULTY,
    EvmInst::GASLIMIT,
    EvmInst::POP,
    EvmInst::MLOAD,
    EvmInst::MSTORE,
    EvmInst::MSTORE8,
    EvmInst::SLOAD,
    EvmInst::SSTORE,
    EvmInst::JUMP,
    EvmInst::JUMPI,
    EvmInst::PC,
    EvmInst::MSIZE,
    EvmInst::GAS,
    EvmInst::JUMPDEST,
    EvmInst::PUSH1,
    EvmInst::PUSH2,
    EvmInst::PUSH3,
    EvmInst::PUSH4,
    EvmInst::PUSH5,
    EvmInst::PUSH6,
    EvmInst::PUSH7,
    EvmInst::PUSH8,
    EvmInst::PUSH9,
    EvmInst::PUSH10,
    EvmInst::PUSH11,
    EvmInst::PUSH12,
    EvmInst::PUSH13,
    EvmInst::PUSH14,
    EvmInst::PUSH15,
    EvmInst::PUSH16,
    EvmInst::PUSH17,
    EvmInst::PUSH18,
    EvmInst::PUSH19,
    EvmInst::PUSH20,
    EvmInst::PUSH21,
    EvmInst::PUSH22,
    EvmInst::PUSH23,
    EvmInst::PUSH24,
    EvmInst::PUSH25,
    EvmInst::PUSH26,
    EvmInst::PUSH27,
    EvmInst::PUSH28,
    EvmInst::PUSH29,
    EvmInst::PUSH30,
    EvmInst::PUSH31,
    EvmInst::PUSH32,
    EvmInst::DUP1,
    EvmInst::DUP2,
    EvmInst::DUP3,
    EvmInst::DUP4,
    EvmInst::DUP5,
    EvmInst::DUP6,
    EvmInst::DUP7,
    EvmInst::DUP8,
    EvmInst::DUP9,
    EvmInst::DUP10,
    EvmInst::DUP11,
    EvmInst::DUP12,
    EvmInst::DUP13,
    EvmInst::DUP14,
    EvmInst::DUP15,
    EvmInst::DUP16,
    EvmInst::SWAP1,
    EvmInst::SWAP2,
    EvmInst::SWAP3,
    EvmInst::SWAP4,
    EvmInst::SWAP5,
    EvmInst::SWAP6,
    EvmInst::SWAP7,
    EvmInst::SWAP8,
    EvmInst::SWAP9,
    EvmInst::SWAP10,
    EvmInst::SWAP11,
    EvmInst::SWAP12,
    EvmInst::SWAP13,
    EvmInst::SWAP14,
    EvmInst::SWAP15,
    EvmInst::SWAP16,
    EvmInst::LOG0,
    EvmInst::LOG1,
    EvmInst::LOG2,
    EvmInst::LOG3,
    EvmInst::LOG4,
    EvmInst::CREATE,
    EvmInst::CALL,
    EvmInst::CALLCODE,
    EvmInst::RETURN,
    EvmInst::DELEGATECALL,
    EvmInst::CREATE2,
    EvmInst::REVERT,
    EvmInst::STATICCALL,
    EvmInst::SUICIDE,
];
