use crate::output::*;
use getopts::Options as g_Options;
use std::env;

#[derive(Copy, Clone, Debug)]
pub enum Colors {
    Never,
    Auto,
    Always
}

#[derive(Clone, Debug)]
pub struct Options {
    pub file: Option<String>,
    pub force_interactive: bool,
    pub load_lua: bool,
    pub load_extra: Option<String>,
    pub colors: Colors,
    pub output: OutputMode,
}

impl Options {
    // set default values of options
    pub fn new() -> Self {
        Self {
            file: None,
            force_interactive: false,
            load_lua: false,
            load_extra: None,
            colors: Colors::Auto,
            output: OutputMode::auto(),
        }
    }

    pub fn parse(mut self) -> Result<Self, ()> {
        let args: Vec<String> = env::args().collect();
        let argv0 = args[0].clone();

        let mut opts = g_Options::new();

        opts.optflag("h", "help", "print this help message.");
        opts.optflag("V", "version", "print the version.");
        opts.optflag("q", "quiet", "reduce output.");
        opts.optflag("i", "", "force interactive mode.");
        opts.optflag("l", "load", "load extended stdlib from $PESCLIBS.");

        opts.optopt("L", "lua", "load the Lua file(s) in <PATH>.",
            "PATH");
        opts.optopt("", "color", "use colors: 'never', 'auto', 'always'.",
            "WHEN");

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
            // TODO
            todo!();
        }

        self.file = if !matches.free.is_empty() {
            Some(matches.free[0].clone())
        } else {
            None
        };

        self.force_interactive = matches.opt_present("i");
        self.load_lua = matches.opt_present("l");
        self.load_extra = matches.opt_str("L");

        self.colors = match matches.opt_str("color") {
            None => self.colors,
            Some(c) => match c.as_str() {
                "never" => Colors::Never,
                "auto" => Colors::Auto,
                "always" => Colors::Always,
                _ => {
                    println!("pesc: error: '{}': invalid option for --colors", c);
                    return Err(());
                },
            },
        };

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

Options:
    -h, --help             print this help message.
    -V, --version          print the version.
    -q, --quiet            reduce output.
    -i                     force interactive mode.
    -l, --load             load extended stdlib from $PESCLIBS.
    -j, --json             print the stack in JSON.
    -f, --file    [FILE]   execute <FILE>, print the result, and exit.
    -s, --save    [FILE]   save pesc's state in <FILE> before exiting.
    -r, --restore [FILE]   restore pesc's state from <FILE>.
    -L, --lua     [PATH]   load the Lua file(s) in <PATH>.
        --color   [WHEN]   use colors: 'never', 'auto', 'always'.
", argv0);
    }
}
