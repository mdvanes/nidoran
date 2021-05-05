mod utils;
extern crate generic_array;
extern crate serde_json;
extern crate web_sys;

use generic_array::arr;
use generic_array::functional::*;
use generic_array::sequence::*;
use generic_array::typenum::consts::U1;
use generic_array::typenum::consts::U2;
use generic_array::typenum::consts::U9;
use generic_array::{ArrayLength, GenericArray};
use wasm_bindgen::prelude::*;

// #[macro_use]
// extern crate serde_derive;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

// Pseudo code from JS
// pub fn printMatrix = (m: )=>{
//     return m.map(row=>{
//         return row.map(cell=>cell ? `${cell}` : " ").join(" ");
//     }
//     ).join("\n");
// }
fn format_matrix_old(m: [[i8; 9]; 9]) -> String {
    let mut owned_string: String = String::new(); // "".to_owned();
                                                  // let borrowed_string: &str = "world";

    // for x in &m[0] {
    //     owned_string.push_str(&(String::new() + &x.to_string() + ", "));
    // }
    let mut xcounter = 0;

    for x in &m {
        xcounter += 1;
        let mut ycounter = 0;
        for y in x {
            ycounter += 1;
            // owned_string.push_str(&(String::new() + &y.to_string() + ", "));
            owned_string.push_str(&y.to_string());
            if ycounter != x.len() {
                owned_string.push_str(", ");
            }
        }
        if xcounter != m.len() {
            owned_string.push_str("\n");
        }
    }

    // There is no semi colon on the end of this line, making it the "tail", which is returned
    owned_string.to_string()

    // TODO array.map is experimental? Reimplement this with: map & "join", see JS implementation
    // let s = &m[0].map(|i| String::new() + &i.to_string() + ", ").collect();
    // s.to_string()
}

fn intsToStrings(x: [i8; 2]) -> GenericArray<String, U2> {
    let z = GenericArray::from_slice(&x);
    z.map(|y| y.to_string())
    // z.map(|y| y + 1)
}

fn toGenArr(arr: &[i8; 9]) -> GenericArray<i8, U9> {
    *GenericArray::from_slice(arr)
}

// TODO see https://github.com/fizyk20/generic-array/blob/master/DESIGN.md
// Note: || func is a closure

// TODO convert i8 to u8?
fn format_matrix(m: [[i8; 9]; 9]) -> String {
    // let a = (arr![i32; 1, 2, 3]).map(|x| x + 1);
    // let b: [i32; 3] = [1, 2, 3];
    // let c: [i8; 3] = [1, 2, 3];
    // let b = [i32; c];
    // let row1: &GenericArray<i8, U9> = GenericArray::from_slice(&m[0]);
    // // let a = (arr!b).map(|x| x + 1);
    // let row1_mapped = row1
    //     .map(|x| x.to_string())
    //     .fold(String::new(), |acc, next| acc + ", " + &next);
    // log!("row1_mapped {:?}", row1_mapped);

    // let test_multi_arr: GenericArray<i8, U2> = arr![i8; 1, 2];
    // let test_multi_arr1: GenericArray<[i8; 2], U2> = arr![[i8; 2]; [1, 2], [3, 4]];
    // log!("test_multi_arr1 {:?}", test_multi_arr1);
    // let test_multi_arr1_mapped = test_multi_arr1.map(intsToStrings);
    // log!("test_multi_arr1_mapped {:?}", test_multi_arr1_mapped);

    let m_gen_arr: &GenericArray<[i8; 9], U9> = GenericArray::from_slice(&m);
    log!("m_gen_arr {:?}", m_gen_arr);
    let m_deep_gen_arr = m_gen_arr.map(toGenArr);
    log!("m_deep_gen_arr {:?}", m_deep_gen_arr);
    let m_strings = m_deep_gen_arr.map(|row| row.map(|cell| cell.to_string()));
    log!("m_strings {:?}", m_strings);
    // let m1_mapped = row1.map(|x| x.map(|y| y.to_string()));
    // //.fold(String::new(), |acc, next| acc + ", " + &next);
    // log!("m1_mapped {:?}", m1_mapped);

    let mut owned_string: String = String::new(); // "".to_owned();
                                                  // let borrowed_string: &str = "world";

    // for x in &m[0] {
    //     owned_string.push_str(&(String::new() + &x.to_string() + ", "));
    // }
    let mut xcounter = 0;

    for x in &m {
        xcounter += 1;
        let mut ycounter = 0;
        for y in x {
            ycounter += 1;
            // owned_string.push_str(&(String::new() + &y.to_string() + ", "));
            owned_string.push_str(&y.to_string());
            if ycounter != x.len() {
                owned_string.push_str(", ");
            }
        }
        if xcounter != m.len() {
            owned_string.push_str("\n");
        }
    }

    // There is no semi colon on the end of this line, making it the "tail", which is returned
    owned_string.to_string()

    // TODO array.map is experimental? Reimplement this with: map & "join", see JS implementation
    // let s = &m[0].map(|i| String::new() + &i.to_string() + ", ").collect();
    // s.to_string()
}

#[wasm_bindgen]
pub fn solve(input: JsValue) -> String {
    utils::set_panic_hook();

    // Take `matrix` from `input`
    // let some_matrix: [i8; 3] = input.into_serde().unwrap(); // [0; 3]; //Array::new();
    // let some_matrix: [[i8; 9]; 9] = input.into_serde().unwrap(); // [0; 3]; //Array::new();
    // some_matrix
    // log!("some_matrix {:?}", some_matrix);

    // let v2 = vec![1; 10];
    // This does not work when trying to run in the browser, maybe when running on CLI: println!("{:?}", v2);
    web_sys::console::log_1(&"Start Nidoran".into());
    // web_sys::console::log("v2");
    // utils.log!("oh bla bla {:?}", v2);

    // Vec is growable, array is fixed size
    // let some_matrix: Vec<Vec<u8>> = [];
    // let some_matrix: [i32; 3] = [9, 8, 7];
    // log!("some_matrix {:?}", some_matrix);
    // https://www.google.com/imgres?imgurl=https%3A%2F%2Fi.pinimg.com%2Foriginals%2F8f%2Fe8%2F82%2F8fe8826266ed01e1e7e5a4ab3e31e9f5.jpg&imgrefurl=https%3A%2F%2Fnl.pinterest.com%2Fpin%2F251920172889908428%2F&tbnid=H03lYq_8QYUVVM&vet=12ahUKEwjL4YLz9J3uAhVHP-wKHY5OA2QQMygAegUIARCpAQ..i&docid=fBfHotkgkE3z2M&w=200&h=200&itg=1&q=very%20easy%20sudoku&ved=2ahUKEwjL4YLz9J3uAhVHP-wKHY5OA2QQMygAegUIARCpAQ
    // let matrix: [[i8; 9]; 9] = [
    //     [0, 0, 1, 0, 0, 4, 0, 0, 2],
    //     [0, 5, 0, 0, 0, 3, 0, 1, 9],
    //     [4, 7, 0, 0, 0, 0, 0, 0, 5],
    //     [0, 0, 0, 0, 8, 0, 2, 0, 7],
    //     [0, 0, 4, 0, 9, 0, 8, 0, 0],
    //     [8, 0, 6, 0, 3, 0, 0, 0, 0],
    //     [2, 0, 0, 0, 0, 0, 0, 6, 8],
    //     [1, 8, 0, 2, 0, 0, 0, 4, 0],
    //     [5, 0, 0, 3, 0, 0, 9, 0, 0],
    // ];
    // TODO This works, but panics when the input does not contain [[i8; 9]; 9]
    let matrix: [[i8; 9]; 9] = input.into_serde().unwrap();
    // log!("matrix {:?}", matrix);
    log!("matrix {:?}", format_matrix(matrix));

    // TODO const matrix = getPossibleValues(initialMatrix);

    format_matrix(matrix)
}

#[wasm_bindgen]
pub fn solve_old(input: JsValue) -> String {
    utils::set_panic_hook();

    let elements: Vec<u8> = input.into_serde().unwrap();
    log!("original elements {:?}", elements);

    let orig_elements = elements.clone();

    let input_str: String = orig_elements
        .into_iter()
        .map(|i| String::new() + &i.to_string() + ", ")
        .collect();

    let transformed_elements = elements.into_iter().map(|i| i.pow(2));

    // let s1 = match std::str::from_utf8(&elements) {
    //     Ok(v) => v,
    //     Err(e) => panic!("Invalid UTF-8 sequence: {}", e),
    // };
    // let s1 = String::from_utf8_lossy(vec!(elements));
    // String::new() + "Hello, " + name + "!"
    let s1: String = transformed_elements
        .into_iter()
        .map(|i| String::new() + &i.to_string() + ", ")
        .collect();

    let s = &format!("Solving: {} Result: {}", input_str, s1);

    s.to_string()
}
