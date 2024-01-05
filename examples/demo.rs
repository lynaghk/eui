use eui::*;

// TODO: these derives should be done by our derive macro
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Schema, Serialize, Deserialize, Clone)]
pub enum Light {
    Off,
    On(Color),
}

#[derive(Debug, Schema, Serialize, Deserialize, Clone)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Status for Light {}
impl Command for Light {}

pub fn main() -> ! {
    let (cmd_tx, mut cmd_rx) = tokio::sync::mpsc::channel::<Light>(64);
    let (status_tx, status_rx) = tokio::sync::mpsc::channel::<Light>(64);

    std::thread::spawn(|| {
        serve_blocking("127.0.0.1:8080", status_rx, cmd_tx);
    });

    loop {
        while let Ok(cmd) = cmd_rx.try_recv() {
            println!("{:?}", cmd);
        }

        let _ = status_tx.try_send(Light::Off);

        std::thread::sleep(std::time::Duration::from_millis(10));
    }
}
