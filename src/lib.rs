use std::ffi::CStr;
use std::os::raw::c_char;

#[no_mangle]
pub extern "C" fn hello(s: *const c_char) {
    let c = how_many_characters(s);
    println!("{} days", c);
}

#[no_mangle]
pub extern "C" fn how_many_characters(s: *const c_char) -> u32 {
    let c_str = unsafe {
        assert!(!s.is_null());

        CStr::from_ptr(s)
    };

    let r_str = c_str.to_str().unwrap();
    r_str.chars().count() as u32
}

extern crate cljjnarustlib;

#[no_mangle]
pub extern "C" fn run() {
    println!("{} days cljjnarustclib", 8);

    cljjnarustlib::run().unwrap();
}

#[no_mangle]
pub extern "C" fn update_world(i: usize, x: i32, y: i32, t: i32, step: i32) {
    cljjnarustlib::update_world(i, x, y, t, step);
}

#[no_mangle]
pub extern "C" fn add_object(x: i32, y: i32, t: i32, step: i32) -> usize {
    cljjnarustlib::add_object(x, y, t, step)
}

#[no_mangle]
pub extern "C" fn add_new_object(x: i32, y: i32, t_cstr_ptr: *const c_char, step: i32) -> usize {
    let t_cstr = unsafe {
        assert!(!t_cstr_ptr.is_null());

        CStr::from_ptr(t_cstr_ptr)
    };

    let t = t_cstr.to_str().unwrap();

    cljjnarustlib::add_new_object(x, y, t, step)
}

#[no_mangle]
pub extern "C" fn move_new_object(i: usize, x: i32, y: i32) {
    cljjnarustlib::move_new_object(i, x, y);
}

#[no_mangle]
pub extern "C" fn get_input_state(is: &mut cljjnarustlib::InputState) {
    cljjnarustlib::get_input_state(is);
}
