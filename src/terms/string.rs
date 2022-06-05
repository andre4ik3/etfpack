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

const STRING_EXT: u8 = 107;

pub struct StringPacker;
impl Term<String> for StringPacker {
    fn pack(data: String, buf: &mut BufWriter<Cursor<Vec<u8>>>) -> Result<()> {
        let length = u16::try_from(data.len())?.to_be_bytes().to_vec();
        write_bytes(buf, vec![STRING_EXT])?;
        write_bytes(buf, length)?;
        write_bytes(buf, data.into_bytes())?;
        Ok(())
    }

    fn unpack(buf: &mut BufReader<Cursor<Vec<u8>>>, _: u8) -> Result<String> {
        let length = read_bytes(buf, 2)?;
        let length = u16::from_be_bytes(length.try_into().unwrap());
        let bytes = read_bytes(buf, length.into())?;
        Ok(String::from_utf8(bytes)?)
    }

    fn can_pack(data: &AnyTerm) -> bool {
        match data {
            AnyTerm::String(s) => s.chars().all(char::is_alphanumeric) && s.len() <= 65535,
            _ => false,
        }
    }

    fn can_unpack(first_byte: &u8) -> bool {
        first_byte == &STRING_EXT
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALUE: &str = "Hello";
    const PACKED_VALUE: [u8; 8] = [107, 0, 5, 72, 101, 108, 108, 111];

    #[test]
    fn pack() {
        let mut buf = BufWriter::new(Cursor::new(Vec::<u8>::new()));
        StringPacker::pack(VALUE.to_string(), &mut buf).unwrap();
        let buf = buf.into_inner().unwrap().into_inner();
        assert_eq!(buf, PACKED_VALUE);
    }

    #[test]
    fn unpack() {
        let mut buf = BufReader::new(Cursor::new(PACKED_VALUE.to_vec()));
        let fb = read_bytes(&mut buf, 1).unwrap()[0];
        let value = StringPacker::unpack(&mut buf, fb).unwrap();
        assert_eq!(value, VALUE.to_string());
    }
}
