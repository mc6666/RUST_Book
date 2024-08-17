use std::{ffi::{CString, CStr, c_char, c_int}, error::Error};

#[link(name = "c_cpp")]  // name of the C shared library
extern "C" {
    fn introduce(name: *const c_char, age: c_int) -> *const c_char;

    fn deallocate_string(s: *const c_char);
}

fn get_str_slice_from_c_char_ptr<'a>(input_c_char: *const c_char) -> Result<&'a str, Box<dyn Error>> {
    let input_c_str = unsafe { CStr::from_ptr(input_c_char) };
    match input_c_str.to_str() {
        Ok(str_slice) => Ok(str_slice),
        Err(error) => Err(Box::new(error)),
    }
}

pub fn introduce_rust(name: &str, age: c_int) -> Result<String, Box<dyn Error>> {
    let name = CString::new(name)?;
    let introduction = unsafe { introduce(name.as_ptr(), age) };
    let introduction_string = get_str_slice_from_c_char_ptr(introduction)?.to_string();
    unsafe { deallocate_string(introduction) };
    Ok(introduction_string)
}
