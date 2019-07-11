/*
  Copyright (C) 2018-2019 The Purple Core Developers.
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

use std::cmp::{Ordering, PartialOrd};
use std::fmt;
use std::ops::{Add, Div, Mul, Rem, Sub};
use std::{f32, f64};
use VmError;

#[derive(Clone, Copy)]
pub enum VmValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    i32Array2([i32; 2]),
    i32Array4([i32; 4]),
    i32Array8([i32; 8]),
    i32Array16([i32; 16]),
    i32Array32([i32; 32]),
    i32Array64([i32; 64]),
    i32Array128([i32; 128]),
    i32Array256([i32; 256]),
    i64Array2([i64; 2]),
    i64Array4([i64; 4]),
    i64Array8([i64; 8]),
    i64Array16([i64; 16]),
    i64Array32([i64; 32]),
    i64Array64([i64; 64]),
    i64Array128([i64; 128]),
    i64Array256([i64; 256]),
    f32Array2([f32; 2]),
    f32Array4([f32; 4]),
    f32Array8([f32; 8]),
    f32Array16([f32; 16]),
    f32Array32([f32; 32]),
    f32Array64([f32; 64]),
    f32Array128([f32; 128]),
    f32Array256([f32; 256]),
    f64Array2([f64; 2]),
    f64Array4([f64; 4]),
    f64Array8([f64; 8]),
    f64Array16([f64; 16]),
    f64Array32([f64; 32]),
    f64Array64([f64; 64]),
    f64Array128([f64; 128]),
    f64Array256([f64; 256]),
}

impl VmValue {
    /// Returns the byte size of the inner value.
    pub fn byte_size(&self) -> usize {
        match *self {
            VmValue::I32(_) => 4,
            VmValue::I64(_) => 8,
            VmValue::F32(_) => 4,
            VmValue::F64(_) => 8,
            VmValue::i32Array2(_) => 8,
            VmValue::i32Array4(_) => 16,
            VmValue::i32Array8(_) => 32,
            VmValue::i64Array2(_) => 16,
            VmValue::i64Array4(_) => 32,
            VmValue::i64Array8(_) => 64,
            VmValue::f32Array2(_) => 8,
            VmValue::f32Array4(_) => 16,
            VmValue::f32Array8(_) => 32,
            VmValue::f64Array2(_) => 8,
            VmValue::f64Array4(_) => 16,
            VmValue::f64Array8(_) => 32,
            _ => panic!(),
        }
    }

    pub fn is_positive(&self) -> bool {
        match *self {
            VmValue::I32(val) => val >= 0,
            VmValue::I64(val) => val >= 0,
            VmValue::F32(val) => val >= 0.0,
            VmValue::F64(val) => val >= 0.0,
            VmValue::i32Array2(val) => val.iter().all(|&v| v >= 0),
            VmValue::i32Array4(val) => val.iter().all(|&v| v >= 0),
            VmValue::i32Array8(val) => val.iter().all(|&v| v >= 0),
            VmValue::i64Array2(val) => val.iter().all(|&v| v >= 0),
            VmValue::i64Array4(val) => val.iter().all(|&v| v >= 0),
            VmValue::i64Array8(val) => val.iter().all(|&v| v >= 0),
            VmValue::f32Array2(val) => val.iter().all(|&v| v >= 0.0),
            VmValue::f32Array4(val) => val.iter().all(|&v| v >= 0.0),
            VmValue::f32Array8(val) => val.iter().all(|&v| v >= 0.0),
            VmValue::f64Array2(val) => val.iter().all(|&v| v >= 0.0),
            VmValue::f64Array4(val) => val.iter().all(|&v| v >= 0.0),
            VmValue::f64Array8(val) => val.iter().all(|&v| v >= 0.0),
            _ => panic!(),
        }
    }

    fn check_f32_infinite(val: f32) -> Option<f32> {
        if val.is_infinite() {
            None
        } else {
            Some(val)
        }
    }

    fn check_f64_infinite(val: f64) -> Option<f64> {
        if val.is_infinite() {
            None
        } else {
            Some(val)
        }
    }

    fn sum_f32(val1: &f32, val2: &f32) -> Option<f32> {
        VmValue::check_f32_infinite(val1 + val2)
    }

    fn sum_f64(val1: &f64, val2: &f64) -> Option<f64> {
        VmValue::check_f64_infinite(val1 + val2)
    }

    fn sub_f32(val1: &f32, val2: &f32) -> Option<f32> {
        VmValue::check_f32_infinite(val1 - val2)
    }

    fn sub_f64(val1: &f64, val2: &f64) -> Option<f64> {
        VmValue::check_f64_infinite(val1 - val2)
    }

    fn mul_f32(val1: &f32, val2: &f32) -> Option<f32> {
        VmValue::check_f32_infinite(val1 * val2)
    }

    fn mul_f64(val1: &f64, val2: &f64) -> Option<f64> {
        VmValue::check_f64_infinite(val1 * val2)
    }

    fn div_f32(val1: &f32, val2: &f32) -> Option<f32> {
        if *val2 == 0.0 {
            panic!("Attempted to divide by zero!")
        }
        VmValue::check_f32_infinite(val1 / val2)
    }

    fn div_f64(val1: &f64, val2: &f64) -> Option<f64> {
        if *val2 == 0.0 {
            panic!("Attempted to divide by zero!")
        }
        VmValue::check_f64_infinite(val1 / val2)
    }

    fn rem_f32(val1: &f32, val2: &f32) -> Option<f32> {
        if *val2 == 0.0 {
            panic!("Attempted to divide by zero!")
        }
        VmValue::check_f32_infinite(val1 % val2)
    }

    fn rem_f64(val1: &f64, val2: &f64) -> Option<f64> {
        if *val2 == 0.0 {
            panic!("Attempted to divide by zero!")
        }
        VmValue::check_f64_infinite(val1 % val2)
    }
}

impl PartialEq for VmValue {
    fn eq(&self, other: &VmValue) -> bool {
        match (*self, *other) {
            (VmValue::I32(val1), VmValue::I32(val2)) => val1 == val2,
            (VmValue::I64(val1), VmValue::I64(val2)) => val1 == val2,
            (VmValue::F32(val1), VmValue::F32(val2)) => val1 == val2,
            (VmValue::F64(val1), VmValue::F64(val2)) => val1 == val2,
            (VmValue::i32Array2(val1), VmValue::i32Array2(val2)) => val1 == val2,
            (VmValue::i32Array4(val1), VmValue::i32Array4(val2)) => val1 == val2,
            (VmValue::i32Array8(val1), VmValue::i32Array8(val2)) => val1 == val2,
            (VmValue::i64Array2(val1), VmValue::i64Array2(val2)) => val1 == val2,
            (VmValue::i64Array4(val1), VmValue::i64Array4(val2)) => val1 == val2,
            (VmValue::i64Array8(val1), VmValue::i64Array8(val2)) => val1 == val2,
            (VmValue::f32Array2(val1), VmValue::f32Array2(val2)) => val1 == val2,
            (VmValue::f32Array4(val1), VmValue::f32Array4(val2)) => val1 == val2,
            (VmValue::f32Array8(val1), VmValue::f32Array8(val2)) => val1 == val2,
            (VmValue::f64Array2(val1), VmValue::f64Array2(val2)) => val1 == val2,
            (VmValue::f64Array4(val1), VmValue::f64Array4(val2)) => val1 == val2,
            (VmValue::f64Array8(val1), VmValue::f64Array8(val2)) => val1 == val2,
            (_, _) => panic!("Cannot perform equality between different variants!"),
        }
    }
}

impl PartialOrd for VmValue {
    fn partial_cmp(&self, other: &VmValue) -> Option<Ordering> {
        match (self, other) {
            (VmValue::I32(val1), VmValue::I32(val2)) => {
                if val1 < val2 {
                    Some(Ordering::Less)
                } else if val1 > val2 {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (VmValue::I64(val1), VmValue::I64(val2)) => {
                if val1 < val2 {
                    Some(Ordering::Less)
                } else if val1 > val2 {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (VmValue::F32(val1), VmValue::F32(val2)) => {
                if val1 < val2 {
                    Some(Ordering::Less)
                } else if val1 > val2 {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (VmValue::F64(val1), VmValue::F64(val2)) => {
                if val1 < val2 {
                    Some(Ordering::Less)
                } else if val1 > val2 {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (VmValue::i32Array2(val1), VmValue::i32Array2(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::i32Array4(val1), VmValue::i32Array4(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::i32Array8(val1), VmValue::i32Array8(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::i64Array2(val1), VmValue::i64Array2(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::i64Array4(val1), VmValue::i64Array4(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::i64Array8(val1), VmValue::i64Array8(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::f32Array2(val1), VmValue::f32Array2(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::f32Array4(val1), VmValue::f32Array4(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::f32Array8(val1), VmValue::f32Array8(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::f64Array2(val1), VmValue::f64Array2(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::f64Array4(val1), VmValue::f64Array4(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::f64Array8(val1), VmValue::f64Array8(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (_, _) => panic!("Cannot perform compare between different variants!"),
        }
    }
}

impl Add for VmValue {
    type Output = Result<VmValue, VmError>;

    // TODO: Possibly use native SIMD for arrays, but benchmark first
    fn add(self, other: VmValue) -> Result<VmValue, VmError> {
        match (self, other) {
            (VmValue::I32(val1), VmValue::I32(val2)) => match val1.checked_add(val2) {
                Some(result) => Ok(VmValue::I32(result)),
                None => Err(VmError::Overflow),
            },
            (VmValue::I64(val1), VmValue::I64(val2)) => match val1.checked_add(val2) {
                Some(result) => Ok(VmValue::I64(result)),
                None => Err(VmError::Overflow),
            },
            (VmValue::F32(val1), VmValue::F32(val2)) => match VmValue::sum_f32(&val1, &val2) {
                Some(result) => Ok(VmValue::F32(result)),
                None => Err(VmError::Infinity),
            },
            (VmValue::F64(val1), VmValue::F64(val2)) => match VmValue::sum_f64(&val1, &val2) {
                Some(result) => Ok(VmValue::F64(result)),
                None => Err(VmError::Infinity),
            },
            (VmValue::i32Array2(val1), VmValue::i32Array2(val2)) => {
                let mut result: [i32; 2] = [0; 2];

                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| a.checked_add(*b).ok_or(VmError::Overflow));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i32Array2(result))
            }
            (VmValue::i32Array4(val1), VmValue::i32Array4(val2)) => {
                let mut result: [i32; 4] = [0; 4];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| a.checked_add(*b).ok_or(VmError::Overflow));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i32Array4(result))
            }
            (VmValue::i32Array8(val1), VmValue::i32Array8(val2)) => {
                let mut result: [i32; 8] = [0; 8];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| a.checked_add(*b).ok_or(VmError::Overflow));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i32Array8(result))
            }
            (VmValue::i64Array2(val1), VmValue::i64Array2(val2)) => {
                let mut result: [i64; 2] = [0; 2];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| a.checked_add(*b).ok_or(VmError::Overflow));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i64Array2(result))
            }
            (VmValue::i64Array4(val1), VmValue::i64Array4(val2)) => {
                let mut result: [i64; 4] = [0; 4];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| a.checked_add(*b).ok_or(VmError::Overflow));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i64Array4(result))
            }
            (VmValue::i64Array8(val1), VmValue::i64Array8(val2)) => {
                let mut result: [i64; 8] = [0; 8];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| a.checked_add(*b).ok_or(VmError::Overflow));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i64Array8(result))
            }
            (VmValue::f32Array2(val1), VmValue::f32Array2(val2)) => {
                let mut result: [f32; 2] = [0.0; 2];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| VmValue::sum_f32(a, b).ok_or(VmError::Infinity));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f32Array2(result))
            }
            (VmValue::f32Array4(val1), VmValue::f32Array4(val2)) => {
                let mut result: [f32; 4] = [0.0; 4];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| VmValue::sum_f32(a, b).ok_or(VmError::Infinity));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f32Array4(result))
            }
            (VmValue::f32Array8(val1), VmValue::f32Array8(val2)) => {
                let mut result: [f32; 8] = [0.0; 8];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| VmValue::sum_f32(a, b).ok_or(VmError::Infinity));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f32Array8(result))
            }
            (VmValue::f64Array2(val1), VmValue::f64Array2(val2)) => {
                let mut result: [f64; 2] = [0.0; 2];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| VmValue::sum_f64(a, b).ok_or(VmError::Infinity));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f64Array2(result))
            }
            (VmValue::f64Array4(val1), VmValue::f64Array4(val2)) => {
                let mut result: [f64; 4] = [0.0; 4];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| VmValue::sum_f64(a, b).ok_or(VmError::Infinity));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f64Array4(result))
            }
            (VmValue::f64Array8(val1), VmValue::f64Array8(val2)) => {
                let mut result: [f64; 8] = [0.0; 8];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| VmValue::sum_f64(a, b).ok_or(VmError::Infinity));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f64Array8(result))
            }
            (_, _) => panic!("Cannot perform addition between different variants!"),
        }
    }
}

impl Sub for VmValue {
    type Output = Result<VmValue, VmError>;

    fn sub(self, other: VmValue) -> Result<VmValue, VmError> {
        match (self, other) {
            (VmValue::I32(val1), VmValue::I32(val2)) => match val1.checked_sub(val2) {
                Some(result) => Ok(VmValue::I32(result)),
                None => Err(VmError::Overflow),
            },
            (VmValue::I64(val1), VmValue::I64(val2)) => match val1.checked_sub(val2) {
                Some(result) => Ok(VmValue::I64(result)),
                None => Err(VmError::Overflow),
            },
            (VmValue::F32(val1), VmValue::F32(val2)) => match VmValue::sub_f32(&val1, &val2) {
                Some(result) => Ok(VmValue::F32(result)),
                None => Err(VmError::Infinity),
            },
            (VmValue::F64(val1), VmValue::F64(val2)) => match VmValue::sub_f64(&val1, &val2) {
                Some(result) => Ok(VmValue::F64(result)),
                None => Err(VmError::Infinity),
            },
            (VmValue::i32Array2(val1), VmValue::i32Array2(val2)) => {
                let mut result: [i32; 2] = [0; 2];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| a.checked_sub(*b).ok_or(VmError::Overflow));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i32Array2(result))
            }
            (VmValue::i32Array4(val1), VmValue::i32Array4(val2)) => {
                let mut result: [i32; 4] = [0; 4];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| a.checked_sub(*b).ok_or(VmError::Overflow));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i32Array4(result))
            }
            (VmValue::i32Array8(val1), VmValue::i32Array8(val2)) => {
                let mut result: [i32; 8] = [0; 8];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| a.checked_sub(*b).ok_or(VmError::Overflow));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i32Array8(result))
            }
            (VmValue::i64Array2(val1), VmValue::i64Array2(val2)) => {
                let mut result: [i64; 2] = [0; 2];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| a.checked_sub(*b).ok_or(VmError::Overflow));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i64Array2(result))
            }
            (VmValue::i64Array4(val1), VmValue::i64Array4(val2)) => {
                let mut result: [i64; 4] = [0; 4];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| a.checked_sub(*b).ok_or(VmError::Overflow));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i64Array4(result))
            }
            (VmValue::i64Array8(val1), VmValue::i64Array8(val2)) => {
                let mut result: [i64; 8] = [0; 8];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| a.checked_sub(*b).ok_or(VmError::Overflow));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i64Array8(result))
            }
            (VmValue::f32Array2(val1), VmValue::f32Array2(val2)) => {
                let mut result: [f32; 2] = [0.0; 2];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| VmValue::sub_f32(a, b).ok_or(VmError::Infinity));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f32Array2(result))
            }
            (VmValue::f32Array4(val1), VmValue::f32Array4(val2)) => {
                let mut result: [f32; 4] = [0.0; 4];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| VmValue::sub_f32(a, b).ok_or(VmError::Infinity));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f32Array4(result))
            }
            (VmValue::f32Array8(val1), VmValue::f32Array8(val2)) => {
                let mut result: [f32; 8] = [0.0; 8];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| VmValue::sub_f32(a, b).ok_or(VmError::Infinity));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f32Array8(result))
            }
            (VmValue::f64Array2(val1), VmValue::f64Array2(val2)) => {
                let mut result: [f64; 2] = [0.0; 2];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| VmValue::sub_f64(a, b).ok_or(VmError::Infinity));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f64Array2(result))
            }
            (VmValue::f64Array4(val1), VmValue::f64Array4(val2)) => {
                let mut result: [f64; 4] = [0.0; 4];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| VmValue::sub_f64(a, b).ok_or(VmError::Infinity));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f64Array4(result))
            }
            (VmValue::f64Array8(val1), VmValue::f64Array8(val2)) => {
                let mut result: [f64; 8] = [0.0; 8];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| VmValue::sub_f64(a, b).ok_or(VmError::Infinity));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f64Array8(result))
            }
            (_, _) => panic!("Cannot perform substraction between different variants!"),
        }
    }
}

impl Mul for VmValue {
    type Output = Result<VmValue, VmError>;

    fn mul(self, other: VmValue) -> Result<VmValue, VmError> {
        match (self, other) {
            (VmValue::I32(val1), VmValue::I32(val2)) => match val1.checked_mul(val2) {
                Some(result) => Ok(VmValue::I32(result)),
                None => Err(VmError::Overflow),
            },
            (VmValue::I64(val1), VmValue::I64(val2)) => match val1.checked_mul(val2) {
                Some(result) => Ok(VmValue::I64(result)),
                None => Err(VmError::Overflow),
            },
            (VmValue::F32(val1), VmValue::F32(val2)) => match VmValue::mul_f32(&val1, &val2) {
                Some(result) => Ok(VmValue::F32(result)),
                None => Err(VmError::Infinity),
            },
            (VmValue::F64(val1), VmValue::F64(val2)) => match VmValue::mul_f64(&val1, &val2) {
                Some(result) => Ok(VmValue::F64(result)),
                None => Err(VmError::Infinity),
            },
            (VmValue::i32Array2(val1), VmValue::i32Array2(val2)) => {
                let mut result: [i32; 2] = [0; 2];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| a.checked_mul(*b).ok_or(VmError::Overflow));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i32Array2(result))
            }
            (VmValue::i32Array4(val1), VmValue::i32Array4(val2)) => {
                let mut result: [i32; 4] = [0; 4];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| a.checked_mul(*b).ok_or(VmError::Overflow));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i32Array4(result))
            }
            (VmValue::i32Array8(val1), VmValue::i32Array8(val2)) => {
                let mut result: [i32; 8] = [0; 8];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| a.checked_mul(*b).ok_or(VmError::Overflow));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i32Array8(result))
            }
            (VmValue::i64Array2(val1), VmValue::i64Array2(val2)) => {
                let mut result: [i64; 2] = [0; 2];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| a.checked_mul(*b).ok_or(VmError::Overflow));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i64Array2(result))
            }
            (VmValue::i64Array4(val1), VmValue::i64Array4(val2)) => {
                let mut result: [i64; 4] = [0; 4];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| a.checked_mul(*b).ok_or(VmError::Overflow));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i64Array4(result))
            }
            (VmValue::i64Array8(val1), VmValue::i64Array8(val2)) => {
                let mut result: [i64; 8] = [0; 8];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| a.checked_mul(*b).ok_or(VmError::Overflow));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i64Array8(result))
            }
            (VmValue::f32Array2(val1), VmValue::f32Array2(val2)) => {
                let mut result: [f32; 2] = [0.0; 2];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| VmValue::mul_f32(a, b).ok_or(VmError::Infinity));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f32Array2(result))
            }
            (VmValue::f32Array4(val1), VmValue::f32Array4(val2)) => {
                let mut result: [f32; 4] = [0.0; 4];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| VmValue::mul_f32(a, b).ok_or(VmError::Infinity));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f32Array4(result))
            }
            (VmValue::f32Array8(val1), VmValue::f32Array8(val2)) => {
                let mut result: [f32; 8] = [0.0; 8];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| VmValue::mul_f32(a, b).ok_or(VmError::Infinity));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f32Array8(result))
            }
            (VmValue::f64Array2(val1), VmValue::f64Array2(val2)) => {
                let mut result: [f64; 2] = [0.0; 2];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| VmValue::mul_f64(a, b).ok_or(VmError::Infinity));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f64Array2(result))
            }
            (VmValue::f64Array4(val1), VmValue::f64Array4(val2)) => {
                let mut result: [f64; 4] = [0.0; 4];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| VmValue::mul_f64(a, b).ok_or(VmError::Infinity));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f64Array4(result))
            }
            (VmValue::f64Array8(val1), VmValue::f64Array8(val2)) => {
                let mut result: [f64; 8] = [0.0; 8];
                let src = val1
                    .iter()
                    .zip(&val2)
                    .map(|(a, b)| VmValue::mul_f64(a, b).ok_or(VmError::Infinity));

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f64Array8(result))
            }
            (_, _) => panic!("Cannot perform multiplication between different variants!"),
        }
    }
}

impl Div for VmValue {
    type Output = Result<VmValue, VmError>;

    fn div(self, other: VmValue) -> Result<VmValue, VmError> {
        match (self, other) {
            (VmValue::I32(val1), VmValue::I32(val2)) => {
                if val2 == 0 {
                    return Err(VmError::DivideByZero);
                }

                match val1.checked_div(val2) {
                    Some(result) => Ok(VmValue::I32(result)),
                    None => Err(VmError::Overflow),
                }
            }
            (VmValue::I64(val1), VmValue::I64(val2)) => {
                if val2 == 0 {
                    return Err(VmError::DivideByZero);
                }

                match val1.checked_div(val2) {
                    Some(result) => Ok(VmValue::I64(result)),
                    None => Err(VmError::Overflow),
                }
            }
            (VmValue::F32(val1), VmValue::F32(val2)) => {
                if val2 == 0.0 {
                    return Err(VmError::DivideByZero);
                }

                match VmValue::div_f32(&val1, &val2) {
                    Some(result) => Ok(VmValue::F32(result)),
                    None => Err(VmError::Overflow),
                }
            }
            (VmValue::F64(val1), VmValue::F64(val2)) => {
                if val2 == 0.0 {
                    return Err(VmError::DivideByZero);
                }

                match VmValue::div_f64(&val1, &val2) {
                    Some(result) => Ok(VmValue::F64(result)),
                    None => Err(VmError::Overflow),
                }
            }
            (VmValue::i32Array2(val1), VmValue::i32Array2(val2)) => {
                let mut result: [i32; 2] = [0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0 {
                        return Err(VmError::DivideByZero);
                    }

                    a.checked_div(*b).ok_or(VmError::Overflow)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i32Array2(result))
            }
            (VmValue::i32Array4(val1), VmValue::i32Array4(val2)) => {
                let mut result: [i32; 4] = [0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0 {
                        return Err(VmError::DivideByZero);
                    }

                    a.checked_div(*b).ok_or(VmError::Overflow)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i32Array4(result))
            }
            (VmValue::i32Array8(val1), VmValue::i32Array8(val2)) => {
                let mut result: [i32; 8] = [0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0 {
                        return Err(VmError::DivideByZero);
                    }

                    a.checked_div(*b).ok_or(VmError::Overflow)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i32Array8(result))
            }
            (VmValue::i64Array2(val1), VmValue::i64Array2(val2)) => {
                let mut result: [i64; 2] = [0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0 {
                        return Err(VmError::DivideByZero);
                    }

                    a.checked_div(*b).ok_or(VmError::Overflow)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i64Array2(result))
            }
            (VmValue::i64Array4(val1), VmValue::i64Array4(val2)) => {
                let mut result: [i64; 4] = [0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0 {
                        return Err(VmError::DivideByZero);
                    }

                    a.checked_div(*b).ok_or(VmError::Overflow)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i64Array4(result))
            }
            (VmValue::i64Array8(val1), VmValue::i64Array8(val2)) => {
                let mut result: [i64; 8] = [0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0 {
                        return Err(VmError::DivideByZero);
                    }

                    a.checked_div(*b).ok_or(VmError::Overflow)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i64Array8(result))
            }
            (VmValue::f32Array2(val1), VmValue::f32Array2(val2)) => {
                let mut result: [f32; 2] = [0.0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0.0 {
                        return Err(VmError::DivideByZero);
                    }

                    VmValue::div_f32(a, b).ok_or(VmError::Infinity)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f32Array2(result))
            }
            (VmValue::f32Array4(val1), VmValue::f32Array4(val2)) => {
                let mut result: [f32; 4] = [0.0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0.0 {
                        return Err(VmError::DivideByZero);
                    }

                    VmValue::div_f32(a, b).ok_or(VmError::Infinity)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f32Array4(result))
            }
            (VmValue::f32Array8(val1), VmValue::f32Array8(val2)) => {
                let mut result: [f32; 8] = [0.0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0.0 {
                        return Err(VmError::DivideByZero);
                    }

                    VmValue::div_f32(a, b).ok_or(VmError::Infinity)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f32Array8(result))
            }
            (VmValue::f64Array2(val1), VmValue::f64Array2(val2)) => {
                let mut result: [f64; 2] = [0.0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0.0 {
                        return Err(VmError::DivideByZero);
                    }

                    VmValue::div_f64(a, b).ok_or(VmError::Infinity)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f64Array2(result))
            }
            (VmValue::f64Array4(val1), VmValue::f64Array4(val2)) => {
                let mut result: [f64; 4] = [0.0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0.0 {
                        return Err(VmError::DivideByZero);
                    }

                    VmValue::div_f64(a, b).ok_or(VmError::Infinity)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f64Array4(result))
            }
            (VmValue::f64Array8(val1), VmValue::f64Array8(val2)) => {
                let mut result: [f64; 8] = [0.0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0.0 {
                        return Err(VmError::DivideByZero);
                    }

                    VmValue::div_f64(a, b).ok_or(VmError::Infinity)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f64Array8(result))
            }
            (_, _) => panic!("Cannot perform division between different variants!"),
        }
    }
}

impl Rem for VmValue {
    type Output = Result<VmValue, VmError>;

    fn rem(self, other: VmValue) -> Result<VmValue, VmError> {
        match (self, other) {
            (VmValue::I32(val1), VmValue::I32(val2)) => {
                if val2 == 0 {
                    return Err(VmError::DivideByZero);
                }

                match val1.checked_rem(val2) {
                    Some(result) => Ok(VmValue::I32(result)),
                    None => Err(VmError::Overflow),
                }
            }
            (VmValue::I64(val1), VmValue::I64(val2)) => {
                if val2 == 0 {
                    return Err(VmError::DivideByZero);
                }

                match val1.checked_rem(val2) {
                    Some(result) => Ok(VmValue::I64(result)),
                    None => Err(VmError::Overflow),
                }
            }
            (VmValue::F32(val1), VmValue::F32(val2)) => {
                if val2 == 0.0 {
                    return Err(VmError::DivideByZero);
                }

                match VmValue::rem_f32(&val1, &val2) {
                    Some(result) => Ok(VmValue::F32(result)),
                    None => Err(VmError::Overflow),
                }
            }
            (VmValue::F64(val1), VmValue::F64(val2)) => {
                if val2 == 0.0 {
                    return Err(VmError::DivideByZero);
                }

                match VmValue::rem_f64(&val1, &val2) {
                    Some(result) => Ok(VmValue::F64(result)),
                    None => Err(VmError::Overflow),
                }
            }
            (VmValue::i32Array2(val1), VmValue::i32Array2(val2)) => {
                let mut result: [i32; 2] = [0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0 {
                        return Err(VmError::DivideByZero);
                    }

                    a.checked_rem(*b).ok_or(VmError::Overflow)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i32Array2(result))
            }
            (VmValue::i32Array4(val1), VmValue::i32Array4(val2)) => {
                let mut result: [i32; 4] = [0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0 {
                        return Err(VmError::DivideByZero);
                    }

                    a.checked_rem(*b).ok_or(VmError::Overflow)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i32Array4(result))
            }
            (VmValue::i32Array8(val1), VmValue::i32Array8(val2)) => {
                let mut result: [i32; 8] = [0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0 {
                        return Err(VmError::DivideByZero);
                    }

                    a.checked_rem(*b).ok_or(VmError::Overflow)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i32Array8(result))
            }
            (VmValue::i64Array2(val1), VmValue::i64Array2(val2)) => {
                let mut result: [i64; 2] = [0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0 {
                        return Err(VmError::DivideByZero);
                    }

                    a.checked_rem(*b).ok_or(VmError::Overflow)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i64Array2(result))
            }
            (VmValue::i64Array4(val1), VmValue::i64Array4(val2)) => {
                let mut result: [i64; 4] = [0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0 {
                        return Err(VmError::DivideByZero);
                    }

                    a.checked_rem(*b).ok_or(VmError::Overflow)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i64Array4(result))
            }
            (VmValue::i64Array8(val1), VmValue::i64Array8(val2)) => {
                let mut result: [i64; 8] = [0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0 {
                        return Err(VmError::DivideByZero);
                    }

                    a.checked_rem(*b).ok_or(VmError::Overflow)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::i64Array8(result))
            }
            (VmValue::f32Array2(val1), VmValue::f32Array2(val2)) => {
                let mut result: [f32; 2] = [0.0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0.0 {
                        return Err(VmError::DivideByZero);
                    }

                    VmValue::rem_f32(a, b).ok_or(VmError::Infinity)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f32Array2(result))
            }
            (VmValue::f32Array4(val1), VmValue::f32Array4(val2)) => {
                let mut result: [f32; 4] = [0.0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0.0 {
                        return Err(VmError::DivideByZero);
                    }

                    VmValue::rem_f32(a, b).ok_or(VmError::Infinity)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f32Array4(result))
            }
            (VmValue::f32Array8(val1), VmValue::f32Array8(val2)) => {
                let mut result: [f32; 8] = [0.0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0.0 {
                        return Err(VmError::DivideByZero);
                    }

                    VmValue::rem_f32(a, b).ok_or(VmError::Infinity)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f32Array8(result))
            }
            (VmValue::f64Array2(val1), VmValue::f64Array2(val2)) => {
                let mut result: [f64; 2] = [0.0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0.0 {
                        return Err(VmError::DivideByZero);
                    }

                    VmValue::rem_f64(a, b).ok_or(VmError::Infinity)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f64Array2(result))
            }
            (VmValue::f64Array4(val1), VmValue::f64Array4(val2)) => {
                let mut result: [f64; 4] = [0.0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0.0 {
                        return Err(VmError::DivideByZero);
                    }

                    VmValue::rem_f64(a, b).ok_or(VmError::Infinity)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f64Array4(result))
            }
            (VmValue::f64Array8(val1), VmValue::f64Array8(val2)) => {
                let mut result: [f64; 8] = [0.0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| {
                    if *b == 0.0 {
                        return Err(VmError::DivideByZero);
                    }

                    VmValue::rem_f64(a, b).ok_or(VmError::Infinity)
                });

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v.unwrap();
                }

                Ok(VmValue::f64Array8(result))
            }
            (_, _) => panic!("Cannot perform division between different variants!"),
        }
    }
}

impl fmt::Debug for VmValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VmValue::I32(val) => write!(f, "{}", val),
            VmValue::I64(val) => write!(f, "{}", val),
            VmValue::F32(val) => write!(f, "{}", val),
            VmValue::F64(val) => write!(f, "{}", val),
            VmValue::i32Array2(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i32Array4(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i32Array8(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i32Array16(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i32Array32(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i32Array64(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i32Array128(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i32Array256(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i64Array2(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i64Array4(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i64Array8(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i64Array16(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i64Array32(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i64Array64(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i64Array128(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i64Array256(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f32Array2(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f32Array4(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f32Array8(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f32Array16(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f32Array32(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f32Array64(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f32Array128(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f32Array256(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f64Array2(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f64Array4(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f64Array8(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f64Array16(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f64Array32(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f64Array64(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f64Array128(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f64Array256(val) => write!(f, "{:?}", val.to_vec()),
        }
    }
}
