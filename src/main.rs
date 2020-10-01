mod errors;
mod pesc;
mod stdlib;

use crate::pesc::*;
use rustyline::error::ReadlineError;

fn main() {
    let mut pesc = Pesc::new();

    for func in stdlib::functions() {
        pesc.load(func.0, func.1, func.2);
    }

    let args = std::env::args().collect::<Vec<String>>();
    if args.len() > 1 {
        let parsed = match pesc.parse(&args[1]) {
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

    let mut rl = rustyline::Editor::<()>::new();

    loop {
        match rl.readline("pesc> ") {
            Ok(line) => {
                let parsed = match pesc.parse(&line) {
                    Ok(r) => r,
                    Err(e) => {
                        println!("error: {}", e);
                        continue;
                    },
                };

                match pesc.eval(&parsed.1) {
                    Ok(()) => (),
                    Err(e) => println!("error: {}", e),
                }

                pesc.print();
            },
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) =>
                println!("Use Ctrl-D to quit."),
            Err(_) => pesc.print(),
        }
    }
}
