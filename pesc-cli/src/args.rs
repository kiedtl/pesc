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

use crate::output::*;
use getopts::Options as g_Options;
use std::env;

#[derive(Clone, Debug)]
pub struct Options {
    pub file: Option<String>,
    pub output: OutputMode,
    pub verbose: bool,
}

impl Options {
    // set default values of options
    pub fn new() -> Self {
        Self {
            file: None,
            output: OutputMode::auto(),
            verbose: false,
        }
    }

    pub fn parse(mut self) -> Result<Self, ()> {
        let args: Vec<String> = env::args().collect();
        let argv0 = args[0].clone();

        let mut opts = g_Options::new();

        opts.optflag("h", "help", "print this help message.");
        opts.optflag("V", "version", "print the version.");
        opts.optflag("q", "quiet", "reduce output.");
        opts.optflag("v", "verbose", "show elapsed time.");

        let matches = match opts.parse(&args[1..]) {
            Ok(ma) => ma,
            Err(e) => {
                println!("pesc: error: {}", e);
                return Err(());
            },
        };

        if matches.opt_present("h") {
            Options::usage(&argv0);
            return Err(());
        } else if matches.opt_present("V") {
            Options::version();
            return Err(());
        }

        self.file = if !matches.free.is_empty() {
            Some(matches.free[0].clone())
        } else {
            None
        };

        self.verbose = matches.opt_present("v");

        self.output = {
            // if -q is set, force quiet mode
            if matches.opt_present("q") {
                OutputMode::Quiet
            } else {
                // default to the previous value,
                // which is set automatically based on
                // whether stdout is a tty or not
                self.output
            }
        };

        Ok(self)
    }

    fn usage(argv0: &str) {
        println!("Usage: {} [OPTION]... [FILE]
Copyright (c) 2020 Kiëd Llaentenn

Options:
    -h, --help             Print a this help message and exit.
    -V, --version          Print pescli's version and exit.
    -q, --quiet            Print as little information as possible.

Full documentation is available as a manpage (pescli(1)).
Source: https://github.com/lptstr/pesc
Reporting issues: https://github.com/lptstr/pesc/issues/new
", argv0);
    }

    fn version() {
        println!("pescli v{}", crate::VERSION);
    }
}
