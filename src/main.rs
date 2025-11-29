// Cargo.toml
// [dependencies]
// eyre = "0.6"

use eyre::{Result, eyre};

fn main() {
    let err: Result<()> = Err(eyre!("Something went wrong!"));

    // Print the error
    match err {
        Ok(_) => println!("No error"),
        Err(e) => println!("Error: {}", e),
    }
}
