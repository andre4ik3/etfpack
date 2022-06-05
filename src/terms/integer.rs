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

use super::*;
use crate::utils::*;

use anyhow::*;

const INTEGER_EXT: u8 = 98;

pub struct IntegerPacker;
impl Term<i32> for IntegerPacker {
    fn pack(data: i32, buf: &mut BufWriter<Cursor<Vec<u8>>>) -> Result<()> {
        let bytes = data.to_be_bytes();

        write_bytes(buf, vec![INTEGER_EXT])?;
        write_bytes(buf, bytes.to_vec())?;

        Ok(())
    }

    fn unpack(buf: &mut BufReader<Cursor<Vec<u8>>>, _: u8) -> Result<i32> {
        let bytes = read_bytes(buf, 4)?;
        Ok(i32::from_be_bytes(bytes.try_into().unwrap()))
    }

    fn can_pack(data: &AnyTerm) -> bool {
        match data {
            &AnyTerm::Integer(_) => true,
            _ => false,
        }
    }

    fn can_unpack(first_byte: &u8) -> bool {
        match first_byte {
            &INTEGER_EXT => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terms::AnyTerm;

    const VALUE: i32 = 299792458;
    const PACKED_INTEGER: [u8; 5] = [98, 17, 222, 120, 74];

    #[test]
    fn can_pack() {
        assert_eq!(IntegerPacker::can_pack(&AnyTerm::Integer(123)), true);
        assert_eq!(IntegerPacker::can_pack(&AnyTerm::SmallInt(123)), false);
    }

    #[test]
    fn can_unpack() {
        assert_eq!(IntegerPacker::can_unpack(&INTEGER_EXT), true);
        assert_eq!(IntegerPacker::can_unpack(&123), false);
    }

    #[test]
    fn pack() {
        let mut buf = BufWriter::new(Cursor::new(Vec::<u8>::new()));
        IntegerPacker::pack(VALUE, &mut buf).unwrap();
        let buf = buf.into_inner().unwrap().into_inner();
        assert_eq!(buf, PACKED_INTEGER);
    }

    #[test]
    fn unpack() {
        let mut buf = BufReader::new(Cursor::new(PACKED_INTEGER.to_vec()));
        read_bytes(&mut buf, 1).unwrap();
        assert_eq!(IntegerPacker::unpack(&mut buf, INTEGER_EXT).unwrap(), VALUE);
    }
}
