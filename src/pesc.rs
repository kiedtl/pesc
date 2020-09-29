use std::rc::Rc;
use std::error::Error;
use std::fmt::{self, Display};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub enum PescErrorType {
    // <func>
    UnknownFunction(String),

    // <token> (e.g. "[", "(")
    UnmatchedToken(char),

    // <func>, <expected>
    NotEnoughArguments(String, usize),

    // <func>, <expected>, <found>
    InvalidArgumentType(String, String, String),

    // <found>
    InvalidNumberLit(String),

    EmptyLiteral,
}

impl ToString for PescErrorType {
    fn to_string(&self) -> String {
        match self {
            PescErrorType::UnknownFunction(f) =>
                format!("I have no idea what {} means.", f),
            PescErrorType::UnmatchedToken(t) =>
                format!("Where's the matching '{}'?", t),
            PescErrorType::NotEnoughArguments(f, e) =>
                format!("{} needs {} arguments, OK?", f, e),
            PescErrorType::InvalidArgumentType(n, a, h) =>
                format!("{} wanted a {}, but you gave a {}", n, a, h),
            PescErrorType::InvalidNumberLit(f) =>
                format!("What makes you think '{}' is a number?", f),
            PescErrorType::EmptyLiteral =>
                format!("I don't know what to do with an empty literal."),
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

fn pesc_add(st: &mut Vec<PescToken>) -> Result<(), ()> {
    let (a, b);

    // TODO: make concise
    if let PescToken::Number(n) = st.pop().unwrap() {
        a = n;
    } else {
        return Err(());
    }

    if let PescToken::Number(n) = st.pop().unwrap() {
        b = n;
    } else {
        return Err(());
    }

    st.push(PescToken::Number(a + b));
    Ok(())
}

fn pesc_sub(st: &mut Vec<PescToken>) -> Result<(), ()> {
    let (a, b);

    if let PescToken::Number(n) = st.pop().unwrap() {
        a = n;
    } else {
        return Err(());
    }

    if let PescToken::Number(n) = st.pop().unwrap() {
        b = n;
    } else {
        return Err(());
    }

    st.push(PescToken::Number(a - b));
    Ok(())
}

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

type PescFunc = dyn Fn(&mut Vec<PescToken>) -> Result<(), ()>;

pub struct Pesc {
    stack: Vec<PescToken>,
    ops: HashMap<char, String>,

    funcs: Rc<HashMap<String, Box<PescFunc>>>,
}

impl Pesc {
    pub fn new() -> Self {
        let mut ops = HashMap::new();
        ops.insert('+', String::from("add"));
        ops.insert('-', String::from("sub"));

        let mut funcs: HashMap<String, Box<PescFunc>> = HashMap::new();

        funcs.insert(String::from("add"),
            Box::new(|s| pesc_add(s)));
        funcs.insert(String::from("sub"),
            Box::new(|s| pesc_sub(s)));

        Self {
            stack: Vec::new(),
            ops: ops, funcs: Rc::new(funcs),
        }
    }

    pub fn print(&self) {
        for i in self.stack.iter().rev() {
            print!("[{}] ", i);
        }

        println!();
    }

    pub fn eval(&mut self, code: &[PescToken]) {
        code.iter().for_each(|t| {
            match t {
                PescToken::Symbol(o) => {
                    (self.funcs[&self.ops[o]])(&mut self.stack)
                        .unwrap();
                },
                _ => self.stack.push(t.clone()),
            }
        });
    }

    pub fn parse(&self, input: &str) -> Result<Vec<PescToken>, PescError> {
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
            // TODO: handle numeric parsing errors
            // TODO: parse macros
            // TODO: match functions before pushing

            if chs[i].is_numeric() || chs[i] == '_'
                || chs[i] == '.' {
                    let n = chomp(&chs, i, |c| {
                        !c.is_numeric() && chs[i] != '_' && chs[i] != '.'
                    });
                    i = n.1;

                    let num = match n.0.parse::<f64>() {
                        Ok(o) => o,
                        Err(_) => return Err(PescError::new(Some(i),
                            PescErrorType::InvalidNumberLit(n.0)))
                    };

                    toks.push(PescToken::Number(num));
            } else if chs[i] == '"' {
                let s = chomp(&chs, i, |c| c == '"');
                i = s.1;
                toks.push(PescToken::Str(s.0));
            } else if chs[i] == '(' {
                let n = chomp(&chs, i, |c| c == ')');
                i = n.1;
                toks.push(PescToken::Number(n.0.parse::<f64>().unwrap()));
            } else if chs[i] == '[' {
                let s = chomp(&chs, i, |c| c == ']');
                i = s.1;
                toks.push(PescToken::Func(s.0));
            } else if chs[i] == ' ' {
                i += 1;
                continue;
            } else {
                toks.push(PescToken::Symbol(chs[i]));
                i += 1;
            }
        }

        Ok(toks)
    }
}
