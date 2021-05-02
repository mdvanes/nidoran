mod utils;
extern crate serde_json;
extern crate web_sys;

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

// https://www.google.com/imgres?imgurl=https%3A%2F%2Fi.pinimg.com%2Foriginals%2F8f%2Fe8%2F82%2F8fe8826266ed01e1e7e5a4ab3e31e9f5.jpg&imgrefurl=https%3A%2F%2Fnl.pinterest.com%2Fpin%2F251920172889908428%2F&tbnid=H03lYq_8QYUVVM&vet=12ahUKEwjL4YLz9J3uAhVHP-wKHY5OA2QQMygAegUIARCpAQ..i&docid=fBfHotkgkE3z2M&w=200&h=200&itg=1&q=very%20easy%20sudoku&ved=2ahUKEwjL4YLz9J3uAhVHP-wKHY5OA2QQMygAegUIARCpAQ
// const matrix = [[0, 0, 1, 0, 0, 4, 0, 0, 2], [0, 5, 0, 0, 0, 3, 0, 1, 9], [4, 7, 0, 0, 0, 0, 0, 0, 5], [0, 0, 0, 0, 8, 0, 2, 0, 7], [0, 0, 4, 0, 9, 0, 8, 0, 0], [8, 0, 6, 0, 3, 0, 0, 0, 0], [2, 0, 0, 0, 0, 0, 0, 6, 8], [1, 8, 0, 2, 0, 0, 0, 4, 0], [5, 0, 0, 3, 0, 0, 9, 0, 0]];

// #[wasm_bindgen]
// pub fn printMatrix = (m: )=>{
//     return m.map(row=>{
//         return row.map(cell=>cell ? `${cell}` : " ").join(" ");
//     }
//     ).join("\n");
// }

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn solve(input: JsValue) -> String {
    utils::set_panic_hook();

    // let v2 = vec![1; 10];
    // This does not work when trying to run in the browser, maybe when running on CLI: println!("{:?}", v2);
    web_sys::console::log_1(&"Start Nidoran".into());
    // web_sys::console::log("v2");
    // utils.log!("oh bla bla {:?}", v2);

    // Vec is growable, array is fixed size
    // let some_matrix: Vec<Vec<u8>> = [];
    // let some_matrix: [i32; 3] = [9, 8, 7];
    // log!("some_matrix {:?}", some_matrix);
    let matrix: [[i32; 9]; 9] = [[0, 0, 1, 0, 0, 4, 0, 0, 2], [0, 5, 0, 0, 0, 3, 0, 1, 9], [4, 7, 0, 0, 0, 0, 0, 0, 5], [0, 0, 0, 0, 8, 0, 2, 0, 7], [0, 0, 4, 0, 9, 0, 8, 0, 0], [8, 0, 6, 0, 3, 0, 0, 0, 0], [2, 0, 0, 0, 0, 0, 0, 6, 8], [1, 8, 0, 2, 0, 0, 0, 4, 0], [5, 0, 0, 3, 0, 0, 9, 0, 0]];
    log!("matrix {:?}", matrix);

    let elements: Vec<u8> = input.into_serde().unwrap();
    log!("original elements {:?}", elements);

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
