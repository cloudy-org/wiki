use std::ffi::c_int;

#[unsafe(no_mangle)]
pub extern "C" fn add(left: c_int, right: c_int) -> c_int {
    left + right
}