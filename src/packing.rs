//! Written in Rust, to be embedded into other languages.
//! Copyright 2022 andre4ik3
//!
//! Licensed under the Apache License, Version 2.0 (the "License");
//! you may not use this file except in compliance with the License.
//! You may obtain a copy of the License at
//!
//!     http://www.apache.org/licenses/LICENSE-2.0
//!
//! Unless required by applicable law or agreed to in writing, software
//! distributed under the License is distributed on an "AS IS" BASIS,
//! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//! See the License for the specific language governing permissions and
//! limitations under the License.

use crate::{terms::*, utils::*};

use anyhow::*;
use std::io::{BufReader, BufWriter, Cursor};

pub const FORMAT_VERSION: u8 = 131;

/// Internal function that operates on a buf writer.
pub fn pack_buf(_: &mut BufWriter<Cursor<Vec<u8>>>, _: AnyTerm) -> Result<()> {
    todo!()
}

/// Internal function that operates on a buf reader.
pub fn unpack_buf(buf: &mut BufReader<Cursor<Vec<u8>>>) -> Result<AnyTerm> {
    let fb = read_bytes(buf, 1)?[0];

    if SmallIntPacker::can_unpack(&fb) {
        Ok(AnyTerm::SmallInt(SmallIntPacker::unpack(buf, fb)?))
    } else if IntegerPacker::can_unpack(&fb) {
        Ok(AnyTerm::Integer(IntegerPacker::unpack(buf, fb)?))
    } else if FloatPacker::can_unpack(&fb) {
        Ok(AnyTerm::Float(FloatPacker::unpack(buf, fb)?))
    } else if AtomPacker::can_unpack(&fb) {
        Ok(AnyTerm::Atom(AtomPacker::unpack(buf, fb)?))
    } else {
        Err(anyhow!("Unknown first byte"))
    }
}
