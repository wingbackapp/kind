use std::fmt;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum IdError {
    WrongClass,
    InvalidFormat,
    EmptyDbId,
}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::WrongClass => write!(f, "wrong class"),
            Self::InvalidFormat => write!(f, "invalid format"),
            Self::EmptyDbId => write!(f, "empty db id"),
        }
    }
}

impl std::error::Error for IdError {}
