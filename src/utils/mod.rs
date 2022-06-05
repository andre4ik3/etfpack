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
use std::io::{BufReader, BufWriter, Cursor, Read, Write};

/// Reads num bytes from buf, returning them as a Vec<u8>.
pub fn read_bytes(buf: &mut BufReader<Cursor<Vec<u8>>>, num: usize) -> Result<Vec<u8>> {
    let mut dest: Vec<u8> = vec![0; num];
    let read = buf.read(&mut dest)?;
    if read != num {
        return Err(anyhow!("Failed reading bytes"));
    }
    Ok(dest)
}

/// Writes some bytes.
pub fn write_bytes(buf: &mut BufWriter<Cursor<Vec<u8>>>, bytes: Vec<u8>) -> Result<()> {
    buf.write(&bytes)?;
    Ok(())
}

/// Converts a slice of u8 bytes with a null somewhere in the middle to a string.
/// Credit: https://stackoverflow.com/questions/42066381
pub fn str_from_u8_nul_utf8(utf8_src: &[u8]) -> Result<&str, std::str::Utf8Error> {
    let nul_range_end = utf8_src
        .iter()
        .position(|&c| c == b'\0')
        .unwrap_or(utf8_src.len()); // default to length if no `\0` present
    ::std::str::from_utf8(&utf8_src[0..nul_range_end])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read() {
        let buf: Vec<u8> = vec![0, 1, 2, 3, 4, 5, 6, 7];
        let mut buf = BufReader::new(Cursor::new(buf));
        assert_eq!(read_bytes(&mut buf, 3).unwrap(), vec![0, 1, 2]);
        assert_eq!(read_bytes(&mut buf, 3).unwrap(), vec![3, 4, 5]);
        assert_eq!(read_bytes(&mut buf, 2).unwrap(), vec![6, 7]);
        assert_eq!(read_bytes(&mut buf, 1).is_err(), true);
    }

    #[test]
    fn write() {
        let mut buf = BufWriter::new(Cursor::new(Vec::<u8>::new()));

        write_bytes(&mut buf, vec![0, 1, 2]).unwrap();
        write_bytes(&mut buf, vec![3, 4, 5]).unwrap();
        write_bytes(&mut buf, vec![6, 7]).unwrap();

        assert_eq!(
            buf.into_inner().unwrap().into_inner(),
            vec![0, 1, 2, 3, 4, 5, 6, 7]
        );
    }
}
