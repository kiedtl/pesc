use std::fmt::{self, Display};
use std::error::*;
use crate::pesc::*;

#[derive(Clone, Debug)]
pub enum PescErrorType {
    // <func>
    UnknownFunction(String),

    // <token> (e.g. "[", "(")
    UnmatchedToken(char),

    NotEnoughArguments,

    // <expected>, <found>
    InvalidArgumentType(String, String),

    // <found>
    InvalidNumberLit(String),

    EmptyLiteral,

    // <a>, <b>
    DivideByZero(f64, f64),

    // <index>
    OutOfBounds(f64),

    // <found>
    InvalidBoolean(PescToken),
}

impl ToString for PescErrorType {
    fn to_string(&self) -> String {
        match self {
            PescErrorType::UnknownFunction(f) =>
                format!("I have no idea what {} means.", f),
            PescErrorType::UnmatchedToken(t) =>
                format!("Where's the matching '{}'?", t),
            PescErrorType::NotEnoughArguments =>
                format!("I need just 1 more argument, OK?"),
            PescErrorType::InvalidArgumentType(h, a) =>
                format!("I wanted a {}, but you gave a {}", h, a),
            PescErrorType::InvalidNumberLit(f) =>
                format!("What makes you think '{}' is a number?", f),
            PescErrorType::EmptyLiteral =>
                format!("I don't know what to do with an empty literal."),
            PescErrorType::DivideByZero(a, b) =>
                format!("You can't divide {} by {}, so don't try.", a, b),
            PescErrorType::OutOfBounds(i) =>
                format!("The stack isn't as big as you think ({} is out of bounds)", *i as usize),
            PescErrorType::InvalidBoolean(found) =>
                format!("Uh, is {} supposed to be true or false?", found),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PescError {
    ch: Option<usize>,
    kind: PescErrorType,
}

impl PescError {
    pub fn new(c: Option<usize>, k: PescErrorType) -> Self {
        Self {
            ch: c,
            kind: k
        }
    }
}

impl Error for PescError {
}

impl Display for PescError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self.kind.to_string())
    }
}
