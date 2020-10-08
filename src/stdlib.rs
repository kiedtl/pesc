use std::rc::Rc;
use std::vec::Vec;
use crate::errors::*;
use crate::pesc::*;
use crate::utils::*;
use std::ops::*;
use crate::rand;

const PESC_EX_E_ITERS: usize = 64;

// --- helper functions ---

macro_rules! rc_box {
    ($x:ident) => (Rc::new(Box::new($x)))
}

// --- declaration ---

pub fn standard<'a>() -> Vec<(Option<char>, &'a str, Rc<Box<PescFunc>>)> {
    vec![
        (Some('+'), "add",  rc_box!(pesc_add)),
        (Some('-'), "sub",  rc_box!(pesc_sub)),
        (Some('*'), "mul",  rc_box!(pesc_mul)),
        (Some('/'), "div",  rc_box!(pesc_div)),
        (Some('÷'), "div",  rc_box!(pesc_div)),
        (Some('^'), "pow",  rc_box!(pesc_pow)),
        (Some('%'), "mod",  rc_box!(pesc_mod)),

        (Some('#'), "dup",  rc_box!(pesc_dup)),
        (Some('$'), "pop",  rc_box!(pesc_pop)),
        (Some(','), "swp",  rc_box!(pesc_swp)),
        (Some('ø'), "get",  rc_box!(pesc_get)),
        (Some('@'), "rot",  rc_box!(pesc_rot)),

        (Some('&'), "band", rc_box!(pesc_ex_band)),
        (Some('|'), "bor",  rc_box!(pesc_ex_bor)),
        (Some('X'), "bxor", rc_box!(pesc_ex_bxor)),
        (Some('<'), "shl",  rc_box!(pesc_ex_bshiftl)),
        (Some('>'), "shr",  rc_box!(pesc_ex_bshiftr)),

        (Some('!'), "neg",  rc_box!(pesc_b_neg)),
        (Some(';'), "run",  rc_box!(pesc_run)),
    ]
}

pub fn extended<'a>() -> Vec<(Option<char>, &'a str, Rc<Box<PescFunc>>)> {
    vec![
        (None,      "and",     rc_box!(pesc_b_and)),
        (None,      "or",      rc_box!(pesc_b_or)),
        (None,      "eq?",     rc_box!(pesc_b_eq)),
        (None,      "gt?",     rc_box!(pesc_b_gt)),
        (None,      "lt?",     rc_box!(pesc_b_lt)),
        (Some('?'), "if?",     rc_box!(pesc_b_cond)),

        (None,      "lte",     rc_box!(pesc_ex_lte)),
        (None,      "gte",     rc_box!(pesc_ex_gte)),
        (None,      "def",     rc_box!(pesc_ex_def)),
        (Some('s'), "size",    rc_box!(pesc_ex_size)),
        (Some('r'), "rand",    rc_box!(pesc_ex_rand)),

        (None,      "sin",     rc_box!(pesc_ex_sin)),
        (None,      "cos",     rc_box!(pesc_ex_cos)),
        (None,      "tan",     rc_box!(pesc_ex_tan)),
        (None,      "csc",     rc_box!(pesc_ex_csc)),
        (None,      "sec",     rc_box!(pesc_ex_sec)),
        (None,      "cot",     rc_box!(pesc_ex_cot)),
        (None,      "atan",    rc_box!(pesc_ex_atan)),

        (Some('l'), "log",     rc_box!(pesc_ex_log)),
        (None,      "sqrt",    rc_box!(pesc_ex_sqrt)),
        (None,      "cbrt",    rc_box!(pesc_ex_cbrt)),
        (None,      "fact",    rc_box!(pesc_ex_fact)),
        (Some('A'), "ack",     rc_box!(pesc_ex_ack)),
        (Some('a'), "abs",     rc_box!(pesc_ex_abs)),
        (None,      "lcm",     rc_box!(pesc_ex_lcm)),
        (None,      "gcd",     rc_box!(pesc_ex_gcd)),

        (Some('p'), "pi",      rc_box!(pesc_ex_pi)),
        (Some('e'), "e",       rc_box!(pesc_ex_e)),

        (Some('m'), "min",     rc_box!(pesc_ex_min)),
        (Some('M'), "max",     rc_box!(pesc_ex_max)),
        (Some('c'), "clamp",   rc_box!(pesc_ex_clamp)),

        (None,      "floor",   rc_box!(pesc_ex_floor)),
        (None,      "ceil",    rc_box!(pesc_ex_ceil)),
        (None,      "round",   rc_box!(pesc_ex_round)),

        (None,      "frrn",    rc_box!(pesc_ex_frrn)),
        (None,      "torn",    rc_box!(pesc_ex_torn)),

        (None,      "odd",     rc_box!(pesc_ex_odd)),
        (None,      "even",    rc_box!(pesc_ex_even)),

        (None,      "coprime", rc_box!(pesc_ex_coprime)),
        (None,      "prime",   rc_box!(pesc_ex_prime)),
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

pub fn pesc_ex_lte(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;
    let b = p.pop_number()?;

    p.push(PescToken::Bool(a >= b));
    Ok(())
}

pub fn pesc_ex_gte(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;
    let b = p.pop_number()?;

    p.push(PescToken::Bool(a <= b));
    Ok(())
}

pub fn pesc_ex_def(p: &mut Pesc) -> Result<(), PescErrorType> {
    let name = p.pop_string()?;
    let body = p.pop_macro()?;

    p.funcs.insert(name, Rc::new(Box::new(move |p|
                p.try_exec(PescToken::Macro(body.clone())))));
    Ok(())
}

pub fn pesc_ex_size(p: &mut Pesc) -> Result<(), PescErrorType> {
    p.push(PescToken::Number(p.stack.len() as f64));
    Ok(())
}

pub fn pesc_ex_rand(p: &mut Pesc) -> Result<(), PescErrorType> {
    // TODO: random decimal, no first zero
    let r = unsafe { rand::lrand48() } as f64;
    p.push(PescToken::Number(r));
    Ok(())
}

pub fn pesc_ex_band(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()? as usize;
    let b = p.pop_number()? as usize;

    p.push(PescToken::Number((a & b) as f64));
    Ok(())
}

pub fn pesc_ex_bor(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()? as usize;
    let b = p.pop_number()? as usize;

    p.push(PescToken::Number((a | b) as f64));
    Ok(())
}

pub fn pesc_ex_bxor(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()? as usize;
    let b = p.pop_number()? as usize;

    p.push(PescToken::Number((a ^ b) as f64));
    Ok(())
}

pub fn pesc_ex_bshiftr(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()? as usize;
    let b = p.pop_number()? as usize;

    p.push(PescToken::Number((a >> b) as f64));
    Ok(())
}

pub fn pesc_ex_bshiftl(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()? as usize;
    let b = p.pop_number()? as usize;

    p.push(PescToken::Number((a << b) as f64));
    Ok(())
}

pub fn pesc_ex_sin(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;

    p.push(PescToken::Number(a.sin()));
    Ok(())
}

pub fn pesc_ex_cos(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;

    p.push(PescToken::Number(a.cos()));
    Ok(())
}

pub fn pesc_ex_tan(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;

    p.push(PescToken::Number(a.tan()));
    Ok(())
}

pub fn pesc_ex_sec(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;

    p.push(PescToken::Number(1_f64 / a.cos()));
    Ok(())
}

pub fn pesc_ex_csc(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;

    p.push(PescToken::Number(1_f64 / a.sin()));
    Ok(())
}

pub fn pesc_ex_cot(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;

    p.push(PescToken::Number(1_f64 / a.tan()));
    Ok(())
}

pub fn pesc_ex_atan(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;

    p.push(PescToken::Number(a.atan()));
    Ok(())
}

pub fn pesc_ex_log(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;
    let b = p.pop_number()?;

    p.push(PescToken::Number(a.log(b)));
    Ok(())
}

pub fn pesc_ex_pi(p: &mut Pesc) -> Result<(), PescErrorType> {
    // machin formula
    // pi = (4 * arctangent(1/5) - arctangent(1/239)) * 4

    let pi = (4_f64 * (1_f64/5_f64).atan()
        - (1_f64/239_f64).atan()) * 4_f64;
    p.push(PescToken::Number(pi));
    Ok(())
}

pub fn pesc_ex_e(p: &mut Pesc) -> Result<(), PescErrorType> {
    //         inf
    //         ___  1
    // e = 1 + \   ───
    //         /__ +n!
    //         n=0

    #[inline]
    fn calc_e(iters: usize, accm: f64) -> f64 {
        match iters {
            0 => accm,
            _ => {
                let naccm = 1_f64 / factorial(iters) as f64;
                calc_e(iters - 1, accm + naccm)
            }
        }
    }

    let e = 1_f64 + calc_e(PESC_EX_E_ITERS, 0_f64);

    p.push(PescToken::Number(e));
    Ok(())
}

pub fn pesc_ex_min(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;
    let b = p.pop_number()?;

    p.push(PescToken::Number(if a < b { a } else { b }));
    Ok(())
}

pub fn pesc_ex_max(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()?;
    let b = p.pop_number()?;

    p.push(PescToken::Number(if a > b { a } else { b }));
    Ok(())
}

pub fn pesc_ex_clamp(p: &mut Pesc) -> Result<(), PescErrorType> {
    let val = p.pop_number()?;
    let min = p.pop_number()?;
    let max = p.pop_number()?;

    let res = match () {
        _ if val < min => min,
        _ if val > max => max,
        _ => val,
    };

    p.push(PescToken::Number(res));
    Ok(())
}

pub fn pesc_ex_sqrt(p: &mut Pesc) -> Result<(), PescErrorType> {
    let v = p.pop_number()?;

    p.push(PescToken::Number(v.sqrt()));
    Ok(())
}

pub fn pesc_ex_cbrt(p: &mut Pesc) -> Result<(), PescErrorType> {
    let v = p.pop_number()?;

    p.push(PescToken::Number(v.cbrt()));
    Ok(())
}

pub fn pesc_ex_fact(p: &mut Pesc) -> Result<(), PescErrorType> {
    let v = p.pop_number()? as usize;

    p.push(PescToken::Number(factorial(v) as f64));
    Ok(())
}

pub fn pesc_ex_ceil(p: &mut Pesc) -> Result<(), PescErrorType> {
    let v = p.pop_number()?;

    p.push(PescToken::Number(v.ceil()));
    Ok(())
}

pub fn pesc_ex_floor(p: &mut Pesc) -> Result<(), PescErrorType> {
    let v = p.pop_number()?;

    p.push(PescToken::Number(v.floor()));
    Ok(())
}

pub fn pesc_ex_round(p: &mut Pesc) -> Result<(), PescErrorType> {
    let v = p.pop_number()?;

    p.push(PescToken::Number(v.round()));
    Ok(())
}

pub fn pesc_ex_torn(p: &mut Pesc) -> Result<(), PescErrorType> {
    let mut v = p.pop_number()?.round() as usize;

    let bufsz = 2048 + 6;
    let mut buf: Vec<char> = Vec::new();

    while v != 0 {
        match () {
            _ if v >= 1000 => { v -= 1000; buf.push('M') },
            _ if v >=  500 => { v -=  500; buf.push('D') },
            _ if v >=  100 => { v -=  100; buf.push('C') },
            _ if v >=   50 => { v -=   50; buf.push('L') },
            _ if v >=   10 => { v -=   10; buf.push('X') },
            _ if v >=    5 => { v -=    5; buf.push('V') },
            _ if v >=    1 => { v -=    1; buf.push('I') },
            _ => (),
        }
    }

    p.push(PescToken::Str(buf.iter().collect::<String>()));
    Ok(())
}

pub fn pesc_ex_frrn(p: &mut Pesc) -> Result<(), PescErrorType> {
    let v = p.pop_string()?;

    let mut ctr = 0;
    let chs = v.chars().collect::<Vec<char>>();
    let mut buf = 0;

    while ctr < chs.len() {
        buf += rom_num_value(chs[ctr])?;
        ctr += 1;
    }

    p.push(PescToken::Number(buf as f64));
    Ok(())
}

pub fn pesc_ex_gcd(p: &mut Pesc) -> Result<(), PescErrorType> {
    let u = p.pop_number()? as usize;
    let v = p.pop_number()? as usize;

    p.push(PescToken::Number(gcd(u, v) as f64));
    Ok(())
}

pub fn pesc_ex_lcm(p: &mut Pesc) -> Result<(), PescErrorType> {
    let a = p.pop_number()? as usize;
    let b = p.pop_number()? as usize;

    p.push(PescToken::Number(lcm(a, b) as f64));
    Ok(())
}

pub fn pesc_ex_ack(p: &mut Pesc) -> Result<(), PescErrorType> {
    let m = p.pop_number()? as usize;
    let n = p.pop_number()? as usize;

    p.push(PescToken::Number(ackermann(m, n) as f64));
    Ok(())
}

pub fn pesc_ex_odd(p: &mut Pesc) -> Result<(), PescErrorType> {
    let v = p.pop_number()? as usize;

    p.push(PescToken::Bool(v & 1 == 1));
    Ok(())
}

pub fn pesc_ex_even(p: &mut Pesc) -> Result<(), PescErrorType> {
    let v = p.pop_number()? as usize;

    p.push(PescToken::Bool(v & 1 == 0));
    Ok(())
}

pub fn pesc_ex_abs(p: &mut Pesc) -> Result<(), PescErrorType> {
    let v = p.pop_number()?;

    p.push(PescToken::Number(v.abs()));
    Ok(())
}

pub fn pesc_ex_coprime(p: &mut Pesc) -> Result<(), PescErrorType> {
    let u = p.pop_number()? as usize;
    let v = p.pop_number()? as usize;

    p.push(PescToken::Bool(gcd(u, v) == 1));
    Ok(())
}

pub fn pesc_ex_prime(p: &mut Pesc) -> Result<(), PescErrorType> {
    let x = p.pop_number()? as usize;

    p.push(PescToken::Bool(is_prime(x)));
    Ok(())
}

// --- misc functions ---

pub fn pesc_run(p: &mut Pesc) -> Result<(), PescErrorType> {
    let f = p.pop()?;
    p.try_exec(f)
}

