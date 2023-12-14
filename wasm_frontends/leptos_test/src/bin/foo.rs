use eui::bevy_reflect::{self, Reflect, Typed, *};
use leptos_test::*;

#[derive(Debug, Reflect)]
pub enum Light {
    Off,
    On(Color),
}

#[derive(Debug, Reflect)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

pub fn main() {
    // let mut registry = TypeRegistry::empty();
    // registry.register::<Light>();

    // //let registration = registry.get(std::any::TypeId::of::<Foo>()).unwrap();
    // dbg!(registry.get_with_short_type_path("Light"));

    dbg!(Light::type_info());

    let x = Light::On(Color { r: 0, g: 0, b: 0 });

    let mut x = x.clone_value();

    x.apply(&DynamicEnum::new("Off", ()));

    let x = Light::from_reflect(&*x).unwrap();

    println!("{:?}", x);
}
