use anyhow::{anyhow, Context, Result};
use serde::Serialize;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use tracing::info;

use crate::parse_csv_line;

#[derive(Debug, Default, Clone, Serialize)]
pub struct Device {
    pub name: String,
    pub description: String,
    pub path: String,
}

/// Detects devices by executing a given command and parsing its output.
///
/// This function spawns a new process to run the specified command with the arguments
/// `-r --cache=1 info disc:999`. It captures the standard output of this process and
/// reads it line by line, parsing each line as CSV. If a line represents a device (i.e.,
/// it starts with "DRV:"), the function extracts the device type, name, and path from
/// the line. Detected devices are logged and collected into a vector.
///
/// # Arguments
///
/// * `command` - A string slice that holds the command to be executed (path of makemkvcon).
/// * `makemkv_mutex` - An `Arc<Mutex<()>>` that ensures mutual exclusion when accessing shared resources.
///
/// # Returns
///
/// This function returns a `Result` containing a vector of `Device` structs if successful, or an error
/// if no devices are found or if any other error occurs during execution.
///
/// # Errors
///
/// This function will return an error if:
/// - The process cannot be spawned.
/// - The standard output of the process cannot be captured.
/// - No devices are detected.
///
/// # Example
///
/// ```rust
/// use makemkv_core::{detect_devices};
/// use std::sync::{Arc, Mutex};
///
/// let command = "makemkvcon";
/// let makemkv_mutex = Arc::new(Mutex::new(()));
///
/// match detect_devices(command, makemkv_mutex) {
///     Ok(devices) => println!("Detected devices: {:?}", devices),
///     Err(e) => eprintln!("Error detecting devices: {}", e),
/// }
/// ```
pub fn detect_devices(command: &str, makemkv_mutex: &Arc<Mutex<()>>) -> Result<Vec<Device>> {
    let _lock = makemkv_mutex.lock().map_err(|e| anyhow!("failed to lock makemkv_mutex: {}", e))?;

    info!("detecting devices with command: {}", command);

    let mut process = Command::new(command)
        .args(&["-r", "--cache=1", "info", "disc:999"])
        .stdout(Stdio::piped())
        .spawn()
        .context("failed to spawn devices process")?;

    let stdout = BufReader::new(process.stdout.take().context("failed to capture stdout")?);

    let devices: Vec<Device> = stdout
        .lines()
        .filter_map(|line| {
            let line = line.ok()?;
            let columns = parse_csv_line(&line);

            if columns[0].starts_with("DRV:") {
                let description = columns[4].trim().to_string();
                let name = columns[5].trim().to_string();
                let path = columns[6].trim().to_string();

                if !name.is_empty() && !description.is_empty() && !path.is_empty() {
                    info!(name = &name, description = &description, path = &path, "device detected");
                    return Some(Device { name, description, path });
                }
            }

            None
        })
        .collect();

    if devices.len() == 0 {
        anyhow::bail!("no devices found");
    }

    Ok(devices)
}
