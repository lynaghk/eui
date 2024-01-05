use eui::*;

use eui::schema::Schema;

#[derive(Debug, eui::Schema)]
pub enum Light {
    Off,
    On(Color),
}

#[derive(Debug, eui::Schema)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub fn main() {
    serve_blocking::<Light, _>("127.0.0.1:8080");
}
