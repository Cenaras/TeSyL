use std::fmt::{Display, Formatter};

#[derive(Copy, Clone)]
pub enum Type {
    IntType,
    UnitType,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Type::IntType => write!(f, "IntType"),
            Type::UnitType => write!(f, "UnitType"),
        }
    }
}

// Also have Non-Interference typing here?
