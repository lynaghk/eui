use schemars::{schema_for, JsonSchema};

#[derive(Debug, JsonSchema, postcard::experimental::schema::Schema)]
pub enum Light {
    Off,
    On(Color),
}

#[derive(Debug, JsonSchema, postcard::experimental::schema::Schema)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub fn main() {
    let schema = schema_for!(Light);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());

    use postcard::experimental::schema::Schema;
    dbg!(Light::SCHEMA);
}
