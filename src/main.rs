use std::io::Write;
use console::Term;

fn main() {
    let term = Term::stdout();
    term.set_title("Testing lora data");
    write!(&term, "Please set serial port (设置串口): ").unwrap();
    let port = term.read_line_initial_text("COM1").unwrap();
    write!(&term, "\n").unwrap();
    term.write_line("Receiving...").unwrap();
 
    lora_sensor_simulator::receive(&port, 38400);
}
