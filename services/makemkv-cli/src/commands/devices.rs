use std::sync::{Arc, Mutex};

use makemkv_core::detect_devices;

#[derive(Clone, Debug, clap::Parser)]
#[clap(about = "Get available devices")]
pub struct Devices {
    #[arg(short, long, default_value_t = String::from("makemkvcon"), help = "Path of makemkvcon executable")]
    pub location: String,

    #[arg(short, long, help = "Path to output file (JSON format)")]
    pub output: Option<String>,
}

pub fn device_execution(args: &Devices) {
    let lock = Arc::new(Mutex::new(()));
    let devices = detect_devices(&args.location, lock).unwrap();

    if devices.len() == 0 {
        return eprintln!("No devices found.");
    }

    let json = serde_json::to_string_pretty(&devices).unwrap();

    if let Some(output) = &args.output {
        return std::fs::write(output, json).unwrap();
    }

    println!("{}", json);
}
