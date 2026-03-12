use rust_ten_commandments::{print_commandments, write_commandments_to_file};

fn main() {
    print_commandments();

    if let Err(e) = write_commandments_to_file() {
        eprintln!("The Crab could not inscribe the tablet: {}", e);
    }
}