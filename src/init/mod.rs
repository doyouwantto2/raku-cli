use chord::chord;

pub mod chord;

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    chord()?;
    Ok(())
}
