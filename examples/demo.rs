use eui::*;

#[eui]
#[derive(Debug)]
pub enum Light {
    Off,
    White(u8),
    On(Color),
}

#[eui]
#[derive(Debug)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
}

impl Status for Light {}
impl Command for Light {}

pub fn main() -> ! {
    env_logger::init();

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
