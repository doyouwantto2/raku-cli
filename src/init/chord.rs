use bzip2::read::BzDecoder;
use indicatif::{ProgressBar, ProgressStyle};
use reqwest::blocking::Client;
use std::fs::{self, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use tar::Archive;

const ARCHIVE_NAME: &str = "SalamanderGrandPianoV3_48khz24bit.tar.bz2";
const DOWNLOAD_URL: &str =
    "https://archive.org/download/SalamanderGrandPianoV3/SalamanderGrandPianoV3_48khz24bit.tar.bz2";
const EXTRACTED_DIR: &str = "SalamanderGrandPianoV3_48khz24bit";

pub fn chord() -> Result<(), Box<dyn std::error::Error>> {
    if Path::new(EXTRACTED_DIR).exists() {
        println!("✓ Samples already installed.");
        return Ok(());
    }

    println!("Sample directory not found. Installing...");

    download_with_resume(DOWNLOAD_URL, ARCHIVE_NAME)?;

    println!("Extracting samples...");

    let archive_file = File::open(ARCHIVE_NAME)?;
    let decompressor = BzDecoder::new(archive_file);
    let mut archive = Archive::new(decompressor);
    archive.unpack(".")?;

    println!("✓ Samples extracted!");

    fs::remove_file(ARCHIVE_NAME)?;
    Ok(())
}

fn download_with_resume(url: &str, output: &str) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();

    let mut downloaded: u64 = 0;
    if Path::new(output).exists() {
        downloaded = fs::metadata(output)?.len();
        println!("Resuming download at {} bytes…", downloaded);
    }

    let mut request = client.get(url);
    if downloaded > 0 {
        request = request.header("Range", format!("bytes={}-", downloaded));
    }

    let mut response = request.send()?;
    if !response.status().is_success() && response.status() != 206 {
        return Err(format!("Failed to download: HTTP {}", response.status()).into());
    }

    let total_size = response
        .content_length()
        .map(|size| size + downloaded)
        .unwrap_or(downloaded);

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::with_template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})",
        )?
        .progress_chars("#>-"),
    );
    pb.set_position(downloaded);

    let mut file = OpenOptions::new().create(true).append(true).open(output)?;

    let mut buffer = [0u8; 32_768]; // bigger buffer for speed
    loop {
        let bytes_read = response.read(&mut buffer)?;
        if bytes_read == 0 {
            break;
        }

        file.write_all(&buffer[..bytes_read])?;
        downloaded += bytes_read as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Downloaded");

    Ok(())
}
