use conectric_sdk::serial::ConectricSerial;

fn main() {
    println!("Conectric Gateway : RUST Edition");
    ConectricSerial::start_gateway();
}
