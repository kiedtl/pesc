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

    // <index>, <length>
    OutOfBounds(f64, usize),

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
            PescErrorType::OutOfBounds(i, _) =>
                format!("The stack isn't as big as you think ({} is out of bounds)", *i as usize),
            PescErrorType::InvalidBoolean(found) =>
                format!("Uh, is {} supposed to be true or false?", found),
        }
    }
}

#[derive(Clone, Debug)]
pub struct PescError {
    pub ch: Option<usize>,
    pub token: Option<PescToken>,
    pub kind: PescErrorType,
}

impl PescError {
    pub fn new(c: Option<usize>, t: Option<PescToken>, k: PescErrorType)
        -> Self
    {
        Self {
            ch: c,
            token: t,
            kind: k
        }
    }

    fn hints(&self) -> Vec<String> {
        match self.kind {
            PescErrorType::UnknownFunction(_) => vec![
                "is the function loaded correctly?".to_string(),
            ],
            PescErrorType::UnmatchedToken(_) => vec![],

            // TODO: check function documentation and hint
            // with the correct number of arguments
            PescErrorType::NotEnoughArguments => vec![],
            PescErrorType::InvalidArgumentType(_, _) => vec![],
            PescErrorType::InvalidNumberLit(_) => vec![
                "number literals may only contain character [0-9_\\.]".to_string(),
                "bases other than decimal are currently not supported.".to_string(),
            ],
            PescErrorType::EmptyLiteral => vec![],
            PescErrorType::DivideByZero(_, _) => vec![],
            PescErrorType::OutOfBounds(_, a) => vec![
                format!("the stack is {} elements long.", a),
            ],
            PescErrorType::InvalidBoolean(_) => vec![
                "only tokens of type `number`, `string`, and `bool` can be cast as boolean.".to_string()
            ],
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
