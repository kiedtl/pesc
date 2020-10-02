use crate::pesc::*;
use crate::tty::{self, OutputStream};

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OutputMode {
    Human, Machine, Simple, Quiet
}

impl OutputMode {
    pub fn auto() -> OutputMode {
        if tty::is_tty(OutputStream::Stdout) {
            OutputMode::Human
        } else {
            OutputMode::Simple
        }
    }

    pub fn format_stack(&self, p: &Pesc) {
        match self {
            OutputMode::Human => {
                let max_sz = tty::tty_sz().0;
                let mut buf = String::new();

                for i in p.m_stack.iter().rev() {
                    if buf.len() + i.to_string().len() + 1
                        >= max_sz {
                            buf = format!("{} »", buf);
                            break;
                    }

                    buf = format!("{}{} ", buf, i);
                }

                println!("{}", buf);
            },
            OutputMode::Machine => unimplemented!(),
            OutputMode::Simple
            | OutputMode::Quiet => p.print(),
        }
    }
}
