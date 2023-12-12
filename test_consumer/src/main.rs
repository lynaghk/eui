use eui::schema::Schema;

#[derive(eui::Schema)]
struct MyData {
    x: u64,
}

fn main() {
    println!("{:?}", MyData::SCHEMA);
}
