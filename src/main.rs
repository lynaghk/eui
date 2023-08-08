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

// pub fn main() {
//     use postcard::experimental::schema::Schema;
//     println!("{}", serde_json::to_string_pretty(&Light::SCHEMA).unwrap());
// }

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
pub fn eui_is_valid(s: &str) -> bool {
    serde_json::from_str::<Light>(s).is_ok()
}

#[wasm_bindgen]
pub fn eui_schema() -> String {
    use postcard::experimental::schema::Schema;
    serde_json::to_string_pretty(Light::SCHEMA).unwrap()
}

fn main() -> Result<(), JsValue> {
    // let window = web_sys::window().unwrap();
    // let document = window.document().unwrap();
    // let body = document.body().unwrap();
    //render(&document, &body)?;

    Ok(())
}
