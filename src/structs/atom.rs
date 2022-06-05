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

use serde::{Deserialize, Serialize};

/// The possible types of an atom.
#[derive(Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum AtomKind {
    /// ATOM_UTF8_EXT
    UTF8,
    /// SMALL_ATOM_UTF8_EXT
    SmallUTF8,
    /// ATOM_EXT (deprecated)
    Legacy,
    // SMALL_ATOM_EXT (deprecated)
    SmallLegacy,
}

/// Represents an atom value.
#[derive(Serialize, Deserialize, Debug)]
pub struct Atom {
    /// The kind of the atom.
    pub kind: AtomKind,
    /// The value of the atom, as a string.
    pub value: String,
}
