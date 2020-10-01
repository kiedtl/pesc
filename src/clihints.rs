// the following code was proudly stolen
// from the `diy_hint` example in rustyline's source.

use std::collections::HashSet;
use crate::pesc::Pesc;
use rustyline::{
    hint::Hinter, Context
};
use rustyline_derive::{
    Completer, Helper,
    Highlighter, Validator
};

#[derive(Completer, Helper, Validator, Highlighter)]
pub struct CommandHinter {
    // TODO: use ** radix trie **
    hints: HashSet<String>,
}

impl CommandHinter {
    pub fn new(hints: HashSet<String>) -> Self {
        Self { hints: hints }
    }
}

impl Hinter for CommandHinter {
    fn hint(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Option<String> {
        if pos < line.len() {
            return None;
        }

        self.hints
            .iter()
            .filter_map(|hint| {
                // expect hint after word complete, like redis cli, add condition:
                // line.ends_with(" ")
                if pos > 0 && hint.starts_with(&line[..pos]) {
                    Some(hint[pos..].to_owned())
                } else {
                    None
                }
            })
            .next()
    }
}

pub fn hints(p: &Pesc) -> HashSet<String> {
    let mut set = HashSet::new();

    // some hints ;)
    set.insert(String::from("help // read the manpage, dummy"));
    set.insert(String::from("man  // `man pesc`, if your package manager's worth anything."));
    set.insert(String::from("man pesc // no, you need to exit first."));
    set.insert(String::from("quit // just hit Ctrl-D"));
    set.insert(String::from("exit // it's ^D to quit. Ctrl-D."));
    set.insert(String::from("nice // gee, thanks"));
    set.insert(String::from("lol  // what's so funny?"));
    set.insert(String::from("what // maybe you need to take a look at the manpage"));

    p.funcs.iter().for_each(|f| {
        set.insert(format!("[{}]", f.0));
    });
    set
}
