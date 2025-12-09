use rodio::{Decoder, OutputStream, source::Source};
use std::fs::File;
use std::io::BufReader;

fn main() {
    // let stream_handle =
    //     rodio::OutputStreamBuilder::open_default_stream().expect("open default audio stream");
    //
    // let chord = File::open().unwrap();
    //
    // let file = BufReader::new(chord);
    // let sink = rodio::play(&stream_handle.mixer(), file).unwrap();
    //
    // std::thread::sleep(std::time::Duration::from_secs(5));
    let splendid = std::fs::read_dir("splendid/Samples").unwrap();
    let salamander = std::fs::read_dir("salamander/Samples").unwrap();

    let mut cnt_splendid: i32 = 0;
    let mut cnt_salamander: i32 = 0;

    println!("Splendid:");
    for c in splendid {
        cnt_splendid += 1;
        println!("{:?}", c);
    }

    println!("Salamander:");
    for c in salamander {
        cnt_salamander += 1;
        println!("{:?}", c);
    }

    println!("The total chord of splendid is: {} chord", cnt_splendid);
    println!("The total chord of salamander is: {} chord", cnt_salamander);
}
