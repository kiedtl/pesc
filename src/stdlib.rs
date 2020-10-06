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
        (Some('%'), "mod", func!(pesc_mod)),

        (Some('#'), "dup", func!(pesc_dup)),
        (Some('$'), "pop", func!(pesc_pop)),
        (Some(','), "swp", func!(pesc_swp)),
        (Some('ø'), "get", func!(pesc_get)),
        (Some('@'), "rot", func!(pesc_rot)),

        (Some('!'), "neg", func!(pesc_b_neg)),
        (Some('&'), "and", func!(pesc_b_and)),
        (Some('|'), "or",  func!(pesc_b_or)),
        (Some('='), "eq?", func!(pesc_b_eq)),
        (Some('<'), "gt?", func!(pesc_b_gt)),
        (Some('>'), "lt?", func!(pesc_b_lt)),
        (Some('?'), "if?", func!(pesc_b_cond)),

        (Some(';'), "run", func!(pesc_run)),
    ]
}

// --- math functions ---

pub fn pesc_add(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;
    let b = p.pop_number()?;

    p.push(PescToken::Number(a + b));
    Ok(())
}

pub fn pesc_sub(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;
    let b = p.pop_number()?;

    p.push(PescToken::Number(a - b));
    Ok(())
}

pub fn pesc_mul(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;
    let b = p.pop_number()?;

    p.push(PescToken::Number(a * b));
    Ok(())
}

pub fn pesc_div(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;
    let b = p.pop_number()?;

    if b == 0_f64 {
        Err(PescErrorType::DivideByZero(a, b))
    } else {
        p.push(PescToken::Number(a / b));
        Ok(())
    }
}

pub fn pesc_pow(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;
    let b = p.pop_number()?;

    p.push(PescToken::Number(a.powf(b)));
    Ok(())
}

pub fn pesc_mod(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;
    let b = p.pop_number()?;

    if b == 0_f64 {
        Err(PescErrorType::DivideByZero(a, b))
    } else {
        p.push(PescToken::Number(a % b));
        Ok(())
    }
}

// --- stack functions ---

pub fn pesc_dup(p: &mut Pesc) -> Result<(), PescErrorType> {
    let x = p.pop()?;
    p.push(x.clone()); p.push(x);
    Ok(())
}

pub fn pesc_pop(p: &mut Pesc) -> Result<(), PescErrorType> {
    p.pop()?;
    Ok(())
}

pub fn pesc_swp(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop()?;
    let b = p.pop()?;

    p.push(a); p.push(b);
    Ok(())
}

pub fn pesc_get(p: &mut Pesc) -> Result<(), PescErrorType> {
    // copy the nth item on the stack and dup
    let nth = p.pop_number()?;
    let x   = p.nth_ref(nth)?.clone();

    p.push(x);
    Ok(())
}

pub fn pesc_rot(p: &mut Pesc) -> Result<(), PescErrorType> {
    // swap the nth item on the stack with the first item
    let idx   = p.pop_number()?;
    let nth   = p.nth_ref(idx)?.clone();
    let first = p.nth_ref(0.0)?.clone();

    p.set(0.0,   nth)?;
    p.set(idx, first)?;
    Ok(())
}

// --- boolean functions ---

pub fn pesc_b_neg(p: &mut Pesc) -> Result<(), PescErrorType> {
    let v = !p.pop_boolean()?;
    p.push(PescToken::Bool(v));
    Ok(())
}

pub fn pesc_b_and(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_boolean()?;
    let b = p.pop_boolean()?;

    p.push(PescToken::Bool(a && b));
    Ok(())
}

pub fn pesc_b_or(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_boolean()?;
    let b = p.pop_boolean()?;

    p.push(PescToken::Bool(a || b));
    Ok(())
}

pub fn pesc_b_eq(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop()?;
    let b = p.pop()?;

    p.push(PescToken::Bool(a == b));
    Ok(())
}

pub fn pesc_b_gt(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;
    let b = p.pop_number()?;

    p.push(PescToken::Bool(a < b));
    Ok(())
}

pub fn pesc_b_lt(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;
    let b = p.pop_number()?;

    p.push(PescToken::Bool(a > b));
    Ok(())
}

pub fn pesc_b_cond(p: &mut Pesc) -> Result<(), PescErrorType> {
    let cond = p.pop_boolean()?;
    let main_branch = p.pop()?;
    let else_branch = p.pop()?;

    match cond {
        true  => p.try_exec(main_branch)?,
        false => p.try_exec(else_branch)?,
    }

    Ok(())
}

// --- misc functions ---

pub fn pesc_run(p: &mut Pesc) -> Result<(), PescErrorType> {
    let f = p.pop()?;
    p.try_exec(f)
}
