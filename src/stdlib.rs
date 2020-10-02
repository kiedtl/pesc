use std::rc::Rc;
use std::vec::Vec;
use crate::errors::*;
use crate::pesc::*;

// --- helper functions ---

macro_rules! func {
    ($x:ident) => (Rc::new(Box::new($x)))
}

pub fn functions<'a>() -> Vec<(Option<char>, &'a str, Rc<Box<PescFunc>>)> {
    vec![
        (Some('+'), "add", func!(pesc_add)),
        (Some('-'), "sub", func!(pesc_sub)),
        (Some('*'), "mul", func!(pesc_mul)),
        (Some('/'), "div", func!(pesc_div)),
        (Some('^'), "pow", func!(pesc_pow)),

        (Some('#'), "dup", func!(pesc_dup)),
        (Some('$'), "pop", func!(pesc_pop)),
        (Some(','), "swp", func!(pesc_swp)),
        (Some('ø'), "get", func!(pesc_get)),
        (Some('@'), "rot", func!(pesc_rot)),

        (Some(';'), "run", func!(pesc_run)),
    ]
}

// --- math functions ---

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

pub fn pesc_mul(p: &mut Pesc) -> Result<(), PescError> {
    let a = p.pop_number()?;
    let b = p.pop_number()?;

    p.push(PescToken::Number(a * b));
    Ok(())
}

pub fn pesc_div(p: &mut Pesc) -> Result<(), PescError> {
    let a = p.pop_number()?;
    let b = p.pop_number()?;

    if b == 0_f64 {
        Err(PescError::new(None, PescErrorType::DivideByZero(a, b)))
    } else {
        p.push(PescToken::Number(a / b));
        Ok(())
    }
}

pub fn pesc_pow(p: &mut Pesc) -> Result<(), PescError> {
    let a = p.pop_number()?;
    let b = p.pop_number()?;

    p.push(PescToken::Number(a.powf(b)));
    Ok(())
}

// --- stack functions ---

pub fn pesc_dup(p: &mut Pesc) -> Result<(), PescError> {
    let x = p.pop()?;
    p.push(x.clone()); p.push(x);
    Ok(())
}

pub fn pesc_pop(p: &mut Pesc) -> Result<(), PescError> {
    p.pop()?;
    Ok(())
}

pub fn pesc_swp(p: &mut Pesc) -> Result<(), PescError> {
    let a = p.pop()?;
    let b = p.pop()?;

    p.push(a); p.push(b);
    Ok(())
}

pub fn pesc_get(p: &mut Pesc) -> Result<(), PescError> {
    // copy the nth item on the stack and dup
    let nth = p.pop_number()?;
    let x   = p.nth_ref(nth)?.clone();

    p.push(x);
    Ok(())
}

pub fn pesc_rot(p: &mut Pesc) -> Result<(), PescError> {
    // swap the nth item on the stack with the first item
    let idx   = p.pop_number()?;
    let nth   = p.nth_ref(idx)?.clone();
    let first = p.nth_ref(0.0)?.clone();

    p.set(0.0,   nth)?;
    p.set(idx, first)?;
    Ok(())
}

// --- misc functions ---

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
