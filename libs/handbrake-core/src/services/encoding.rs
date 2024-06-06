use anyhow::{Context, Result};
use serde::Serialize;
use std::io::{BufRead, BufReader};
use std::path::Path;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc::Sender, Arc};
use tracing::info;

use crate::Profile;

#[derive(Debug, Serialize)]
pub struct EncodingProgressPayload {
    pub progress: f32,
    pub eta: f32,
    pub step: usize,
}

/// Encodes a list of files using the specified command and profile, and sends progress updates through a channel.
///
/// This function takes a command, an encoding profile, a list of input files, an output directory,
/// a cancellation flag, and a sender for progress updates. It encodes each file according to the
/// profile settings and sends progress updates via the provided sender. If the cancel flag is set
/// during processing, the operation is aborted.
///
/// # Arguments
///
/// * `command` - A string slice that holds the encoding command to be executed.
/// * `profile` - A reference to a `Profile` struct containing encoding settings.
/// * `files` - A slice of string slices representing the paths of the files to be encoded.
/// * `output_dir` - A string slice specifying the directory where encoded files will be saved.
/// * `cancel_flag` - An `Arc<AtomicBool>` that can be used to signal cancellation of the operation.
/// * `sender` - A `Sender` channel for sending progress updates and completion notifications.
///
/// # Returns
///
/// This function returns a `Result` which will be:
/// * `Ok(())` if all files are encoded successfully.
/// * An error if any step in the encoding process fails.
///
/// # Errors
///
/// This function will return an error if:
/// * Any of the input files cannot be processed.
/// * The output directory cannot be created.
/// * The encoding process cannot be started.
/// * The output of the encoding process cannot be read or parsed.
///
/// # Example
///
/// ```
/// use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
/// use std::sync::mpsc::channel;
/// use your_crate::{encode_files, Profile};
///
/// let command = "ffmpeg";
/// let profile = Profile {
///     file_name: "profile.json".to_string(),
///     preset_name: "preset".to_string(),
/// };
/// let files = vec!["input1.mp4", "input2.mp4"];
/// let output_dir = "/path/to/output";
/// let cancel_flag = Arc::new(AtomicBool::new(false));
/// let (sender, receiver) = channel();
///
/// std::thread::spawn(move || {
///     encode_files(command, &profile, &files, output_dir, cancel_flag, sender).expect("Failed to encode files");
/// });
///
/// for msg in receiver {
///     println!("Received: {:?}", msg);
/// }
/// ```
///
/// Note: In a real-world application, proper error handling should be added to handle any potential issues gracefully.

pub fn encode_files(
    command: &str, profile: &Profile, files: &[&str], output_dir: &str, cancel_flag: Arc<AtomicBool>, sender: Sender<(&str, Option<EncodingProgressPayload>)>,
) -> Result<()> {
    for (i, &file) in files.iter().enumerate() {
        let file_name = Path::new(file).file_name().context("failed to get file name")?.to_str().unwrap();

        let encoding_output_dir = Path::new(output_dir).join("encoding/");

        if !encoding_output_dir.exists() {
            std::fs::create_dir_all(&encoding_output_dir).context("failed to create encoding output directory")?;
        }

        let output_path = encoding_output_dir.join(file_name);
        let output_file = output_path.to_str().unwrap();

        info!("encoding file:{} with profile: {} into {}", file, profile.file_name, output_file);

        let mut process = Command::new(command)
            .args(&["--json", "--input", file, "--output", output_file, "--preset-import-file", &profile.file_name, "-Z", &profile.preset_name])
            .stdout(Stdio::piped())
            .spawn()
            .context("failed to spawn encoding process")?;

        let stdout = BufReader::new(process.stdout.take().context("failed to capture stdout")?);

        let mut current_progress = 0.0;
        let mut current_eta = 0.0;

        for line in stdout.lines() {
            if cancel_flag.load(Ordering::Relaxed) {
                process.kill()?;
                anyhow::bail!("operation aborted");
            }

            let line = line?;
            let splitted = line.trim().split(":").collect::<Vec<&str>>();

            match line.clone() {
                x if x.contains("\"Progress\"") => {
                    current_progress = splitted
                        .last()
                        .context("failed to get progress value")?
                        .replace(",", "")
                        .trim()
                        .parse()
                        .context("failed to parse progress value")?;
                }
                x if x.contains("\"ETASeconds\"") => {
                    current_eta = splitted
                        .last()
                        .context("failed to get eta value")?
                        .replace(",", "")
                        .trim()
                        .parse()
                        .context("failed to parse eta value")?;
                }
                _ => {}
            }

            sender
                .send(("progress", Some(EncodingProgressPayload { progress: current_progress, eta: current_eta, step: i })))
                .unwrap();
        }
    }

    sender.send(("done", None)).unwrap();

    Ok(())
}
