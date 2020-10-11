use std::rc::Rc;
use std::fmt::{self, Display};
use std::collections::HashMap;
use crate::errors::*;

/// This is a special, reserved operator that pushes (true) to the stack.
pub const BOOLEAN_TRUE:  char = 'T';

/// Same as BOOLEAN_TRUE, only it pushes (false) to the stack.
pub const BOOLEAN_FALSE: char = 'F';

#[derive(Clone, Debug, PartialEq)]
/// A single Pesc token, which can be either the result of parsing
/// some Pesc code, or be a value on the stack.
pub enum PescToken {
    Str(String),
    Number(PescNumber),
    Func(String),
    Macro(Vec<PescToken>),
    Symbol(char),
    Bool(bool),
}

impl Display for PescToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            PescToken::Macro(m) => write!(f, "<mac {:p}>", m),
            PescToken::Symbol(y) => write!(f, "<sym '{}'>", y),
            PescToken::Str(s) => write!(f, "{:?}", s),
            PescToken::Number(n) => write!(f, "{}", n),
            PescToken::Func(s) => write!(f, "<fn {}>", s),
            PescToken::Bool(b) => write!(f, "({})", b),
        }
    }
}

pub type PescNumber = f64;
pub type PescFunc = dyn Fn(&mut Pesc) -> Result<(), PescErrorType>;

pub struct Pesc {
    pub stack: Vec<PescToken>,
    pub funcs: HashMap<String, Rc<Box<PescFunc>>>,
    pub ops: HashMap<char, String>,
}

impl Pesc {
    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            funcs: HashMap::new(),
            ops: HashMap::new(),
        }
    }

    /// "Load" a function in order to allow Pesc code to call it.
    pub fn load(&mut self, op: Option<char>, fnname: &str,
        func: Rc<Box<PescFunc>>)
    {
        if let Some(o) = op {
            assert!(!self.ops.contains_key(&o),
                "cannot add operator {:?}: already added", op);
            assert!(o != BOOLEAN_TRUE && o != BOOLEAN_FALSE,
                "cannot add operator {:?}: reserved keyword", op);

            self.ops.insert(o, String::from(fnname));
        }

        let s_fnname = String::from(fnname);

        if !self.funcs.contains_key(&s_fnname) {
            self.funcs.insert(s_fnname, func);
        }
    }

    /// Evaluate some Pesc tokens (that have already been parsed with
    /// Pesc::parse)
    pub fn eval(&mut self, code: &[PescToken])
        -> Result<(), (Vec<PescToken>, PescError)>
    {
        for t in code {
            match t {
                PescToken::Symbol(_)
                | PescToken::Func(_) => {
                    match self.exec(t.clone()) {
                        Ok(()) => (),
                        Err((b, e)) => return Err((b,
                            PescError::new(None, Some(t.clone()), e))),
                    };
                },
                _ => self.stack.push(t.clone()),
            }
        }

        Ok(())
    }

    /// Execute a Pesc macro or a Pesc function, returning an error
    /// in the case of failure.
    pub fn try_exec(&mut self, tok: PescToken) -> Result<(), PescErrorType> {
        match self.exec(tok) {
            Ok(()) => Ok(()),
            Err((_, e)) => Err(e),
        }
    }

    /// The same as Pesc::try_exec, but return a copy of the Pesc stack
    /// at the time of the failure as well as an error.
    fn exec(&mut self, tok: PescToken)
        -> Result<(), (Vec<PescToken>, PescErrorType)>
    {
        match tok {
            PescToken::Symbol(o) => {
                if !self.ops.contains_key(&o) {
                    return Err((self.stack.clone(),
                        PescErrorType::UnknownFunction(format!("'{}'", o))));
                }

                self.exec(PescToken::Func(self.ops[&o].clone()))
            },
            PescToken::Func(_func) => {
                let func = _func.to_lowercase();
                if !self.funcs.contains_key(&func) {
                    return Err((self.stack.clone(),
                        PescErrorType::UnknownFunction(func)));
                }

                let backup = self.stack.clone();
                match (&self.funcs.clone()[&func])(self) {
                    Ok(()) => Ok(()),
                    Err(e) => {
                        let badstack = self.stack.clone();
                        self.stack = backup;
                        Err((badstack, e))
                    },
                }
            },
            PescToken::Macro(mac) => match self.eval(&mac) {
                Ok(()) => Ok(()),
                Err((b, e)) => Err((b, e.kind)),
            },
            _ => Err((self.stack.clone(), PescErrorType::InvalidArgumentType(
                String::from("macro/function"), tok.to_string())))
        }
    }

    // TODO: cleanup, remove duplicated code
    // here be atrocious code
    /// Parse some input into a vector of Pesc tokens.
    /// Note that the first element in the tuple (that is, the usize)
    /// can be safely discarded.
    ///
    /// # Examples
    ///
    /// ```
    /// let pesc = Pesc::new();
    ///
    /// let stuff = "1 1 + \\ 2 + [ack]";
    /// let parsed = (Pesc::parse(stuff)).1;
    ///
    /// pesc.eval(parsed);
    ///
    /// // print the stack
    /// pesc.stack.iter().rev().for_each(|i| println!("{}", i));
    /// ```
    pub fn parse(input: &str) -> Result<(usize, Vec<PescToken>), PescError> {
        let mut toks = Vec::new();

        let chs = input.chars()
            .collect::<Vec<char>>();
        let mut i = 0;

        // chomp the chars, placing them in a buffer.
        // return (String, usize, bool) when until() -> true or
        // when it reaches the end of the chars
        //
        //     String = the buffer,
        //     usize  = the end index,
        //     bool   = did we reach the end of the data
        //              without having until() return true?
        //
        fn chomp<F>(ch: &[char], mut c: usize, until: F)
            -> (String, usize, bool)
        where
            F: Fn(char) -> bool
        {
            let mut buf = String::new();
            let early_return;

            loop {
                if c >= ch.len() {
                    early_return = true;
                    break;
                }

                if until(ch[c]) == true {
                    early_return = false;
                    break;
                }

                buf += &format!("{}", ch[c]);
                c += 1;
            }

            (buf, c, early_return)
        }

        while i < chs.len() {
            match chs[i] {
                // integer literals
                _ if chs[i].is_numeric() || chs[i] == '.'
                                         || chs[i] == '_' => {
                    let mut sign = 1_f64;

                    let n = chomp(&chs, i, |c| {
                        !c.is_digit(10) && c != '_' && c != '.'
                    });
                    i = n.1;

                    if n.0.chars().nth(0).unwrap() == '_' {
                        sign = -1_f64;
                    }

                    let num = match n.0.replace("_", "").parse::<PescNumber>() {
                        Ok(o) => o,
                        Err(_) => return Err(PescError::new(Some(i), None,
                            PescErrorType::InvalidNumberLit(n.0)))
                    };

                    toks.push(PescToken::Number(num * sign));
                },

                '(' => {
                    let mut sign = 1_f64;

                    let n = chomp(&chs, i + 1, |c| c == ')');
                    i = n.1 + 1;

                    if n.2 {
                        // we hit the end of the data
                        // without finding a matching bracket
                        return Err(PescError::new(Some(i), None,
                            PescErrorType::UnmatchedToken('(')));
                    }

                    if n.0.len() == 0 {
                        return Err(PescError::new(Some(i), None,
                            PescErrorType::EmptyLiteral));
                    } else {
                        if n.0.chars().nth(0).unwrap() == '_' {
                            // we don't need to check for a '-' prefix,
                            // as the parse::<f64>() function will detect
                            // that for us
                            sign = -1_f64;
                        }
                    }

                    let num = match n.0.replace("_", "").parse::<PescNumber>() {
                        Ok(o) => o,
                        Err(_) => return Err(PescError::new(Some(i), None,
                            PescErrorType::InvalidNumberLit(n.0)))
                    };

                    toks.push(PescToken::Number(num * sign));
                },

                // strings
                '"' => {
                    let s = chomp(&chs, i + 1, |c| c == '"');
                    i = s.1 + 1;

                    if s.2 {
                        // we hit the end of the data
                        // without finding a matching quote
                        return Err(PescError::new(Some(i), None,
                            PescErrorType::UnmatchedToken('"')));
                    }

                    toks.push(PescToken::Str(s.0));
                },

                // functions
                '[' => {
                    let s = chomp(&chs, i + 1, |c| c == ']');
                    i = s.1 + 1;

                    if s.2 {
                        // we hit the end of the data
                        // without finding a matching bracket
                        return Err(PescError::new(Some(i), None,
                            PescErrorType::UnmatchedToken('[')));
                    }

                    toks.push(PescToken::Func(s.0));
                },

                // macros
                '{' => {
                    let res = Pesc::parse(&input[i + 1..])?;
                    toks.push(PescToken::Macro(res.1));

                    // move pointer past matching '}', or we
                    // will exit prematurely (see next item)
                    i += res.0 + 2;
                },

                '}' => return Ok((i, toks)),

                // whitespace
                '\n'
                | '\t'
                | ' ' => { i += 1; continue; },

                // comments
                '#' =>
                    i = chomp(&chs, i + 1, |c| c == '\n' || c == '\\').1 + 1,

                // boolean values
                BOOLEAN_TRUE => {
                    toks.push(PescToken::Bool(true));
                    i += 1;
                },

                BOOLEAN_FALSE => {
                    toks.push(PescToken::Bool(false));
                    i += 1;
                },

                // treat unknown characters as symbols aka operators
                _ => {
                    toks.push(PescToken::Symbol(chs[i]));
                    i += 1;
                }
            }
        }

        Ok((i, toks))
    }

    /// Get a reference to the nth item in the Pesc stack.
    pub fn nth_ref(&self, i: PescNumber) -> Result<&PescToken, PescErrorType> {
        match self.stack.iter().rev().nth(i as usize) {
            Some(value) => Ok(value),
            None => Err(PescErrorType::OutOfBounds(i, self.stack.len())),
        }
    }

    /// Set the nth value in the Pesc stack to a value.
    pub fn set(&mut self, i: PescNumber, v: PescToken) -> Result<(), PescErrorType> {
        let len = self.stack.len();
        if len <= i as usize {
            Err(PescErrorType::OutOfBounds(i, self.stack.len()))
        } else {
            self.stack[(len - 1) - (i as usize)] = v;
            Ok(())
        }
    }

    /// Push an item to the stack.
    pub fn push(&mut self, v: PescToken) {
        self.stack.push(v)
    }

    /// Pop the first item on the stack.
    pub fn pop(&mut self) -> Result<PescToken, PescErrorType> {
        match self.stack.pop() {
            Some(value) => Ok(value),
            None => Err(PescErrorType::NotEnoughArguments)
        }
    }

    // TODO: merge pop_* into a single function (so we don't have all
    // this duplicated code)
    /// Pop the first value off of the stack, returning an error if the
    /// value is not a number.
    pub fn pop_number(&mut self) -> Result<PescNumber, PescErrorType> {
        let v = self.pop()?;

        if let PescToken::Number(n) = v {
            Ok(n)
        } else {
            Err(PescErrorType::InvalidArgumentType(
                String::from("number"), v.to_string()))
        }
    }

    /// Pop the first value off of the stack, returning an error if the
    /// value is not a string.
    pub fn pop_string(&mut self) -> Result<String, PescErrorType> {
        let v = self.pop()?;

        if let PescToken::Str(n) = v {
            Ok(n)
        } else {
            Err(PescErrorType::InvalidArgumentType(
                String::from("string"), v.to_string()))
        }
    }

    /// Pop the first value off of the stack, returning an error if the
    /// value is not a macro.
    pub fn pop_macro(&mut self) -> Result<Vec<PescToken>, PescErrorType> {
        let v = self.pop()?;

        if let PescToken::Macro(m) = v {
            Ok(m)
        } else {
            Err(PescErrorType::InvalidArgumentType(
                String::from("macro"), v.to_string()))
        }
    }

    /// Pop the first value off of the stack, returning an error if the
    /// value is not a boolean value (i.e., is not a string, number, or
    /// bool)
    pub fn pop_boolean(&mut self) -> Result<bool, PescErrorType> {
        let v = self.pop()?;
        match v {
            PescToken::Str(s) => if s == String::from("") {
                Ok(false)
            } else {
                Ok(true)
            },
            PescToken::Number(n) => if n == 0.0 {
                Ok(false)
            } else {
                Ok(true)
            },
            PescToken::Bool(b) => Ok(b),
            _ => Err(PescErrorType::InvalidBoolean(v))
        }
    }
}
