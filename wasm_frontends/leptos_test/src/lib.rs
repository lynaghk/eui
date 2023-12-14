use eui::bevy_reflect::{self, Reflect, Typed};

#[derive(Debug, eui::Schema, Reflect)]
pub enum Light {
    Off,
    On(Color),
}

#[derive(Debug, eui::Schema, Reflect)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}
