use std::fmt;
use libc::c_int;
use crate::ffi;
use anyhow::{anyhow, Result};

pub struct Rational {
    num: i32,
    den: i32,
}

impl Rational {
    pub fn new(num: i32, den: i32) -> Self {
        Self { num, den }
    }

    pub fn num(&self) -> i32 {
        self.num
    }

    pub fn den(&self) -> i32 {
        self.den
    }

    pub fn reduce(&self) -> Result<Rational> {
        unsafe {
            let mut dst_num: c_int = 0;
            let mut dst_len: c_int = 0;
            let exact = ffi::av_reduce(&mut dst_num, &mut dst_len, i64::from(self.num), i64::from
                (self.den), i64::from(i32::MAX));
            if exact == 1 {
                Ok(Rational::new(dst_num, dst_len))
            } else {
                Err(anyhow!(""))
            }
        }
    }
}

impl From<ffi::AVRational> for Rational {
    fn from(value: ffi::AVRational) -> Rational {
        Rational {
            num: value.num,
            den: value.den,
        }
    }
}

impl From<Rational> for ffi::AVRational {
    fn from(value: Rational) -> ffi::AVRational {
        ffi::AVRational {
            num: value.num,
            den: value.den,
        }
    }
}

impl From<f64> for Rational {
    fn from(value: f64) -> Rational {
        unsafe {
            Rational::from(ffi::av_d2q(value, c_int::MAX))
        }
    }
}

impl From<Rational> for f64 {
    fn from(value: Rational) -> f64 {
        unsafe {
            ffi::av_q2d(value.into())
        }
    }
}

impl From<Rational> for u32 {
    fn from(value: Rational) -> u32 {
        unsafe {
            ffi::av_q2intfloat(value.into())
        }
    }
}

impl From<(i32, i32)> for Rational {
    fn from((num, den): (i32, i32)) -> Rational {
        Rational::new(num, den)
    }
}

impl fmt::Debug for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        f.write_str(&format!(
            "Rational({}/{})",
            self.num(),
            self.den()
        ))
    }
}