use std::io::Cursor;

use js_sys::Uint8Array;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn process(input: Uint8Array, minutes: u32) -> Uint8Array {
    let mut input_buffer = vec![0; input.length() as usize];
    input.copy_to(&mut input_buffer);

    let mut output_buffer = vec![];

    core::process(
        Cursor::new(input_buffer),
        &mut Cursor::new(&mut output_buffer),
        std::time::Duration::from_mins(minutes as u64),
    );

    Uint8Array::new_from_slice(&output_buffer)
}
