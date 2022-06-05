// //! Copyright 2022 andre4ik3
// //!
// //! Licensed under the Apache License, Version 2.0 (the "License");
// //! you may not use this file except in compliance with the License.
// //! You may obtain a copy of the License at
// //!
// //!     http://www.apache.org/licenses/LICENSE-2.0
// //!
// //! Unless required by applicable law or agreed to in writing, software
// //! distributed under the License is distributed on an "AS IS" BASIS,
// //! WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// //! See the License for the specific language governing permissions and
// //! limitations under the License.

// use super::*;
// use crate::utils::*;

// use anyhow::*;

// const PORT_EXT: u8 = 102;
// const NEW_PORT_EXT: u8 = 89;
// const V4_PORT_EXT: u8 = 120;

// pub struct PortPacker;
// impl Term<Port> for PortPacker {
//     /// Always packs as a V4 port.
//     fn pack(data: Port, buf: &mut BufWriter<Cursor<Vec<u8>>>) -> Result<()> {
//         todo!()
//     }

//     fn unpack(buf: &mut BufReader<Cursor<Vec<u8>>>, fb: u8) -> Result<Port> {
//         let node_fb = read_bytes(buf, 1)?[0];

//         if !AtomPacker::can_unpack(&node_fb) {
//             return Err(anyhow!("Invalid node atom."));
//         }

//         let node = AtomPacker::unpack(buf, node_fb)?;

//         let id_len = match fb {};

//         todo!()
//     }

//     fn can_pack(data: &AnyTerm) -> bool {
//         match data {
//             &AnyTerm::SmallInt(_) => true,
//             _ => false,
//         }
//     }

//     fn can_unpack(first_byte: &u8) -> bool {
//         match first_byte {
//             &PORT_EXT => true,
//             &NEW_PORT_EXT => true,
//             &V4_PORT_EXT => true,
//             _ => false,
//         }
//     }
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use crate::terms::AnyTerm;

//     const VALUE: u8 = 248;
//     const PACKED_INTEGER: [u8; 2] = [97, 248];

//     #[test]
//     fn can_pack() {
//         assert_eq!(SmallIntPacker::can_pack(&AnyTerm::Integer(123)), false);
//         assert_eq!(SmallIntPacker::can_pack(&AnyTerm::SmallInt(123)), true);
//     }

//     #[test]
//     fn can_unpack() {
//         assert_eq!(SmallIntPacker::can_unpack(&SMALL_INTEGER_EXT), true);
//         assert_eq!(SmallIntPacker::can_unpack(&123), false);
//     }

//     #[test]
//     fn pack() {
//         let mut buf = BufWriter::new(Cursor::new(Vec::<u8>::new()));
//         SmallIntPacker::pack(VALUE, &mut buf).unwrap();
//         let buf = buf.into_inner().unwrap().into_inner();
//         assert_eq!(buf, PACKED_INTEGER);
//     }

//     #[test]
//     fn unpack() {
//         let mut buf = BufReader::new(Cursor::new(PACKED_INTEGER.to_vec()));
//         read_bytes(&mut buf, 1).unwrap();
//         assert_eq!(
//             SmallIntPacker::unpack(&mut buf, SMALL_INTEGER_EXT).unwrap(),
//             VALUE
//         );
//     }
// }
