use proc_macro::TokenStream;
use std::ffi::c_char;

unsafe extern "C" {
    fn printf(f: *const c_char, ...) -> i32;
}

// hacky way of using macros, returned token doesnt matter here
#[proc_macro_attribute]
pub fn xy(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: {}", attr);
    println!("item: {}", item);
    let binding = attr.to_string();
    if !binding.contains(",") {
        panic!("Wrong syntax! use (x = someint, y = otherint)");
    }
    let attr: Vec<&str> = binding.split(",").collect();
    let x = attr[0];
    let y = attr[1];
    println!("x is {}", x);
    println!("y is {}", y);
    let mut rhs = String::new();
    let mut lhs = String::new();

    for c in x.chars() {
        if c.is_digit(10) {
            rhs.push(c);
        }
    }
    for c in y.chars() {
        if c.is_digit(10) {
            lhs.push(c);
        }
    }

    assert!(!rhs.is_empty());
    assert!(!lhs.is_empty());
    unsafe {
        printf("\033[%d;%dH".as_ptr() as *const i8, lhs, rhs);
        printf("im here".as_ptr() as *const i8);
    }

    item
}
