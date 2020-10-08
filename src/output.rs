use crate::pesc::*;
use crate::tty::{
    self, OutputStream,
    TermStyle, TermColor
};

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

                for i in stack.iter().rev() {
                    let item_color = match i {
                        PescToken::Str(_) => TermStyle::Fg(TermColor::Cyan),
                        PescToken::Number(_) => TermStyle::BrightFg(TermColor::White),
                        PescToken::Macro(_) => TermStyle::Underline,
                        PescToken::Bool(_) => TermStyle::Fg(TermColor::Yellow),
                        _ => TermStyle::Fg(TermColor::White),
                    };

                    let fmt_item = format!("{g}[{r}{c}{item:>0$}{r}{g}]{r}",
                        PADDING, c = item_color,
                        g = TermStyle::BrightFg(TermColor::Black),
                        r = TermStyle::Reset, item = i.to_string());

                    if TermStyle::strip(&item_buf).len()
                        + TermStyle::strip(&fmt_item).len() + 1 >= max_sz {
                            item_buf += " »";
                            break;
                    } else {
                        item_buf += &fmt_item;
                        num_buf  += &format!("{c:>0$}",
                            TermStyle::strip(&fmt_item).len(), c = ctr);
                        ctr += 1;
                    }
                }

                num_buf += "\x1b[m";
                println!("{}\n{}", item_buf, num_buf);
            },
            OutputMode::Machine => unimplemented!(),
            OutputMode::Simple
            | OutputMode::Quiet => stack.iter()
                    .rev()
                    .for_each(|i| println!("{} ", i)),
        }
    }
}
