use std::fs::{self};

use makemkv_core::{detect_devices, read_disc_properties};

fn main() {
    let devices = detect_devices("examples/makemkvcon_device").unwrap();

    if devices.len() == 0 {
        println!("No devices found");
        return;
    }

    // let disc = read_disc_properties("examples/makemkvcon_movie", &devices[0].path);
    let disc = read_disc_properties(
        "/Applications/MakeMKV.app/Contents/MacOS/makemkvcon",
        &devices[0].path,
    );

    let json_disc = serde_json::to_string_pretty(&disc.unwrap()).unwrap();
    fs::write("parsed.json", json_disc).expect("written file");

    // let start = std::time::Instant::now();
    // let info = read_disc_properties("/Applications/MakeMKV.app/Contents/MacOS/makemkvcon");
    // println!("elapsed: {:?} for {:#?}", start.elapsed(), info.unwrap());
    // println!("elapsed: {:?}", start.elapsed());
}
