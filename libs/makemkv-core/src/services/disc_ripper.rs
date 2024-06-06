use anyhow::{anyhow, Context, Result};
use serde::Serialize;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc::Sender, Arc, Mutex};
use tracing::info;

use crate::{parse_csv_line, ProgressTracker};

#[derive(Debug, Serialize)]
pub struct ProgressPayload {
    pub step_title: String,
    pub step_details: String,
    pub progress: f32,
    pub step: usize,
    pub eta: f32,
}

const PRGT_PREFIX: &str = "PRGT:";
const PRGC_PREFIX: &str = "PRGC:";
const PRGV_PREFIX: &str = "PRGV:";

/// Rips titles from a disc device using the specified command, reporting progress and handling cancellation.
///
/// This function spawns a process to rip each title from the specified disc device, reporting progress through a channel and allowing for cancellation.
/// It acquires a lock on the provided mutex to ensure exclusive access to the MakeMKV process.
///
/// # Arguments
///
/// * `command` - The command to run for ripping titles (e.g., `makemkvcon`).
/// * `makemkv_mutex` - A mutex to ensure exclusive access to the MakeMKV process.
/// * `cancel_flag` - An atomic boolean flag to signal cancellation of the ripping process.
/// * `sender` - A channel sender for sending progress updates and completion notifications.
/// * `output_dir` - The directory to output the ripped titles.
/// * `device` - The device identifier for the disc (e.g., `/dev/sr0`).
/// * `ids` - A vector of title IDs to rip.
///
/// # Returns
///
/// * `Result<()>` - Ok if successful, otherwise an error indicating what went wrong.
///
/// # Errors
///
/// Returns an error if:
/// - The mutex cannot be locked.
/// - The ripping process cannot be spawned.
/// - The stdout of the process cannot be captured.
/// - Parsing the progress values fails.
///
/// # Examples
///
/// ```
/// let command = "makemkvcon";
/// let makemkv_mutex = Arc::new(Mutex::new(()));
/// let cancel_flag = Arc::new(AtomicBool::new(false));
/// let (sender, receiver) = std::sync::mpsc::channel();
/// let output_dir = "/path/to/output";
/// let device = "/dev/sr0";
/// let ids = vec![1, 2, 3];
///
/// rip_titles(command, &makemkv_mutex, cancel_flag, sender, output_dir, device, ids)?;
/// ```
pub fn rip_titles(
    command: &str, makemkv_mutex: &Arc<Mutex<()>>, cancel_flag: Arc<AtomicBool>, sender: Sender<(&str, Option<ProgressPayload>)>, output_dir: &str, device: &str,
    ids: &[usize],
) -> Result<()> {
    let _lock = makemkv_mutex.lock().map_err(|e| anyhow!("failed to lock makemkv_mutex: {}", e))?;

    for (i, &id) in ids.iter().enumerate() {
        let mut process = Command::new(command)
            .args(&["--messages=-stdout", "--progress=-same", "-r", "mkv", &format!("dev:{}", device), &id.to_string(), output_dir])
            .stdout(Stdio::piped())
            .spawn()
            .context("failed to spawn ripping process")?;

        let stdout = BufReader::new(process.stdout.take().context("failed to capture stdout")?);

        let mut current_step = String::new();
        let mut current_step_details = String::new();
        let mut current_progress = 0.0;
        let mut progress_tracker = ProgressTracker::new();

        for line in stdout.lines() {
            if cancel_flag.load(Ordering::Relaxed) {
                process.kill()?;
                info!("makemkv operation aborted");
                return Ok(());
            }

            let line = line?;
            let columns = parse_csv_line(&line);

            match columns.get(0).map(|s| s.as_str()) {
                Some(x) if x.starts_with(PRGT_PREFIX) => {
                    current_step = columns.get(2).context("missing step title value")?.to_string();
                }
                Some(x) if x.starts_with(PRGC_PREFIX) => {
                    current_step_details = columns.get(2).context("missing step details value")?.to_string();
                }
                Some(x) if x.starts_with(PRGV_PREFIX) => {
                    let curr: f32 = columns.get(1).context("missing current progress value")?.parse()?;
                    let total: f32 = columns.get(2).context("missing total progress value")?.parse()?;
                    current_progress = curr / total;
                    progress_tracker.update(curr, total).unwrap();
                }
                _ => {}
            }

            let payload = ProgressPayload {
                step_title: current_step.to_owned(),
                step_details: current_step_details.to_owned(),
                progress: current_progress,
                eta: progress_tracker.get_eta(),
                step: i,
            };

            sender.send(("progress", Some(payload))).unwrap();
        }
    }

    sender.send(("done", None)).unwrap();

    Ok(())
}
