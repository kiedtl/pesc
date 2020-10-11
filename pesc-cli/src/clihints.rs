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

// TODO: choose a more accurate name for this module

use pesc::pesc::Pesc;
use pesc::errors::*;

use crate::tty::*;

use rustyline::error::ReadlineError;
use rustyline::{
    hint::{
        Hinter,
        HistoryHinter
    }, Context,
    validate::{
        ValidationContext, Validator,
        ValidationResult::{
            self, Incomplete, Valid
        },
    },
    highlight::Highlighter,
};
use rustyline_derive::{
    Completer, Helper,
};

use std::borrow::{Cow, Cow::Owned};

#[derive(Completer, Helper)]
pub struct BustyLine {
    hinter: HistoryHinter,
}

impl BustyLine {
    pub fn new() -> Self {
        Self {
            hinter: HistoryHinter {},
        }
    }
}

impl Validator for BustyLine {
    fn validate(&self, ctx: &mut ValidationContext)
        -> Result<ValidationResult, ReadlineError>
    {
        let input = ctx.input();

        match Pesc::parse(&input) {
            Ok(_) => Ok(Valid(None)),
            Err(e) => {
                if let PescErrorType::UnmatchedToken(_) = e.kind {
                    Ok(Incomplete)
                } else {
                    Ok(Valid(None))
                }
            },
        }
    }
}

impl Hinter for BustyLine {
    fn hint(&self, line: &str, pos: usize, ctx: &Context<'_>) -> Option<String> {
        self.hinter.hint(line, pos, ctx)
    }
}

impl Highlighter for BustyLine {
    fn highlight_hint<'h>(&self, hint: &'h str) -> Cow<'h, str> {
        Owned(format!("{}{}{}", TermStyle::BrightFg(TermColor::Black),
            hint, TermStyle::Reset))
    }

    fn highlight<'l>(&self, line: &'l str, _: usize) -> Cow<'l, str> {
        Owned(line.to_owned())
    }
}
