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

// NOTE: items are commented out to avoid cluttering my terminal
// with "vArIaNt Is NeVeR cOnStRuCtEd" warnings.

use std::fmt;
use std::os::raw::c_int;
use std::result::Result;
use terminal_size::{Width, Height, terminal_size};

#[derive(Copy, Clone, Debug)]
pub enum TermStyle {
    Bold,
    Underline,
    Italic,
    //Inverted,
    //Blink,
    //Strike,
    Reset,

    //Bg(TermColor),
    //BrightBg(TermColor),
    Fg(TermColor),
    BrightFg(TermColor),
}

impl TermStyle {
    pub fn strip(s: &str) -> String {
        // TODO: cleanup
        let input = s.clone().chars()
            .collect::<Vec<char>>();
        let mut buf = Vec::new();

        let mut c = 0;
        while c < input.len() {
            // use a `while` loop instead of an `if` statement,
            // to handle the case of having two escape sequences
            // right next to each other
            while c < input.len() && input[c] == '\x1b' {
                while input[c] != 'm' && c < input.len() {
                    c += 1;
                }

                c += 1;
            }

            if c < input.len() {
                buf.push(input[c]);
                c += 1;
            }
        }

        buf.iter().collect::<String>()
    }
}

impl fmt::Display for TermStyle {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let r = match self {
            TermStyle::Bold => String::from("\x1b[1m"),
            TermStyle::Underline => String::from("\x1b[4m"),
            TermStyle::Italic => String::from("\x1b[3m"),
            //TermStyle::Inverted => String::from("\x1b[7m"),
            //TermStyle::Blink => String::from("\x1b[5m"),
            //TermStyle::Strike => String::from("\x1b[9m"),
            TermStyle::Reset => String::from("\x1b[m"),

            //TermStyle::Bg(c) => format!("\x1b[4{}m", c),
            //TermStyle::BrightBg(c) => format!("\x1b[10{}m", c),
            TermStyle::Fg(c) => format!("\x1b[3{}m", c),
            TermStyle::BrightFg(c) => format!("\x1b[9{}m", c),
        };

        write!(f, "{}", r)
    }
}

#[derive(Copy, Clone, Debug)]
pub enum TermColor {
    Black,
    //Red,
    //Green,
    Yellow,
    //Blue,
    //Magenta,
    Cyan,
    White,
}

impl fmt::Display for TermColor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        let r = match self {
            TermColor::Black        => "0",
            //TermColor::Red          => "1",
            //TermColor::Green        => "2",
            TermColor::Yellow       => "3",
            //TermColor::Blue         => "4",
            //TermColor::Magenta      => "5",
            TermColor::Cyan         => "6",
            TermColor::White        => "7",
        };

        write!(f, "{}", r)
    }
}

pub enum OutputStream {
    Stdout,

    // listen, rustc, I understand they're not being
    // used right now, but you really don't need to raise
    // a fuss about that now

    //Stderr,
    //Stdin,
    //Other(usize),
}

impl Into<c_int> for OutputStream {
    fn into(self) -> c_int {
        match self {
            OutputStream::Stdout   => 1 as c_int,
            //OutputStream::Stderr   => 2 as c_int,
            //OutputStream::Other(f) => f as c_int,
            //OutputStream::Stdin    => 0 as c_int,
        }
    }
}

extern "C" {
    pub fn isatty(fd: c_int) -> c_int;
}

pub fn is_tty(fd: OutputStream) -> bool {
    let r = unsafe { isatty(fd.into()) };
    r != 0
}

pub fn tty_sz() -> (usize, usize) {
    if let Some((Width(w), Height(h))) = terminal_size() {
        (w as usize, h as usize)
    } else {
        (80, 24)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip() {
        assert_eq!(&TermStyle::strip("Thi\x1b[0ms is a test"), "This is a test");
        assert_eq!(&TermStyle::strip("Thi\x1b[1ms is a test"), "This is a test");
        assert_eq!(&TermStyle::strip("Thi\x1b[107ms is a test\x1b[0m"), "This is a test");
        assert_eq!(&TermStyle::strip("Thi\x1b[107ms test has some \x1b[1mmmms \x1b[0m"), "This test has some mmms ");
        assert_eq!(&TermStyle::strip("Thi\x1b[107ms test \x1b[mhas some \x1b[1mmmms \x1b[0m"), "This test has some mmms ");
        assert_eq!(&TermStyle::strip("\x1b[90m[\x1b[m     0\x1b[90m]\x1b[m"), "[     0]");
        assert_eq!(&TermStyle::strip("\u{1b}[90m[\u{1b}[m          8\u{1b}[90m]\u{1b}[m\u{1b}[90m[\u{1b}[m          3\u{1b}[90m]\u{1b}[m"), "[          8][          3]");
    }
}
