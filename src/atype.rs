use std::fmt;

#[derive(Debug, PartialEq)]
#[derive(Copy, Clone)]
pub enum Atype {
    White,
    Black,
    Red,
}

impl fmt::Display for Atype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Atype::White => write!(f, "⚪️"),
            Atype::Black => write!(f, "⚫️"),
            Atype::Red => write!(f, "🔴"),
        }
    }
}