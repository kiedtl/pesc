/*
 * pescli - a CLI Pesc calculator
 * Copyright (C) 2020 Kiëd Llaentenn <kiedtl [at] protonmail [dot] com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
*/

mod args;
mod clihints;
mod tty;
mod output;

pub const VERSION: &'static str = "0.1.0";

use pesc::pesc::*;
use pesc::stdlib;

use crate::clihints::*;
use crate::args::*;

use std::time::Instant;

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
            Ok(()) => opts.output.format_stack(&pesc.stack),
            Err((_, e)) => {
                println!("pesc: error: {}", e);
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

    let mut rl = Editor::<BustyLine>::with_config(config);
    rl.set_helper(Some(BustyLine::new()));

    loop {
        match rl.readline("pesc> ") {
            Ok(line) => {
                let now = Instant::now();

                let parsed = match Pesc::parse(&line) {
                    Ok(r) => r,
                    Err(e) => {
                        println!("error: {}", e);
                        continue;
                    },
                };

                match pesc.eval(&parsed.1) {
                    Ok(()) => (),
                    Err((_, e)) => {
                        println!("error: {}", e);
                    },
                }

                opts.output.format_stack(&pesc.stack);

                if opts.verbose {
                    println!();
                    opts.output.format_elapsed(now.elapsed());
                }
            },
            Err(ReadlineError::Eof) => break,
            Err(ReadlineError::Interrupted) =>
                println!("Use Ctrl-D to quit."),
            Err(_) => opts.output.format_stack(&pesc.stack),
        }
    }
}
