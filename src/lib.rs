use std::io::{self, Write};
use std::time::Duration;

use serialport::prelude::*;

pub fn send_data(port_name: &str, baud_rate: u32, secs: u64) -> () {
    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(10);
    settings.baud_rate = baud_rate;

    let data: &[u8] = &[
        0xFF, 0xFE, 0x42, 0xE0, 0x01, 0xAA, 0xAA, 0xAA, 0xAA, 0x01, 0x64, 0x10,
    ];

    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut port) => {
            println!(
                "Sending data to {} at {} baud every 10 second",
                &port_name, &baud_rate
            );
            loop {
                match port.write(&data) {
                    Ok(_) => println!("write data to {} succeed!", &port_name),
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => println!("{:?}", e),
                }
                std::thread::sleep(Duration::from_secs(secs));
            }
        }
        Err(e) => {
            println!("Failed to open \"{}\". Error: {}", port_name, e);
            std::process::exit(1);
        }
    }
}
