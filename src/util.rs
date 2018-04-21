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


/// Convert a `0x` prefixed hex string to u64
///
/// Example: `hex_to_u64("0xff") -> 255`
pub fn hex_to_u64(s: &str) -> Option<u64> {
    if s.len() > 2 {
        // skip leading 0x
        match u64::from_str_radix(&s[2..], 16) {
            Ok(val) => Some(val),
            Err(_) => None
        }
    } else {
        None
    }
}
