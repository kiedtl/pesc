mod args;
mod errors;
mod pesc;
mod stdlib;
mod clihints;
mod tty;
mod output;
mod serde;
mod rand;
mod utils;

use crate::pesc::*;
use crate::clihints::*;
use crate::output::*;
use crate::args::*;

use rustyline::{
    config::{
        Builder,
        EditMode,
    },
    error::ReadlineError,
    Editor,
};

fn main() {
    let opts = match Options::new().parse() {
        Ok(o) => o,
        Err(()) => return,
    };

    let mut pesc = Pesc::new();
    let output = OutputMode::auto();

    // load standard library
    for func in stdlib::standard() {
        pesc.load(func.0, func.1, func.2);
    }

    for func in stdlib::extended() {
        pesc.load(func.0, func.1, func.2);
    }

    // waitaminute, let's see if there is a file we
    // need execute
    if let Some(path) = opts.file {
        let data = std::fs::read_to_string(path).unwrap();
        let parsed = match Pesc::parse(&data) {
            Ok(r) => r,
            Err(e) => {
                println!("pesc: error: {}", e);
                return;
            },
        };

        match pesc.eval(&parsed.1) {
            Ok(()) => output.format_stack(&pesc.stack),
            Err((b, e)) => {
                println!("pesc: error: {}", e);
                println!("pesc: problematic stack:");
                output.format_stack(&b);
            },
        }

        return;
    }

    // nope, display a pretty prompt & take orders
    // from stdin
    let config = Builder::new()
        .auto_add_history(true)
        .history_ignore_space(true)
        .edit_mode(EditMode::Vi)
        .build();

    let mut rl = Editor::<CommandHinter>::with_config(config);
    rl.set_helper(Some(CommandHinter::new(hints(&pesc))));

    loop {
        match rl.readline("pesc> ") {
            Ok(line) => {
                let parsed = match Pesc::parse(&line) {
                    Ok(r) => r,
                    Err(e) => {
                        println!("error: {}", e);
                        continue;
                    },
                };

                match pesc.eval(&parsed.1) {
                    Ok(()) => (),
                    Err((b, e)) => {
                        println!("error: {}", e);
                        println!("problematic stack:");
                        output.format_stack(&b);
                        println!("\n\n");
                    },
                }

                output.format_stack(&pesc.stack);
            },
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) =>
                println!("Use Ctrl-D to quit."),
            Err(_) => output.format_stack(&pesc.stack),
        }
    }
}
