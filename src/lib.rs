use chrono::prelude::*;
use rand::{thread_rng, Rng};
use serialport::prelude::*;
use std::io::{self, Write};
use std::time::Duration;
use hex;

pub mod data_frame;

use crate::data_frame::DataFrame;

pub fn send(port_name: &str, baud_rate: u32, secs: u64, id: [u8; 4], data: f32) -> () {
    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(10);
    settings.baud_rate = baud_rate;
    let mut rng = thread_rng();

    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut port) => {
            println!(
                "Sending data to {} at {} baud every {} second",
                &port_name, &baud_rate, secs
            );
            loop {
                let data: f32 = rng.gen_range(0.0, data);
                let sensor_data = Vec::from(&DataFrame::new(1, id, 1, 100, data).encode()[..]);
                let mut buffer: Vec<u8> = Vec::new();
                buffer.extend_from_slice(&[0xFF, 0xFE, 0x42]);
                buffer.extend_from_slice(&sensor_data[..]);
                let mut serial_buf: Vec<u8> = vec![0; 1000];
                match port.write(&buffer) {
                    Ok(_) => println!("send data: {:?}", buffer),
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => println!("{:?}", e),
                }
                
                for _ in 0..30 {
                    match port.read(serial_buf.as_mut_slice()) {
                        Ok(_t) => {
                            println!("receive: {:?}", &serial_buf[..]);
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                        Err(e) => eprintln!("{:?}", e),
                    }
                   
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
                    Ok(_t) => {
                        let mut cache: [u8; 13] = [0; 13];
                        cache.copy_from_slice(&serial_buf[..13]);
                        let data = DataFrame::parse(cache).expect("data buffer error");
                        println!(
                            "[ id: {}, frame_type: {}, device_type: {}, battery: {}, data: {} ] {}",
                            hex::encode(&data.id()),
                            data.frame_type(),
                            data.device_type(),
                            data.battery(),
                            data.data(),
                            Local::now().format("%Y-%m-%d %H:%M:%S").to_string()
                        );
                        std::thread::sleep(Duration::from_millis(100));

                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
                std::thread::sleep(Duration::from_millis(100));
            }
        }
        Err(e) => {
            println!("Failed to open \"{}\". Error: {}", port_name, e);

            std::thread::sleep(Duration::from_secs(3));
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
