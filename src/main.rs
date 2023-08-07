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
extern "C" {
    fn eui_invoke(s: &str);
}

fn main() -> Result<(), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let body = document.body().unwrap();

    render(&document, &body)?;

    use postcard::experimental::schema::Schema;

    eui_invoke(
        &serde_json::to_string_pretty(&(Light::SCHEMA, Light::On(Color { r: 1, g: 2, b: 3 })))
            .unwrap(),
    );

    Ok(())
}
