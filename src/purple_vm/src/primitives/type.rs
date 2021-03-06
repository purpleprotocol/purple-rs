/*
  Copyright (C) 2018-2020 The Purple Core Developers.
  This file is part of the Purple Core Library.

  The Purple Core Library is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  The Purple Core Library is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with the Purple Core Library. If not, see <http://www.gnu.org/licenses/>.
*/

use crate::instruction_set::Instruction;

use byteorder::{BigEndian, ReadBytesExt};
use std::io::Cursor;

#[derive(Clone, Debug, Copy, PartialEq)]
pub enum VmType {
    I32,
    I64,
    F32,
    F64,
    i32Array2,
    i32Array4,
    i32Array8,
    i32Array16,
    i32Array32,
    i32Array64,
    i32Array128,
    i32Array256,
    i64Array2,
    i64Array4,
    i64Array8,
    i64Array16,
    i64Array32,
    i64Array64,
    i64Array128,
    i64Array256,
    f32Array2,
    f32Array4,
    f32Array8,
    f32Array16,
    f32Array32,
    f32Array64,
    f32Array128,
    f32Array256,
    f64Array2,
    f64Array4,
    f64Array8,
    f64Array16,
    f64Array32,
    f64Array64,
    f64Array128,
    f64Array256,
}

impl VmType {
    pub fn from_op(op: u8) -> Option<VmType> {
        match Instruction::from_repr(op) {
            Some(Instruction::i32Const) => Some(VmType::I32),
            Some(Instruction::i64Const) => Some(VmType::I64),
            Some(Instruction::f32Const) => Some(VmType::F32),
            Some(Instruction::f64Const) => Some(VmType::F64),
            Some(Instruction::i32Array2) => Some(VmType::i32Array2),
            Some(Instruction::i32Array4) => Some(VmType::i32Array4),
            Some(Instruction::i32Array8) => Some(VmType::i32Array8),
            Some(Instruction::i32Array16) => Some(VmType::i32Array16),
            Some(Instruction::i32Array32) => Some(VmType::i32Array32),
            Some(Instruction::i32Array64) => Some(VmType::i32Array64),
            Some(Instruction::i32Array128) => Some(VmType::i32Array128),
            Some(Instruction::i32Array256) => Some(VmType::i32Array256),
            Some(Instruction::i64Array2) => Some(VmType::i64Array2),
            Some(Instruction::i64Array4) => Some(VmType::i64Array4),
            Some(Instruction::i64Array8) => Some(VmType::i64Array8),
            Some(Instruction::i64Array16) => Some(VmType::i64Array16),
            Some(Instruction::i64Array32) => Some(VmType::i64Array32),
            Some(Instruction::i64Array64) => Some(VmType::i64Array64),
            Some(Instruction::i64Array128) => Some(VmType::i64Array128),
            Some(Instruction::i64Array256) => Some(VmType::i64Array256),
            Some(Instruction::f32Array2) => Some(VmType::f32Array2),
            Some(Instruction::f32Array4) => Some(VmType::f32Array4),
            Some(Instruction::f32Array8) => Some(VmType::f32Array8),
            Some(Instruction::f32Array16) => Some(VmType::f32Array16),
            Some(Instruction::f32Array32) => Some(VmType::f32Array32),
            Some(Instruction::f32Array64) => Some(VmType::f32Array64),
            Some(Instruction::f32Array128) => Some(VmType::f32Array128),
            Some(Instruction::f32Array256) => Some(VmType::f32Array256),
            Some(Instruction::f64Array2) => Some(VmType::f64Array2),
            Some(Instruction::f64Array4) => Some(VmType::f64Array4),
            Some(Instruction::f64Array8) => Some(VmType::f64Array8),
            Some(Instruction::f64Array16) => Some(VmType::f64Array16),
            Some(Instruction::f64Array32) => Some(VmType::f64Array32),
            Some(Instruction::f64Array64) => Some(VmType::f64Array64),
            Some(Instruction::f64Array128) => Some(VmType::f64Array128),
            Some(Instruction::f64Array256) => Some(VmType::f64Array256),
            _ => None,
        }
    }

    pub fn validate_structure(&self, buf: &[u8]) -> bool {
        if buf.len() != self.byte_size() {
            return false;
        }

        match *self {
            VmType::I32 => match decode_be_i32!(buf) {
                Ok(_) => true,
                _ => false,
            },
            VmType::I64 => match decode_be_i64!(buf) {
                Ok(_) => true,
                _ => false,
            },
            VmType::F32 => match decode_be_f32!(buf) {
                Ok(_) => true,
                _ => false,
            },
            VmType::F64 => match decode_be_f64!(buf) {
                Ok(_) => true,
                _ => false,
            },
            VmType::i32Array2 => {
                let mut iterator = buf.chunks_exact(4);
                for _ in 0..2 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_i32::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::i32Array4 => {
                let mut iterator = buf.chunks_exact(4);
                for _ in 0..4 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_i32::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::i32Array8 => {
                let mut iterator = buf.chunks_exact(4);
                for _ in 0..8 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_i32::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::i32Array16 => {
                let mut iterator = buf.chunks_exact(4);
                for _ in 0..16 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_i32::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::i32Array32 => {
                let mut iterator = buf.chunks_exact(4);
                for _ in 0..32 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_i32::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::i32Array64 => {
                let mut iterator = buf.chunks_exact(4);
                for _ in 0..64 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_i32::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::i32Array128 => {
                let mut iterator = buf.chunks_exact(4);
                for _ in 0..128 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_i32::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::i32Array256 => {
                let mut iterator = buf.chunks_exact(4);
                for _ in 0..256 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_i32::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::i64Array2 => {
                let mut iterator = buf.chunks_exact(8);
                for _ in 0..2 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_i64::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::i64Array4 => {
                let mut iterator = buf.chunks_exact(8);
                for _ in 0..4 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_i64::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::i64Array8 => {
                let mut iterator = buf.chunks_exact(8);
                for _ in 0..8 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_i64::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::i64Array16 => {
                let mut iterator = buf.chunks_exact(8);
                for _ in 0..16 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_i64::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::i64Array32 => {
                let mut iterator = buf.chunks_exact(8);
                for _ in 0..32 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_i64::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::i64Array64 => {
                let mut iterator = buf.chunks_exact(8);
                for _ in 0..64 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_i64::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::i64Array128 => {
                let mut iterator = buf.chunks_exact(8);
                for _ in 0..128 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_i64::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::i64Array256 => {
                let mut iterator = buf.chunks_exact(8);
                for _ in 0..256 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_i64::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::f32Array2 => {
                let mut iterator = buf.chunks_exact(4);
                for _ in 0..2 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_f32::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::f32Array4 => {
                let mut iterator = buf.chunks_exact(4);
                for _ in 0..4 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_f32::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::f32Array8 => {
                let mut iterator = buf.chunks_exact(4);
                for _ in 0..8 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_f32::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::f32Array16 => {
                let mut iterator = buf.chunks_exact(4);
                for _ in 0..16 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_f32::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::f32Array32 => {
                let mut iterator = buf.chunks_exact(4);
                for _ in 0..32 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_f32::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::f32Array64 => {
                let mut iterator = buf.chunks_exact(4);
                for _ in 0..64 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_f32::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::f32Array128 => {
                let mut iterator = buf.chunks_exact(4);
                for _ in 0..128 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_f32::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::f32Array256 => {
                let mut iterator = buf.chunks_exact(4);
                for _ in 0..256 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_f32::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::f64Array2 => {
                let mut iterator = buf.chunks_exact(8);
                for _ in 0..2 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_f64::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::f64Array4 => {
                let mut iterator = buf.chunks_exact(8);
                for _ in 0..4 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_f64::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::f64Array8 => {
                let mut iterator = buf.chunks_exact(8);
                for _ in 0..8 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_f64::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::f64Array16 => {
                let mut iterator = buf.chunks_exact(8);
                for _ in 0..16 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_f64::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::f64Array32 => {
                let mut iterator = buf.chunks_exact(8);
                for _ in 0..32 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_f64::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::f64Array64 => {
                let mut iterator = buf.chunks_exact(8);
                for _ in 0..64 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_f64::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::f64Array128 => {
                let mut iterator = buf.chunks_exact(8);
                for _ in 0..128 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_f64::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
            VmType::f64Array256 => {
                let mut iterator = buf.chunks_exact(8);
                for _ in 0..256 {
                    let mut cursor = Cursor::new(iterator.next().unwrap());
                    match cursor.read_f64::<BigEndian>() {
                        Ok(_) => (),
                        Err(_) => return false,
                    }
                }

                true
            }
        }
    }

    pub fn array_accepts(&self) -> Option<VmType> {
        match *self {
            VmType::i32Array2
            | VmType::i32Array4
            | VmType::i32Array8
            | VmType::i32Array16
            | VmType::i32Array32
            | VmType::i32Array64
            | VmType::i32Array128
            | VmType::i32Array256 => Some(VmType::I32),
            VmType::i64Array2
            | VmType::i64Array4
            | VmType::i64Array8
            | VmType::i64Array16
            | VmType::i64Array32
            | VmType::i64Array64
            | VmType::i64Array128
            | VmType::i64Array256 => Some(VmType::I64),
            VmType::f32Array2
            | VmType::f32Array4
            | VmType::f32Array8
            | VmType::f32Array16
            | VmType::f32Array32
            | VmType::f32Array64
            | VmType::f32Array128
            | VmType::f32Array256 => Some(VmType::F32),
            VmType::f64Array2
            | VmType::f64Array4
            | VmType::f64Array8
            | VmType::f64Array16
            | VmType::f64Array32
            | VmType::f64Array64
            | VmType::f64Array128
            | VmType::f64Array256 => Some(VmType::F64),
            _ => None,
        }
    }

    /// Returns the byte size of the type.
    pub fn byte_size(&self) -> usize {
        match *self {
            VmType::I32 => 4,
            VmType::I64 => 8,
            VmType::F32 => 4,
            VmType::F64 => 8,
            VmType::i32Array2 => 8,
            VmType::i32Array4 => 16,
            VmType::i32Array8 => 32,
            VmType::i32Array16 => 64,
            VmType::i32Array32 => 128,
            VmType::i32Array64 => 256,
            VmType::i32Array128 => 512,
            VmType::i32Array256 => 1024,
            VmType::i64Array2 => 16,
            VmType::i64Array4 => 32,
            VmType::i64Array8 => 64,
            VmType::i64Array16 => 128,
            VmType::i64Array32 => 256,
            VmType::i64Array64 => 512,
            VmType::i64Array128 => 1024,
            VmType::i64Array256 => 2048,
            VmType::f32Array2 => 8,
            VmType::f32Array4 => 16,
            VmType::f32Array8 => 32,
            VmType::f32Array16 => 64,
            VmType::f32Array32 => 128,
            VmType::f32Array64 => 256,
            VmType::f32Array128 => 512,
            VmType::f32Array256 => 1024,
            VmType::f64Array2 => 16,
            VmType::f64Array4 => 32,
            VmType::f64Array8 => 64,
            VmType::f64Array16 => 128,
            VmType::f64Array32 => 256,
            VmType::f64Array64 => 512,
            VmType::f64Array128 => 1024,
            VmType::f64Array256 => 2048,
        }
    }

    pub fn is_float(&self) -> bool {
        match *self {
            VmType::F32
            | VmType::F64
            | VmType::f32Array2
            | VmType::f32Array4
            | VmType::f32Array8
            | VmType::f32Array16
            | VmType::f32Array32
            | VmType::f32Array64
            | VmType::f32Array128
            | VmType::f32Array256
            | VmType::f64Array2
            | VmType::f64Array4
            | VmType::f64Array8
            | VmType::f64Array16
            | VmType::f64Array32
            | VmType::f64Array64
            | VmType::f64Array128
            | VmType::f64Array256 => return true,
            _ => return false,
        }
    }

    pub fn is_int(&self) -> bool {
        return !self.is_float();
    }

    pub fn is_i32(&self) -> bool {
        match *self {
            VmType::I32
            | VmType::i32Array2
            | VmType::i32Array4
            | VmType::i32Array8
            | VmType::i32Array16
            | VmType::i32Array32
            | VmType::i32Array64
            | VmType::i32Array128
            | VmType::i32Array256 => true,
            _ => false,
        }
    }

    pub fn is_i64(&self) -> bool {
        match *self {
            VmType::I64
            | VmType::i64Array2
            | VmType::i64Array4
            | VmType::i64Array8
            | VmType::i64Array16
            | VmType::i64Array32
            | VmType::i64Array64
            | VmType::i64Array128
            | VmType::i64Array256 => true,
            _ => false,
        }
    }

    pub fn is_f32(&self) -> bool {
        match *self {
            VmType::F32
            | VmType::f32Array2
            | VmType::f32Array4
            | VmType::f32Array8
            | VmType::f32Array16
            | VmType::f32Array32
            | VmType::f32Array64
            | VmType::f32Array128
            | VmType::f32Array256 => true,
            _ => false,
        }
    }

    pub fn is_f64(&self) -> bool {
        match *self {
            VmType::F64
            | VmType::f64Array2
            | VmType::f64Array4
            | VmType::f64Array8
            | VmType::f64Array16
            | VmType::f64Array32
            | VmType::f64Array64
            | VmType::f64Array128
            | VmType::f64Array256 => true,
            _ => false,
        }
    }

    pub fn is_array(&self) -> bool {
        match *self {
            VmType::I32 | VmType::I64 | VmType::F32 | VmType::F64 => false,
            _ => true,
        }
    }
}
