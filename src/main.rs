use serde::{Deserialize, Serialize};

#[derive(Debug, postcard::experimental::schema::Schema, Serialize, Deserialize)]
pub enum Light {
    Off,
    On(Color),
}

#[derive(Debug, postcard::experimental::schema::Schema, Serialize, Deserialize)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub fn main() {
    use postcard::experimental::schema::Schema;
    //println!("{}", serde_json::to_string_pretty(&Light::SCHEMA).unwrap());

    let x = Light::On(Color {
        r: 123,
        g: 33,
        b: 22,
    });

    // let x = Light::Off;

    let bs = postcard::to_stdvec_cobs(&x).unwrap();
    //write bs in binary to stdout
    let mut stdout = std::io::stdout().lock();
    use std::io::Write;
    stdout.write_all(&bs[..]).unwrap();
    stdout.flush().unwrap();
}

use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlElement};

fn render(document: &Document, body: &HtmlElement) -> Result<(), JsValue> {
    // Manufacture the element we're gonna append
    let val = document.create_element("p")?;

    val.set_text_content(Some("Hello from Rust!"));
    body.append_child(&val)?;
    Ok(())
}

#[wasm_bindgen]
pub fn eui_serialize(s: &str) -> Option<Vec<u8>> {
    serde_json::from_str::<Light>(s)
        .ok()
        .and_then(|x| postcard::to_stdvec_cobs(&x).ok())
}

#[wasm_bindgen]
pub fn eui_schema() -> String {
    use postcard::experimental::schema::Schema;
    serde_json::to_string_pretty(Light::SCHEMA).unwrap()
}

// fn main() -> Result<(), JsValue> {
//     // let window = web_sys::window().unwrap();
//     // let document = window.document().unwrap();
//     // let body = document.body().unwrap();
//     //render(&document, &body)?;

//     Ok(())
// }
