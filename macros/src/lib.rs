//!NOTE: nested unsafe so that we dont have to wrap every macro call inside of an unsafe block.
//!
#[macro_export]
macro_rules! xyprint {
    ($x:expr, $y:expr, $fmt:expr) => {
        unsafe {
            unsafe extern "C" {
                fn printf(f: *const i8, ...) -> i32;
            }
            printf("\x1B[%d;%dH\0".as_ptr() as *const i8, $y, $x);
            let cstr = std::ffi::CString::new($fmt).unwrap();
            printf("%s\0".as_ptr() as *const i8, cstr.as_ptr());
        }
    };
}
#[macro_export]
macro_rules! clear {
    () => {
        unsafe {
            unsafe extern "C" {
                fn printf(f: *const i8, ...) -> i32;
            }
            printf("\x1B[H\x1B[J".as_ptr() as *const i8);
        }
    };
}
