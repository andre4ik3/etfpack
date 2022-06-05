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

const FLOAT_EXT: u8 = 99;
const NEW_FLOAT_EXT: u8 = 70;

pub struct FloatPacker;
impl Term<f64> for FloatPacker {
    /// Always packs as new (IEEE) float.
    fn pack(data: f64, buf: &mut BufWriter<Cursor<Vec<u8>>>) -> Result<()> {
        write_bytes(buf, vec![NEW_FLOAT_EXT])?;
        write_bytes(buf, data.to_be_bytes().to_vec())?;
        Ok(())
    }

    fn unpack(buf: &mut BufReader<Cursor<Vec<u8>>>, fb: u8) -> Result<f64> {
        if !FloatPacker::can_unpack(&fb) {
            return Err(anyhow!("Unknown float type"));
        }

        if fb == FLOAT_EXT {
            let string = read_bytes(buf, 31)?;
            let string = str_from_u8_nul_utf8(&string)?;
            return Ok(string.parse::<f64>()?);
        } else if fb == NEW_FLOAT_EXT {
            let bytes = read_bytes(buf, 8)?;
            // This shouldn't panic because read_bytes checks length.
            return Ok(f64::from_be_bytes(bytes.try_into().unwrap()));
        }

        // checked above
        unreachable!()
    }

    fn can_pack(data: &AnyTerm) -> bool {
        match data {
            &AnyTerm::Float(_) => true,
            _ => false,
        }
    }

    fn can_unpack(first_byte: &u8) -> bool {
        first_byte == &FLOAT_EXT || first_byte == &NEW_FLOAT_EXT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALUE: f64 = 1.234;
    const PACKED_FLOAT: [u8; 32] = [
        99, 49, 46, 50, 51, 52, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        0, 0, 0, 0,
    ];
    const PACKED_NEW_FLOAT: [u8; 9] = [70, 63, 243, 190, 118, 200, 180, 57, 88];

    #[test]
    fn pack() {
        let mut buf = BufWriter::new(Cursor::new(Vec::<u8>::new()));
        FloatPacker::pack(VALUE, &mut buf).unwrap();
        let buf = buf.into_inner().unwrap().into_inner();
        assert_eq!(buf, PACKED_NEW_FLOAT);
    }

    #[test]
    fn unpack_old() {
        let mut buf = BufReader::new(Cursor::new(PACKED_FLOAT.to_vec()));
        let fb = read_bytes(&mut buf, 1).unwrap()[0];
        assert_eq!(FloatPacker::unpack(&mut buf, fb).unwrap(), VALUE);
    }

    #[test]
    fn unpack_new() {
        let mut buf = BufReader::new(Cursor::new(PACKED_NEW_FLOAT.to_vec()));
        let fb = read_bytes(&mut buf, 1).unwrap()[0];
        assert_eq!(FloatPacker::unpack(&mut buf, fb).unwrap(), VALUE);
    }
}
