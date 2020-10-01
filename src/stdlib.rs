use std::rc::Rc;
use std::vec::Vec;
use crate::errors::*;
use crate::pesc::*;

macro_rules! func {
    ($x:ident) => (Rc::new(Box::new($x)))
}

pub fn functions<'a>() -> Vec<(Option<char>, &'a str, Rc<Box<PescFunc>>)> {
    vec![
        (Some('+'), "add", func!(pesc_add)),
        (Some('-'), "sub", func!(pesc_sub)),
        (Some(';'), "run", func!(pesc_run)),
    ]
}

pub fn pesc_add(p: &mut Pesc) -> Result<(), PescError> {
    let a = p.pop_number()?;
    let b = p.pop_number()?;

    p.push(PescToken::Number(a + b));
    Ok(())
}

pub fn pesc_sub(p: &mut Pesc) -> Result<(), PescError> {
    let a = p.pop_number()?;
    let b = p.pop_number()?;

    p.push(PescToken::Number(a - b));
    Ok(())
}

pub fn pesc_run(p: &mut Pesc) -> Result<(), PescError> {
    let sttop = p.pop()?;
    if let PescToken::Func(func) = sttop {
        (&p.funcs.clone()[&func])(p)
    } else if let PescToken::Macro(mac) = sttop {
        p.eval(&mac)
    } else {
        Err(PescError::new(None,
            PescErrorType::InvalidArgumentType(
                String::from("macro/function"),
                sttop.to_string())))
    }
}
