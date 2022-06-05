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

const ATOM_UTF8_EXT: u8 = 118;
const SMALL_ATOM_UTF8_EXT: u8 = 119;
const ATOM_EXT: u8 = 100;
const SMALL_ATOM_EXT: u8 = 115;

pub struct AtomPacker;
impl Term<Atom> for AtomPacker {
    fn pack(data: Atom, buf: &mut BufWriter<Cursor<Vec<u8>>>) -> Result<()> {
        let is_ascii = data.value.chars().all(char::is_alphanumeric);
        let max_length: usize = match data.kind {
            AtomKind::SmallUTF8 | AtomKind::SmallLegacy => u8::MAX.into(),
            AtomKind::UTF8 | AtomKind::Legacy => u16::MAX.into(),
        };

        // Check if the string can actually be parsed...
        if !is_ascii {
            if let AtomKind::Legacy | AtomKind::SmallLegacy = data.kind {
                return Err(anyhow!("String not ASCII"));
            };
        }

        // ... and whether or not it can fit.
        if data.value.len() > max_length {
            return Err(anyhow!("String too large"));
        }

        // Work out the first byte ("Term ID") to write.
        let first_byte = match data.kind {
            AtomKind::UTF8 => ATOM_UTF8_EXT,
            AtomKind::SmallUTF8 => SMALL_ATOM_UTF8_EXT,
            AtomKind::Legacy => ATOM_EXT,
            AtomKind::SmallLegacy => SMALL_ATOM_EXT,
        };

        // Then work out the length. This shouldn't panic because length was
        // already checked above.
        let length: Vec<u8> = {
            if max_length == u8::MAX.into() {
                vec![u8::try_from(data.value.len()).unwrap()]
            } else {
                u16::try_from(data.value.len())
                    .unwrap()
                    .to_be_bytes()
                    .to_vec()
            }
        };

        write_bytes(buf, vec![first_byte])?;
        write_bytes(buf, length)?;
        write_bytes(buf, data.value.as_bytes().to_vec())?;

        Ok(())
    }

    fn unpack(buf: &mut BufReader<Cursor<Vec<u8>>>, fb: u8) -> Result<Atom> {
        let length: Result<usize> = match fb {
            ATOM_EXT | ATOM_UTF8_EXT => Ok(2),
            SMALL_ATOM_EXT | SMALL_ATOM_UTF8_EXT => Ok(1),
            _ => Err(anyhow!("Unknown first byte")),
        };

        let kind = match fb {
            ATOM_UTF8_EXT => AtomKind::UTF8,
            SMALL_ATOM_UTF8_EXT => AtomKind::SmallUTF8,
            ATOM_EXT => AtomKind::Legacy,
            SMALL_ATOM_EXT => AtomKind::SmallLegacy,
            _ => unreachable!(),
        };

        let length = read_bytes(buf, length?)?;
        let length = match length.len() {
            1 => u16::from(length[0]),
            2 => u16::from_be_bytes([length[0], length[1]]),
            _ => unreachable!(),
        };

        let value = read_bytes(buf, length.into())?;
        let value = String::from_utf8(value)?;

        Ok(Atom { kind, value })
    }

    fn can_pack(data: &AnyTerm) -> bool {
        match data {
            &AnyTerm::Atom(_) => true,
            _ => false,
        }
    }

    fn can_unpack(first_byte: &u8) -> bool {
        match first_byte {
            &ATOM_UTF8_EXT => true,
            &SMALL_ATOM_UTF8_EXT => true,
            &ATOM_EXT => true,
            &SMALL_ATOM_EXT => true,
            _ => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const VALUE: &str = "bruh";
    const PACKED_ATOM: [u8; 7] = [100, 0, 4, 98, 114, 117, 104];
    const PACKED_ATOM_UTF8: [u8; 7] = [118, 0, 4, 98, 114, 117, 104];
    const PACKED_SMALL_ATOM: [u8; 6] = [115, 4, 98, 114, 117, 104];
    const PACKED_SMALL_ATOM_UTF8: [u8; 6] = [119, 4, 98, 114, 117, 104];

    #[test]
    fn can_pack() {
        let atom = Atom {
            kind: AtomKind::Legacy,
            value: "123".to_string(),
        };
        let string = AnyTerm::String("b".to_string());

        assert_eq!(AtomPacker::can_pack(&AnyTerm::Atom(atom)), true);
        assert_eq!(AtomPacker::can_pack(&string), false);
    }

    #[test]
    fn can_unpack() {
        assert_eq!(AtomPacker::can_unpack(&ATOM_EXT), true);
        assert_eq!(AtomPacker::can_unpack(&ATOM_UTF8_EXT), true);
        assert_eq!(AtomPacker::can_unpack(&SMALL_ATOM_EXT), true);
        assert_eq!(AtomPacker::can_unpack(&SMALL_ATOM_UTF8_EXT), true);
        assert_eq!(AtomPacker::can_unpack(&123), false);
    }

    #[test]
    fn pack_atom() {
        let mut buf = BufWriter::new(Cursor::new(Vec::<u8>::new()));

        let atom = Atom {
            kind: AtomKind::Legacy,
            value: VALUE.to_string(),
        };

        AtomPacker::pack(atom, &mut buf).unwrap();
        let buf = buf.into_inner().unwrap().into_inner();
        assert_eq!(buf, PACKED_ATOM);
    }

    #[test]
    fn unpack_atom() {
        let mut buf = BufReader::new(Cursor::new(PACKED_ATOM.to_vec()));
        let fb = read_bytes(&mut buf, 1).unwrap()[0];
        let atom = AtomPacker::unpack(&mut buf, fb).unwrap();
        assert_eq!(atom.kind, AtomKind::Legacy);
        assert_eq!(atom.value, VALUE);
    }

    #[test]
    fn pack_atom_utf8() {
        let mut buf = BufWriter::new(Cursor::new(Vec::<u8>::new()));

        let atom = Atom {
            kind: AtomKind::UTF8,
            value: VALUE.to_string(),
        };

        AtomPacker::pack(atom, &mut buf).unwrap();
        let buf = buf.into_inner().unwrap().into_inner();
        assert_eq!(buf, PACKED_ATOM_UTF8);
    }

    #[test]
    fn unpack_atom_utf8() {
        let mut buf = BufReader::new(Cursor::new(PACKED_ATOM_UTF8.to_vec()));
        let fb = read_bytes(&mut buf, 1).unwrap()[0];
        let atom = AtomPacker::unpack(&mut buf, fb).unwrap();
        assert_eq!(atom.kind, AtomKind::UTF8);
        assert_eq!(atom.value, VALUE);
    }

    #[test]
    fn pack_small_atom() {
        let mut buf = BufWriter::new(Cursor::new(Vec::<u8>::new()));

        let atom = Atom {
            kind: AtomKind::SmallLegacy,
            value: VALUE.to_string(),
        };

        AtomPacker::pack(atom, &mut buf).unwrap();
        let buf = buf.into_inner().unwrap().into_inner();
        assert_eq!(buf, PACKED_SMALL_ATOM);
    }

    #[test]
    fn unpack_small_atom() {
        let mut buf = BufReader::new(Cursor::new(PACKED_SMALL_ATOM.to_vec()));
        let fb = read_bytes(&mut buf, 1).unwrap()[0];
        let atom = AtomPacker::unpack(&mut buf, fb).unwrap();
        assert_eq!(atom.kind, AtomKind::SmallLegacy);
        assert_eq!(atom.value, VALUE);
    }

    #[test]
    fn pack_small_atom_utf8() {
        let mut buf = BufWriter::new(Cursor::new(Vec::<u8>::new()));

        let atom = Atom {
            kind: AtomKind::SmallUTF8,
            value: VALUE.to_string(),
        };

        AtomPacker::pack(atom, &mut buf).unwrap();
        let buf = buf.into_inner().unwrap().into_inner();
        assert_eq!(buf, PACKED_SMALL_ATOM_UTF8);
    }

    #[test]
    fn unpack_small_atom_utf8() {
        let mut buf = BufReader::new(Cursor::new(PACKED_SMALL_ATOM_UTF8.to_vec()));
        let fb = read_bytes(&mut buf, 1).unwrap()[0];
        let atom = AtomPacker::unpack(&mut buf, fb).unwrap();
        assert_eq!(atom.kind, AtomKind::SmallUTF8);
        assert_eq!(atom.value, VALUE);
    }
}
