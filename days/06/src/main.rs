use day06::find_packet_start_in_file;

fn main() {
    let filename = "input.txt";

    match find_packet_start_in_file(filename) {
        Ok(Some(index)) => println!("Packet starts at index {}", index),
        Ok(None) => println!("No packet found"),
        Err(error) => println!("Error: {}", error),
    }
}
