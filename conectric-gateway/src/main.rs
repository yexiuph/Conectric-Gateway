use conectric_sdk::serial::ConectricSerial;

fn main() {
    println!("Conectric Gateway : RUST Edition");

    let mut conectric_serial = ConectricSerial::new();
    conectric_serial.start_gateway();

    // Check if the serial port is opened
    if let Some(serial_port) = &mut conectric_serial.serial_port {
        serial_port.write(b"test\n").expect("Write failed!");
    } else {
        println!("Serial port is not opened.");
    }
}
