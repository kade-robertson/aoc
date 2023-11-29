use std::fmt::{Display, Formatter};

pub enum Solution {
    U64(u64),
    I64(i64),
    U32(u32),
    I32(i32),
    U16(u16),
    I16(i16),
    USize(usize),
    Str(String),
}

impl Display for Solution {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Solution::U64(val) => write!(f, "{}", val),
            Solution::I64(val) => write!(f, "{}", val),
            Solution::U32(val) => write!(f, "{}", val),
            Solution::I32(val) => write!(f, "{}", val),
            Solution::U16(val) => write!(f, "{}", val),
            Solution::I16(val) => write!(f, "{}", val),
            Solution::USize(val) => write!(f, "{}", val),
            Solution::Str(val) => write!(f, "{}", val),
        }
    }
}

impl From<u64> for Solution {
    fn from(val: u64) -> Self {
        Solution::U64(val)
    }
}

impl From<i64> for Solution {
    fn from(val: i64) -> Self {
        Solution::I64(val)
    }
}

impl From<u32> for Solution {
    fn from(val: u32) -> Self {
        Solution::U32(val)
    }
}

impl From<i32> for Solution {
    fn from(val: i32) -> Self {
        Solution::I32(val)
    }
}

impl From<u16> for Solution {
    fn from(val: u16) -> Self {
        Solution::U16(val)
    }
}

impl From<i16> for Solution {
    fn from(val: i16) -> Self {
        Solution::I16(val)
    }
}

impl From<usize> for Solution {
    fn from(val: usize) -> Self {
        Solution::USize(val)
    }
}

impl From<&str> for Solution {
    fn from(val: &str) -> Self {
        Solution::Str(val.to_string())
    }
}
