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

//noinspection RsFieldNaming
#[allow(non_snake_case)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvmTrace {
    pub ts: u64,
    pub block_num: u32,
    pub txn_index: u16,
    pub addr_from: String,
    pub addr_to: String,
    pub gas_px: u64,
    pub STOP_count: u16,
    pub STOP_gas: u32,
    pub ADD_count: u16,
    pub ADD_gas: u32,
    pub MUL_count: u16,
    pub MUL_gas: u32,
    pub SUB_count: u16,
    pub SUB_gas: u32,
    pub DIV_count: u16,
    pub DIV_gas: u32,
    pub SDIV_count: u16,
    pub SDIV_gas: u32,
    pub MOD_count: u16,
    pub MOD_gas: u32,
    pub SMOD_count: u16,
    pub SMOD_gas: u32,
    pub ADDMOD_count: u16,
    pub ADDMOD_gas: u32,
    pub MULMOD_count: u16,
    pub MULMOD_gas: u32,
    pub EXP_count: u16,
    pub EXP_gas: u32,
    pub SIGNEXTEND_count: u16,
    pub SIGNEXTEND_gas: u32,
    pub LT_count: u16,
    pub LT_gas: u32,
    pub GT_count: u16,
    pub GT_gas: u32,
    pub SLT_count: u16,
    pub SLT_gas: u32,
    pub SGT_count: u16,
    pub SGT_gas: u32,
    pub EQ_count: u16,
    pub EQ_gas: u32,
    pub ISZERO_count: u16,
    pub ISZERO_gas: u32,
    pub AND_count: u16,
    pub AND_gas: u32,
    pub OR_count: u16,
    pub OR_gas: u32,
    pub XOR_count: u16,
    pub XOR_gas: u32,
    pub NOT_count: u16,
    pub NOT_gas: u32,
    pub BYTE_count: u16,
    pub BYTE_gas: u32,
    pub SHA3_count: u16,
    pub SHA3_gas: u32,
    pub ADDRESS_count: u16,
    pub ADDRESS_gas: u32,
    pub BALANCE_count: u16,
    pub BALANCE_gas: u32,
    pub ORIGIN_count: u16,
    pub ORIGIN_gas: u32,
    pub CALLER_count: u16,
    pub CALLER_gas: u32,
    pub CALLVALUE_count: u16,
    pub CALLVALUE_gas: u32,
    pub CALLDATALOAD_count: u16,
    pub CALLDATALOAD_gas: u32,
    pub CALLDATASIZE_count: u16,
    pub CALLDATASIZE_gas: u32,
    pub CALLDATACOPY_count: u16,
    pub CALLDATACOPY_gas: u32,
    pub CODESIZE_count: u16,
    pub CODESIZE_gas: u32,
    pub CODECOPY_count: u16,
    pub CODECOPY_gas: u32,
    pub GASPRICE_count: u16,
    pub GASPRICE_gas: u32,
    pub EXTCODESIZE_count: u16,
    pub EXTCODESIZE_gas: u32,
    pub EXTCODECOPY_count: u16,
    pub EXTCODECOPY_gas: u32,
    pub RETURNDATASIZE_count: u16,
    pub RETURNDATASIZE_gas: u32,
    pub RETURNDATACOPY_count: u16,
    pub RETURNDATACOPY_gas: u32,
    pub BLOCKHASH_count: u16,
    pub BLOCKHASH_gas: u32,
    pub COINBASE_count: u16,
    pub COINBASE_gas: u32,
    pub TIMESTAMP_count: u16,
    pub TIMESTAMP_gas: u32,
    pub NUMBER_count: u16,
    pub NUMBER_gas: u32,
    pub DIFFICULTY_count: u16,
    pub DIFFICULTY_gas: u32,
    pub GASLIMIT_count: u16,
    pub GASLIMIT_gas: u32,
    pub POP_count: u16,
    pub POP_gas: u32,
    pub MLOAD_count: u16,
    pub MLOAD_gas: u32,
    pub MSTORE_count: u16,
    pub MSTORE_gas: u32,
    pub MSTORE8_count: u16,
    pub MSTORE8_gas: u32,
    pub SLOAD_count: u16,
    pub SLOAD_gas: u32,
    pub SSTORE_count: u16,
    pub SSTORE_gas: u32,
    pub JUMP_count: u16,
    pub JUMP_gas: u32,
    pub JUMPI_count: u16,
    pub JUMPI_gas: u32,
    pub PC_count: u16,
    pub PC_gas: u32,
    pub MSIZE_count: u16,
    pub MSIZE_gas: u32,
    pub GAS_count: u16,
    pub GAS_gas: u32,
    pub JUMPDEST_count: u16,
    pub JUMPDEST_gas: u32,
    pub PUSH1_count: u16,
    pub PUSH1_gas: u32,
    pub PUSH2_count: u16,
    pub PUSH2_gas: u32,
    pub PUSH3_count: u16,
    pub PUSH3_gas: u32,
    pub PUSH4_count: u16,
    pub PUSH4_gas: u32,
    pub PUSH5_count: u16,
    pub PUSH5_gas: u32,
    pub PUSH6_count: u16,
    pub PUSH6_gas: u32,
    pub PUSH7_count: u16,
    pub PUSH7_gas: u32,
    pub PUSH8_count: u16,
    pub PUSH8_gas: u32,
    pub PUSH9_count: u16,
    pub PUSH9_gas: u32,
    pub PUSH10_count: u16,
    pub PUSH10_gas: u32,
    pub PUSH11_count: u16,
    pub PUSH11_gas: u32,
    pub PUSH12_count: u16,
    pub PUSH12_gas: u32,
    pub PUSH13_count: u16,
    pub PUSH13_gas: u32,
    pub PUSH14_count: u16,
    pub PUSH14_gas: u32,
    pub PUSH15_count: u16,
    pub PUSH15_gas: u32,
    pub PUSH16_count: u16,
    pub PUSH16_gas: u32,
    pub PUSH17_count: u16,
    pub PUSH17_gas: u32,
    pub PUSH18_count: u16,
    pub PUSH18_gas: u32,
    pub PUSH19_count: u16,
    pub PUSH19_gas: u32,
    pub PUSH20_count: u16,
    pub PUSH20_gas: u32,
    pub PUSH21_count: u16,
    pub PUSH21_gas: u32,
    pub PUSH22_count: u16,
    pub PUSH22_gas: u32,
    pub PUSH23_count: u16,
    pub PUSH23_gas: u32,
    pub PUSH24_count: u16,
    pub PUSH24_gas: u32,
    pub PUSH25_count: u16,
    pub PUSH25_gas: u32,
    pub PUSH26_count: u16,
    pub PUSH26_gas: u32,
    pub PUSH27_count: u16,
    pub PUSH27_gas: u32,
    pub PUSH28_count: u16,
    pub PUSH28_gas: u32,
    pub PUSH29_count: u16,
    pub PUSH29_gas: u32,
    pub PUSH30_count: u16,
    pub PUSH30_gas: u32,
    pub PUSH31_count: u16,
    pub PUSH31_gas: u32,
    pub PUSH32_count: u16,
    pub PUSH32_gas: u32,
    pub DUP1_count: u16,
    pub DUP1_gas: u32,
    pub DUP2_count: u16,
    pub DUP2_gas: u32,
    pub DUP3_count: u16,
    pub DUP3_gas: u32,
    pub DUP4_count: u16,
    pub DUP4_gas: u32,
    pub DUP5_count: u16,
    pub DUP5_gas: u32,
    pub DUP6_count: u16,
    pub DUP6_gas: u32,
    pub DUP7_count: u16,
    pub DUP7_gas: u32,
    pub DUP8_count: u16,
    pub DUP8_gas: u32,
    pub DUP9_count: u16,
    pub DUP9_gas: u32,
    pub DUP10_count: u16,
    pub DUP10_gas: u32,
    pub DUP11_count: u16,
    pub DUP11_gas: u32,
    pub DUP12_count: u16,
    pub DUP12_gas: u32,
    pub DUP13_count: u16,
    pub DUP13_gas: u32,
    pub DUP14_count: u16,
    pub DUP14_gas: u32,
    pub DUP15_count: u16,
    pub DUP15_gas: u32,
    pub DUP16_count: u16,
    pub DUP16_gas: u32,
    pub SWAP1_count: u16,
    pub SWAP1_gas: u32,
    pub SWAP2_count: u16,
    pub SWAP2_gas: u32,
    pub SWAP3_count: u16,
    pub SWAP3_gas: u32,
    pub SWAP4_count: u16,
    pub SWAP4_gas: u32,
    pub SWAP5_count: u16,
    pub SWAP5_gas: u32,
    pub SWAP6_count: u16,
    pub SWAP6_gas: u32,
    pub SWAP7_count: u16,
    pub SWAP7_gas: u32,
    pub SWAP8_count: u16,
    pub SWAP8_gas: u32,
    pub SWAP9_count: u16,
    pub SWAP9_gas: u32,
    pub SWAP10_count: u16,
    pub SWAP10_gas: u32,
    pub SWAP11_count: u16,
    pub SWAP11_gas: u32,
    pub SWAP12_count: u16,
    pub SWAP12_gas: u32,
    pub SWAP13_count: u16,
    pub SWAP13_gas: u32,
    pub SWAP14_count: u16,
    pub SWAP14_gas: u32,
    pub SWAP15_count: u16,
    pub SWAP15_gas: u32,
    pub SWAP16_count: u16,
    pub SWAP16_gas: u32,
    pub LOG0_count: u16,
    pub LOG0_gas: u32,
    pub LOG1_count: u16,
    pub LOG1_gas: u32,
    pub LOG2_count: u16,
    pub LOG2_gas: u32,
    pub LOG3_count: u16,
    pub LOG3_gas: u32,
    pub LOG4_count: u16,
    pub LOG4_gas: u32,
    pub CREATE_count: u16,
    pub CREATE_gas: u32,
    pub CALL_count: u16,
    pub CALL_gas: u32,
    pub CALLCODE_count: u16,
    pub CALLCODE_gas: u32,
    pub RETURN_count: u16,
    pub RETURN_gas: u32,
    pub DELEGATECALL_count: u16,
    pub DELEGATECALL_gas: u32,
    pub CREATE2_count: u16,
    pub CREATE2_gas: u32,
    pub REVERT_count: u16,
    pub REVERT_gas: u32,
    pub STATICCALL_count: u16,
    pub STATICCALL_gas: u32,
    pub INVALID_count: u16,
    pub INVALID_gas: u32,
    pub SUICIDE_count: u16,
    pub SUICIDE_gas: u32,
}
