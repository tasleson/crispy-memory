use libc;
use std::ffi::CString;
use std::os::raw::c_char;

/// File: lib.rs

/// For further reading ...
/// #[no_mangle] - // https://internals.rust-lang.org/t/precise-semantics-of-no-mangle/4098

#[no_mangle]
pub unsafe extern "C" fn get_some_cstr(desc: *mut *mut c_char) -> isize {
    // We want the pointer coming in to not be null and not currently be pointing to something
    // to prevent whatever it's pointing to be lost.
    if desc.is_null() || !(*desc).is_null() {
        return libc::EINVAL as isize;
    }

    let val = CString::new("Returning a string to C to be free() there")
        .expect("Expecting we can allocate a CString");

    // Allocate memory for string as C caller is expected to "free" it.
    // This approach seems to be the safest way to do this, so that you can be certain
    // that the memory is allocated with the same allocator as what the caller will be using to
    // "free" it.  In general having a library which allocates things on the heap and expects the
    // caller to free it is probably not the best thing to do.
    let m = libc::malloc(libc::strlen(val.as_ptr()) + 1) as *mut c_char;
    if m.is_null() {
        return libc::ENOMEM as isize;
    }

    *desc = m;
    libc::strcpy(*desc, val.as_ptr());
    0
}

#[no_mangle]
pub unsafe extern "C" fn get_some_cstr_2(desc: *mut *mut c_char) -> isize {
    // We want the pointer coming in to not be null and not currently be pointing to something
    // to prevent whatever it's pointing to be lost.
    if desc.is_null() || !(*desc).is_null() {
        return libc::EINVAL as isize;
    }

    let val = CString::new("Returning a string to C to be free() there")
        .expect("Expecting we can allocate a CString");

    // The documentation states that the pointer from "into_raw()"
    // needs to be brought back into rust and reconstructed as a CString to be freed
    // correctly, so this example is suppose to result into a memory leak if the memory
    // is released with free().  This is stated because you need to ensure the allocator &
    // de-allocator are the same.
    *desc = val.into_raw();
    0
}

/// Our implementation of the Error type.
pub struct Error {
    magic: u32,
    msg: CString,
    code: isize,
}

///  Some C code uses magic values in structures to determine if the pointer
/// is of the correct type.
const ERROR_MAGIC: u32 = 0xDEADBEEF;

// A function which creates an example Error
fn example_error() -> Error {
    Error {
        magic: ERROR_MAGIC,
        msg: CString::new("Some helpful error message").unwrap(),
        code: -101,
    }
}

/// Adding this so that we can get a message printed when the Error is freed.
impl Drop for Error {
    fn drop(&mut self) {
        println!("Error struct being dropped ...");
    }
}

#[no_mangle]
pub unsafe extern "C" fn error_create_with_result(_e: *mut *mut Error) -> isize {
    let e = Box::new(example_error());
    *_e = Box::into_raw(e);
    0
}

#[no_mangle]
pub unsafe extern "C" fn error_free_with_result(e: *mut *mut Error) -> i32 {
    if e.is_null() || (*e).is_null() {
        return libc::EINVAL;
    }

    // Try to ensure we have a pointer to the correct structure...
    if (*(*e)).magic != ERROR_MAGIC {
        return libc::EINVAL;
    }

    // Reconstruct the Error into a box and then drop it so that it's freed.
    drop(Box::from_raw(*e));
    *e = 0 as *mut Error;
    0
}

/// The next two function examples are taken directly out of the rust documentation
/// ref. https://doc.rust-lang.org/std/boxed/
#[no_mangle]
pub extern "C" fn error_new() -> Box<Error> {
    Box::new(example_error())
}

/// We take ownership as we are passing by value, so when function
/// exits the drop gets run.  Handles being passed null.
#[no_mangle]
pub extern "C" fn error_free(_: Option<Box<Error>>) {}

/// Our example "getter" methods which work on the Error type.  The value
/// returned is only valid as long as the Error has not been freed.  If C
/// caller needs a longer lifetime they need to copy the value.
#[no_mangle]
pub unsafe extern "C" fn error_msg_get(e: &Error) -> *const c_char {
    e.msg.as_ptr()
}

#[no_mangle]
pub extern "C" fn error_code_get(e: &Error) -> isize {
    e.code
}
