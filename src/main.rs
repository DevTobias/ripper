use makemkv_core::read_disc_properties;

fn main() {
    let start = std::time::Instant::now();

    // let info = read_disc_properties("/Applications/MakeMKV.app/Contents/MacOS/makemkvcon");
    let info = read_disc_properties("./info");

    println!("elapsed: {:?} for {:#?}", start.elapsed(), info);
}
