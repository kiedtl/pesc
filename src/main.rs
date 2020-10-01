mod errors;
mod pesc;
mod stdlib;

use crate::pesc::*;

fn main() {
    let mut pesc = Pesc::new();

    for func in stdlib::functions() {
        pesc.load(func.0, func.1, func.2);
    }

    let code = &std::env::args()
        .collect::<Vec<String>>()[1];

    let parsed = match pesc.parse(code) {
        Ok(r) => r,
        Err(e) => {
            println!("error: {}", e);
            return;
        },
    };

    match pesc.eval(&parsed.1) {
        Ok(()) => pesc.print(),
        Err(e) => println!("error: {}", e),
    }
}
