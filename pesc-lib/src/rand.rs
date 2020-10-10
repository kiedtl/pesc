use std::os::raw::c_int;

extern "C" {
    pub fn lrand48() -> c_int;
}
