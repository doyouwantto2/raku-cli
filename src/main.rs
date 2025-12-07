use raku::init::init;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    init()?; 

    println!("Init finished!");

    Ok(())
}
