use crate::pesc::*;
use crate::tty::{self, OutputStream};

const PADDING: usize = 11;

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
                if p.m_stack.len() == 0 {
                    println!("(empty stack)");
                    return;
                }

                let max_sz = tty::tty_sz().0;
                let mut item_buf = String::new();
                let mut num_buf  = String::new();
                let mut ctr = 0;

                for i in p.m_stack.iter().rev() {
                    let item = format!("[{item:>0$}]", PADDING,
                        item = i.to_string());
                    if item_buf.len() + item.len() + 1 >= max_sz {
                        item_buf = format!("{} »", item_buf);
                        break;
                    } else {
                        item_buf += &item;
                        num_buf  += &format!("{c:>0$}", item.len(),
                            c = ctr);
                        ctr += 1;
                    }
                }

                println!("{}\n{}", item_buf, num_buf);
            },
            OutputMode::Machine => unimplemented!(),
            OutputMode::Simple
            | OutputMode::Quiet => p.print(),
        }
    }
}
