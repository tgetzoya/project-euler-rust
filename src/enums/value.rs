use std::fmt;

#[derive(Debug)]
pub enum Value{
    I32(i32),
    U16(u16),
    U32(u32),
    U64(u64)
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:15?}", self)
    }
}