use std::rc::Rc;
use std::fmt::{self, Display};
use std::collections::HashMap;
use crate::errors::*;
use crate::stdlib::*;

#[derive(Clone, Debug)]
pub enum PescToken {
    Str(String),
    Number(f64),
    Func(String),
    Macro(Vec<PescToken>),
    Symbol(char),
}

impl Display for PescToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            PescToken::Macro(m) => write!(f, "<mac {:p}>", m),
            PescToken::Symbol(y) => write!(f, "<sym '{}'>", y),
            PescToken::Str(s) => write!(f, "{:?}", s),
            PescToken::Number(n) => write!(f, "{}", n),
            PescToken::Func(s) => write!(f, "<fn {}>", s),
        }
    }
}

pub type PescFunc = dyn Fn(&mut Pesc) -> Result<(), PescError>;

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

    pub fn load(&mut self, op: Option<char>, fnname: &str,
        func: Rc<Box<PescFunc>>)
    {
        if let Some(o) = op {
            self.ops.insert(o, String::from(fnname));
        }

        self.funcs.insert(String::from(fnname), func);
    }

    pub fn print(&self) {
        for i in self.stack.iter().rev() {
            print!("[{}] ", i);
        }

        println!();
    }

    pub fn eval(&mut self, code: &[PescToken]) -> Result<(), PescError> {
        for t in code {
            match t {
                PescToken::Symbol(o) => {
                    let funcs = (&self.funcs).clone();
                    (funcs[&self.ops[o]])(self)?;
                },
                _ => self.stack.push(t.clone()),
            }
        }

        Ok(())
    }

    pub fn parse(&self, input: &str)
        -> Result<(usize, Vec<PescToken>), PescError>
    {
        let mut toks = Vec::new();

        let chs = input.chars()
            .collect::<Vec<char>>();
        let mut i = 0;

        fn chomp<F>(ch: &[char], mut c: usize, until: F) -> (String, usize)
        where
            F: Fn(char) -> bool
        {
            let mut buf = String::new();

            while c < ch.len() && until(ch[c]) == false {
                buf += &format!("{}", ch[c]);
                c += 1;
            }

            (buf, c)
        }

        while i < chs.len() {
            if chs[i].is_numeric() || chs[i] == '_'
                || chs[i] == '.' {
                    let n = chomp(&chs, i, |c| {
                        !c.is_digit(10) && c != '_' && c != '.'
                    });
                    i = n.1;

                    let num = match n.0.replace("_", "").parse::<f64>() {
                        Ok(o) => o,
                        Err(_) => return Err(PescError::new(Some(i),
                            PescErrorType::InvalidNumberLit(n.0)))
                    };

                    toks.push(PescToken::Number(num));
            } else if chs[i] == '"' {
                let s = chomp(&chs, i + 1, |c| c == '"');
                i = s.1 + 1;
                toks.push(PescToken::Str(s.0));
            } else if chs[i] == '(' {
                let n = chomp(&chs, i + 1, |c| c == ')');
                i = n.1 + 1;

                let num = match n.0.replace("_", "").parse::<f64>() {
                    Ok(o) => o,
                    Err(_) => return Err(PescError::new(Some(i),
                        PescErrorType::InvalidNumberLit(n.0)))
                };

                toks.push(PescToken::Number(num));
            } else if chs[i] == '[' {
                let s = chomp(&chs, i + 1, |c| c == ']');
                i = s.1 + 1;

                if !self.funcs.contains_key(&s.0) {
                    return Err(PescError::new(None,
                        PescErrorType::UnknownFunction(s.0)));
                } else {
                    toks.push(PescToken::Func(s.0));
                }
            } else if chs[i] == '{' {
                let res = self.parse(&input[i + 1..])?;
                i += res.0 + 1;
                toks.push(PescToken::Macro(res.1));
            } else if chs[i] == '}' {
                return Ok((i, toks));
            } else if chs[i] == ' ' || chs[i] == '\n' {
                i += 1;
                continue;
            } else {
                if !self.ops.contains_key(&chs[i]) {
                    return Err(PescError::new(None,
                        PescErrorType::UnknownFunction(
                            format!("'{}'", chs[i]))));
                } else {
                    toks.push(PescToken::Symbol(chs[i]));
                }
                i += 1;
            }
        }

        println!("returning");
        Ok((i, toks))
    }

    pub fn push(&mut self, v: PescToken) {
        self.stack.push(v)
    }

    pub fn pop(&mut self) -> Result<PescToken, PescError> {
        match self.stack.pop() {
            Some(value) => Ok(value),
            None => Err(PescError::new(None,
                    PescErrorType::NotEnoughArguments))
        }
    }

    pub fn pop_number(&mut self) -> Result<f64, PescError> {
        let v = self.pop()?;

        if let PescToken::Number(n) = v {
            Ok(n)
        } else {
            Err(PescError::new(None,
                PescErrorType::InvalidArgumentType(
                    String::from("number"),
                    v.to_string())))
        }
    }
}

