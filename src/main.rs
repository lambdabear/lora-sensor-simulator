use clap::{App, AppSettings, Arg, SubCommand};

fn main() {
    let matches = App::new("Lora sensor simulator")
        .subcommand(
            SubCommand::with_name("send")
                .about("Send data very n seconds")
                .setting(AppSettings::DisableVersion)
                .arg(
                    Arg::with_name("port")
                        .help("The device path to a serial port")
                        .use_delimiter(false)
                        .required(true),
                )
                .arg(
                    Arg::with_name("baud")
                        .help("The baud rate to connetc at")
                        .use_delimiter(false)
                        .required(true),
                )
                .arg(
                    Arg::with_name("seconds")
                        .help("The cycle time to send data")
                        .use_delimiter(false)
                        .required(true),
                )
                .arg(
                    Arg::with_name("id")
                        .help("The id of lora node")
                        .use_delimiter(false)
                        .required(true),
                )
                .arg(
                    Arg::with_name("data")
                        .help("The sensor data of lora node")
                        .use_delimiter(false)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("receive")
                .about("Receive data")
                .setting(AppSettings::DisableVersion)
                .arg(
                    Arg::with_name("port")
                        .help("The device path to a serial port")
                        .use_delimiter(false)
                        .required(true),
                )
                .arg(
                    Arg::with_name("baud")
                        .help("The baud rate to connetc at")
                        .use_delimiter(false)
                        .required(true),
                ),
        )
        .subcommand(
            SubCommand::with_name("addr")
                .about("get device loar address")
                .arg(
                    Arg::with_name("port")
                        .help("The device path to a serial port")
                        .required(true),
                ),
        )
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("send") {
        let port_name = matches.value_of("port").unwrap();
        let baud_rate = matches.value_of("baud").unwrap().parse::<u32>().unwrap();
        let secs = matches.value_of("seconds").unwrap().parse::<u64>().unwrap();
        let id = matches.value_of("id").unwrap().parse::<u32>().unwrap();
        let data = matches
            .value_of("data")
            .unwrap()
            .parse::<f32>()
            .expect("f32 number parse error");
        lora_sensor_simulator::send(port_name, baud_rate, secs, id, data);
    }

    if let Some(matches) = matches.subcommand_matches("receive") {
        let port_name = matches.value_of("port").unwrap();
        let baud_rate = matches.value_of("baud").unwrap().parse::<u32>().unwrap();
        lora_sensor_simulator::receive(port_name, baud_rate);
    }

    if let Some(matches) = matches.subcommand_matches("addr") {
        let port_name = matches.value_of("port").unwrap();
        lora_sensor_simulator::get_addr(port_name);
    }
}
