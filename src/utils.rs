use std::mem::swap;
use crate::errors::*;

pub fn is_prime(x: usize) -> bool {
    // stolen from this SO answer:
    // https://stackoverflow.com/a/26760082

    if x <= 3 && x > 1 {
        // both 2 and 3 are prime
        true
    } else if x % 2 == 0 || x % 3 == 0 {
        false
    } else {
        let mut i = 5;
        while i * i <= x {
            if x % i == 0 || x % (i + 2) == 0 {
                return false;
            }

            i += 6;
        }

        true
    }
}

pub fn ackermann(m: usize, n: usize) -> usize {
    // TODO: iterative version; A(4, 2) overflows
    //           ⎛
    //           ⎢  n + 1                    if m = 0
    // A(m, n) = ⎨  A(m - 1, 1)              if m > 0 and n = 0
    //           ⎢  A(m - 1, A(m, n - 1))    if m > 0 and n > 0
    //           ⎝
    match (m, n) {
        (0, n) => n + 1,
        (m, 0) if m > 0 => ackermann(m - 1, 1),
        (m, n) if m > 0 && n > 0 =>
            ackermann(m - 1, ackermann(m, n - 1)),
        _ => unreachable!()
    }
}

pub fn lcm(a: usize, b: usize) -> usize {
    //              ⎛           ⎞
    //              ⎜    |a|    ⎟
    // lcm(a, b) =  ⎜ ───────── ⎟ × |b|
    //              ⎜ gcd(a, b) ⎟
    //              ⎝           ⎠
    (a / gcd(a, b)) * b
}

// Josef Stein's binary GCD algorithm
pub fn gcd(mut u: usize, mut v: usize) -> usize {
    let mut shift = 0;

    // gcd(0, v) == v, gcd(u, 0) == u
    if u == 0 {
        return v;
    } else if v == 0 {
        return u
    }

    // found the answer
    if v == u {
        return v;
    }

    if (u & 1) == 0 {
        if (v & 1) == 0 {
            // gcd(2u, 2v) = gcd(u, v)
            2 * gcd(u / 2, v / 2)
        } else {
            // gcd(2u, v) = gcd(u, v)
            gcd(u / 2, v)
        }
    } else if (v & 1) == 0 {
        // gcd(u, 2v) = gcd(u, v)
        gcd(u, v / 2)
    } else {
        // gcd(u, v) = gcd(|u - v|, min(u, v))
        if u < v {
            swap(&mut u, &mut v);
        }

        gcd(u - v, v)
    }
}

pub fn factorial(n: usize) -> usize {
    match n {
        1 => n,
        _ => n * factorial(n - 1),
    }
}

#[inline]
pub fn rom_num_value(c: char) -> Result<usize, PescErrorType> {
    match c {
        'm' | 'M' => Ok(1000),
        'd' | 'D' => Ok(500),
        'c' | 'C' => Ok(100),
        'l' | 'L' => Ok(50),
        'x' | 'X' => Ok(10),
        'v' | 'V' => Ok(5),
        'i' | 'I' => Ok(1),
        _ => Err(PescErrorType::Other(
                format!("invalid roman numeral ('{}')", c))),
    }
}
