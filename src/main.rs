use clap::{App, AppSettings, Arg};

fn main() {
    let matches = App::new("Lora sensor simulator")
        .about("Send fake data very n seconds")
        .setting(AppSettings::DisableVersion)
        .arg(
            Arg::with_name("port")
            .help("The device path to a serial port")
            .use_delimiter(false)
            .required(true)
        )
        .arg(
            Arg::with_name("baud")
            .help("The baud rate to connetc at")
            .use_delimiter(false)
            .required(true)
        )
        .arg(
            Arg::with_name("seconds")
            .help("The cycle time to send data")
            .use_delimiter(false)
            .required(true)
        )
        .get_matches();
    
    let port_name = matches.value_of("port").unwrap();
    let baud_rate = matches.value_of("baud").unwrap().parse::<u32>().unwrap();
    let secs = matches.value_of("seconds").unwrap().parse::<u64>().unwrap();

    lora_sensor_simulator::send_data(port_name, baud_rate, secs);
}
