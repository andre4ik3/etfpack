//! etfpack - a serializer/deserializer for the Erlang Term Format.
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

mod packing;
mod structs;
mod terms;
mod utils;

use crate::packing::*;
use crate::terms::AnyTerm;

use anyhow::*;
use std::io::{BufReader, BufWriter, Cursor};
use utils::{read_bytes, write_bytes};

/// Packs a term into bytes.
/// TODO: make the result type concrete with a custom error type
pub fn pack(data: AnyTerm) -> Result<Vec<u8>> {
    let mut buf = BufWriter::new(Cursor::new(Vec::<u8>::new()));
    write_bytes(&mut buf, vec![FORMAT_VERSION])?;
    pack_buf(&mut buf, data)?;
    Ok(buf.into_inner()?.into_inner())
}

/// Unpacks some bytes into a term.
/// TODO: make the result type concrete with a custom error type
pub fn unpack(data: Vec<u8>) -> Result<AnyTerm> {
    let mut buf = BufReader::new(Cursor::new(data));
    let version = read_bytes(&mut buf, 1)?[0];

    if version != FORMAT_VERSION {
        return Err(anyhow!("Format version mismatch!"));
    }

    Ok(unpack_buf(&mut buf)?)
}
