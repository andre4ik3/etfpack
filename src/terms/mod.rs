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

mod atom;
mod float;
mod integer;
mod small_integer;
mod string;

pub use atom::*;
pub use float::*;
pub use integer::*;
pub use small_integer::*;
pub use string::*;

use crate::structs::*;

use anyhow::Result;
use std::io::{BufReader, BufWriter, Cursor};

/// Represents an Erlang term.
pub trait Term<T> {
    /// This function should write the data to the buffer.
    /// It should write the Term ID as the first byte.
    fn pack(data: T, buf: &mut BufWriter<Cursor<Vec<u8>>>) -> Result<()>;

    /// This function should read the buffer and return the data.
    /// It should look at the first byte to determine the term variant.
    fn unpack(buf: &mut BufReader<Cursor<Vec<u8>>>, fb: u8) -> Result<T>;

    // Return true if this data type can be packed by the implementation.
    fn can_pack(data: &AnyTerm) -> bool;

    /// Return true if the first byte (Term ID) is one that this implementation
    /// can unpack.
    fn can_unpack(first_byte: &u8) -> bool;
}

/// Represents any term value, in unpacked form.
pub enum AnyTerm {
    SmallInt(u8),
    Integer(i32),
    Float(f64),
    Port(Port),
    Atom(Atom),
    String(String),
}
