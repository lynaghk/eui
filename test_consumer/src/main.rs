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

fn main() {
    println!("{:?}", Light::SCHEMA);
}
