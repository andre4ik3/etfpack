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

use anyhow::*;

use super::*;
use crate::utils::*;

const SMALL_INTEGER_EXT: u8 = 97;

pub struct SmallIntPacker;
impl Term<u8> for SmallIntPacker {
    fn pack(data: u8, buf: &mut BufWriter<Cursor<Vec<u8>>>) -> Result<()> {
        write_bytes(buf, vec![SMALL_INTEGER_EXT, data])?;
        Ok(())
    }

    fn unpack(buf: &mut BufReader<Cursor<Vec<u8>>>, _: u8) -> Result<u8> {
        let bytes = read_bytes(buf, 1)?;
        Ok(bytes[0])
    }

    fn can_pack(data: &AnyTerm) -> bool {
        match data {
            &AnyTerm::SmallInt(_) => true,
            _ => false,
        }
    }

    fn can_unpack(first_byte: &u8) -> bool {
        first_byte == &SMALL_INTEGER_EXT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALUE: u8 = 248;
    const PACKED_INTEGER: [u8; 2] = [97, 248];

    #[test]
    fn pack() {
        let mut buf = BufWriter::new(Cursor::new(Vec::<u8>::new()));
        SmallIntPacker::pack(VALUE, &mut buf).unwrap();
        let buf = buf.into_inner().unwrap().into_inner();
        assert_eq!(buf, PACKED_INTEGER);
    }

    #[test]
    fn unpack() {
        let mut buf = BufReader::new(Cursor::new(PACKED_INTEGER.to_vec()));
        read_bytes(&mut buf, 1).unwrap();
        assert_eq!(
            SmallIntPacker::unpack(&mut buf, SMALL_INTEGER_EXT).unwrap(),
            VALUE
        );
    }
}
