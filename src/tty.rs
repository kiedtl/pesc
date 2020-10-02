use std::os::raw::c_int;
use terminal_size::{Width, Height, terminal_size};

extern "C" {
    pub fn isatty(fd: c_int) -> c_int;
}

pub enum OutputStream {
    Stdout,
    Stderr,
    Stdin,
    Other(usize),
}

impl Into<c_int> for OutputStream {
    fn into(self) -> c_int {
        match self {
            OutputStream::Stdout   => 1 as c_int,
            OutputStream::Stderr   => 2 as c_int,
            OutputStream::Other(f) => f as c_int,
            OutputStream::Stdin    => 0 as c_int,
        }
    }
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
