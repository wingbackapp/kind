use std::fmt;

/// An error which can be returned by the kind library
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IdError {
    WrongClass,
    InvalidFormat,
    EmptyDbId,
}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongClass => write!(f, "wrong object class"),
            Self::InvalidFormat => write!(f, "invalid format for id"),
            Self::EmptyDbId => write!(f, "empty db id"),
        }
    }
}

impl std::error::Error for IdError {}
