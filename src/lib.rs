mod utils;
extern crate serde_json;

use wasm_bindgen::prelude::*;

// #[macro_use]
// extern crate serde_derive;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn solve(input: JsValue) -> String {
    utils::set_panic_hook();

    let elements: Vec<u8> = input.into_serde().unwrap();

    let orig_elements = elements.clone();

    let input_str: String = orig_elements.into_iter().map(|i| String::new() + &i.to_string() + ", ").collect();

    let transformed_elements = elements.into_iter().map(|i| i.pow(2));

    // let s1 = match std::str::from_utf8(&elements) {
    //     Ok(v) => v,
    //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    // };
    // let s1 = String::from_utf8_lossy(vec!(elements));
    // String::new() + "Hello, " + name + "!"
    let s1: String = transformed_elements.into_iter().map(|i| String::new() + &i.to_string() + ", ").collect();

    let s = &format!("Solving: {} Result: {}", input_str, s1);

    s.to_string()
}
