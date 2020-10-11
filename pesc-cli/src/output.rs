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

use std::time;
use pesc::pesc::*;

use crate::tty::{
    self, OutputStream,
    TermStyle, TermColor
};

const PADDING: usize = 3;
const MORE_STR: &'static str = " »";

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OutputMode {
    Human, Simple, Quiet
}

impl OutputMode {
    pub fn auto() -> OutputMode {
        if tty::is_tty(OutputStream::Stdout) {
            OutputMode::Human
        } else {
            OutputMode::Simple
        }
    }

    pub fn format_elapsed(&self, dur: time::Duration) {
        match self {
            OutputMode::Human => {
                println!("{g}{i}Done in {e:.2?}.{r}",
                    g = TermStyle::BrightFg(TermColor::Black),
                    i = TermStyle::Italic, r = TermStyle::Reset,
                    e = dur);
            },
            OutputMode::Simple => println!("elapsed: {:.2?}", dur),
            OutputMode::Quiet => (),
        }
    }

    pub fn format_stack(&self, stack: &Vec<PescToken>) {
        match self {
            OutputMode::Human => {
                if stack.len() == 0 {
                    println!("{g}(empty stack){r}",
                    g = TermStyle::BrightFg(TermColor::Black),
                    r = TermStyle::Reset);
                    return;
                }

                let max_sz = tty::tty_sz().0;
                let mut item_buf = String::new();
                let mut num_buf  = format!("{}",
                    TermStyle::BrightFg(TermColor::Black));
                let mut ctr = 0;

                let mut format_output = |i: &PescToken, ctr, first| -> bool {
                    let item_color = match i {
                        PescToken::Str(_) => TermStyle::Fg(TermColor::Cyan),
                        PescToken::Number(_) => TermStyle::BrightFg(TermColor::White),
                        PescToken::Macro(_) => TermStyle::Underline,
                        PescToken::Bool(_) => TermStyle::Fg(TermColor::Yellow),
                        _ => TermStyle::Fg(TermColor::White),
                    };

                    let fmt_item = format!("{g}[{r}{f}{c}{item:>0$}{r}{g}]{r}",
                        PADDING, c = item_color,
                        g = TermStyle::BrightFg(TermColor::Black),
                        r = TermStyle::Reset, item = i.to_string(),
                        f = if first { TermStyle::Bold } else { TermStyle::Reset });

                    if TermStyle::strip(&item_buf).len()
                        + TermStyle::strip(&fmt_item).len() + 1 >= max_sz {
                            item_buf += MORE_STR;
                            true
                    } else {
                        item_buf += &fmt_item;
                        num_buf  += &format!("{c:>0$}",
                            TermStyle::strip(&fmt_item).len(), c = &ctr);
                        false
                    }
                };

                // treat the first item in the stack specially
                format_output(&stack[stack.len() - 1], ctr, true);
                ctr += 1;

                // and the rest...
                for i in stack.iter().rev().skip(1) {
                    if format_output(i, ctr, false) {
                        break;
                    } else {
                        ctr += 1;
                    }
                }

                num_buf += &TermStyle::Reset.to_string();
                println!("{}\n{}", item_buf, num_buf);
            },
            OutputMode::Simple
            | OutputMode::Quiet => stack.iter()
                    .rev()
                    .for_each(|i| println!("{} ", i)),
        }
    }
}
