use proc_macro::TokenStream;
use std::ffi::c_char;
unsafe extern "C" {
    fn printf(f: *const c_char, ...) -> i32;
}

#[proc_macro_attribute]
pub fn testing(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr: {}", attr);
    println!("item: {}", item);
    item
}
