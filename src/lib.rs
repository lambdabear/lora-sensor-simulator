use std::io::{self, Write};
use std::time::Duration;

use serialport::prelude::*;

pub fn send(port_name: &str, baud_rate: u32, secs: u64) -> () {
    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(10);
    settings.baud_rate = baud_rate;

    let data: &[u8] = &[
        0xFF, 0xFE, 0x42, 0xE0, 0x01, 0xAA, 0xAA, 0xAA, 0xAA, 0x01, 0x64, 0x10,
    ];

    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut port) => {
            println!(
                "Sending data to {} at {} baud every {} second",
                &port_name, &baud_rate, secs
            );
            loop {
                let mut serial_buf: Vec<u8> = vec![0; 1000];
                match port.write(&data) {
                    Ok(_) => println!("send data: {:?}", data),
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => println!("{:?}", e),
                }
                // std::thread::sleep(Duration::from_millis(10));
                for _ in 0..40 {
                    match port.read(serial_buf.as_mut_slice()) {
                        Ok(t) => println!("receive: {:?}", &serial_buf[..t]),
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                        Err(e) => eprintln!("{:?}", e),
                    }
                    std::thread::sleep(Duration::from_millis(50));
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

pub fn receive(port_name: &str, baud_rate: u32) -> () {
    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(10);
    settings.baud_rate = baud_rate;

    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut port) => {
            println!("receiving data:");

            let mut serial_buf: Vec<u8> = vec![0; 1000];
            // std::thread::sleep(Duration::from_millis(10));
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => {
                        println!("{:?}", &serial_buf[..t]);
                        std::thread::sleep(Duration::from_millis(100));
                        let mut data: Vec<u8> = vec![0x00, 0x01, 0x42];
                        data.extend_from_slice(&serial_buf[..t]);
                        match port.write(&data) {
                            Ok(_) => println!("send data: {:?}", &data),
                            Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                            Err(e) => println!("{:?}", e),
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
                std::thread::sleep(Duration::from_millis(100));
            }
        }
        Err(e) => {
            println!("Failed to open \"{}\". Error: {}", port_name, e);
            std::process::exit(1);
        }
    }
}

pub fn get_addr(port_name: &str) {
    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(10);
    settings.baud_rate = 9600;

    let command: &[u8] = &[0xC1, 0x00, 02];

    match serialport::open_with_settings(port_name, &settings) {
        Ok(mut port) => {
            println!("Get Device Address: ");
            let mut serial_buf: Vec<u8> = vec![0; 5];
            match port.write(&command) {
                Ok(_) => println!("send data: {:?}", command),
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                Err(e) => println!("{:?}", e),
            }
            std::thread::sleep(Duration::from_millis(100));
            for _ in 0..20 {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => io::stdout().write_all(&serial_buf[..t]).unwrap(),
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
            println!("{:?}", serial_buf);
        }
        Err(e) => {
            println!("Failed to open \"{}\". Error: {}", port_name, e);
            std::process::exit(1);
        }
    }
}
