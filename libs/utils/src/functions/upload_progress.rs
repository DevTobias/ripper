use ssh2::Session;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{mpsc::Sender, Arc};
use tracing::{error, info};

use crate::ProgressTracker;

const BUFFER_SIZE: usize = 128 * 1024;

/// Moves a file from the source path to the destination path while reporting progress.
///
/// This function reads chunks from the source file and writes them to the destination file,
/// sending progress updates (as a percentage) through a given channel. After the move is complete,
/// the source file is deleted.
///
/// # Arguments
///
/// * `source` - The path to the source file to be moved. It can be any type that implements the `AsRef<Path>` trait.
/// * `destination` - The path to the destination file where the source file will be moved. It can be any type that implements the `AsRef<Path>` trait.
/// * `sender` - A channel sender for sending progress updates as a percentage (from 0.0 to 100.0).
///
/// # Returns
///
/// * `Ok(())` on success.
/// * `Err(io::Error)` if an I/O error occurs during the operation.
///
/// # Errors
///
/// This function will return an error if:
/// - The source file cannot be opened.
/// - The destination file cannot be created.
/// - An error occurs while reading from the source file or writing to the destination file.
/// - The source file cannot be deleted after the move.
///
/// # Example
///
/// ```
/// use std::sync::mpsc::channel;
/// use std::thread;
/// use std::time::Duration;
/// use std::fs::File;
/// use std::io::Write;
///
/// fn main() {
///     let (sender, receiver) = channel();
///     let source_path = "source.txt";
///     let destination_path = "destination.txt";
///
///     // Create a sample source file for demonstration
///     let mut file = File::create(source_path).unwrap();
///     writeln!(file, "Hello, world!").unwrap();
///
///     // Start the file move operation in a separate thread
///     thread::spawn(move || {
///         move_file_with_progress(source_path, destination_path, sender).unwrap();
///     });
///
///     // Monitor progress in the main thread
///     while let Ok(progress) = receiver.recv() {
///         println!("Progress: {:.2}%", progress);
///         thread::sleep(Duration::from_millis(100)); // Simulate doing other work
///     }
/// }
/// ```
///
pub fn move_file_with_progress<P: AsRef<Path>>(source: P, destination: P, sender: Sender<f64>) -> io::Result<()> {
    let mut src_file = File::open(&source)?;
    let mut dest_file = File::create(&destination)?;

    let total_size = src_file.metadata()?.len();
    let mut transferred: u64 = 0;
    let mut buffer = vec![0; BUFFER_SIZE];

    loop {
        let bytes_read = match src_file.read(&mut buffer) {
            Ok(0) => break,
            Ok(len) => len,
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };

        dest_file.write_all(&buffer[..bytes_read])?;
        transferred += bytes_read as u64;

        let progress = (transferred as f64 / total_size as f64) * 100.0;
        sender.send(progress).unwrap();
    }

    fs::remove_file(source)?;

    Ok(())
}

#[derive(Debug)]
pub struct UploadProgressPayload {
    pub progress: f32,
    pub eta: f32,
    pub step: u32,
}

/// Uploads a file to a remote server using SFTP with progress updates.
///
/// This function connects to a remote server via SSH, authenticates using the provided
/// credentials, and uploads a file from the local filesystem to the remote server. It
/// sends progress updates through the provided channel.
///
/// # Arguments
///
/// * `local_path` - The path to the local file to be uploaded.
/// * `remote_user` - The username for SSH authentication on the remote server.
/// * `remote_host` - The hostname or IP address of the remote server.
/// * `remote_path` - The path on the remote server where the file should be uploaded.
/// * `password` - The password for SSH authentication on the remote server.
/// * `sender` - A channel sender for sending progress updates as a percentage (0.0 to 100.0).
///
/// # Errors
///
/// This function returns an `io::Result` which can contain an error if:
/// - The TCP connection to the remote server fails.
/// - The SSH handshake fails.
/// - Authentication fails.
/// - The local file cannot be opened or read.
/// - The remote file cannot be created or written to.
///
/// # Example
///
/// ```
/// use std::sync::mpsc::channel;
///
/// fn main() {
///     let local_path = "path/to/local/file";
///     let remote_user = "username";
///     let remote_host = "hostname";
///     let remote_path = "path/to/remote/file";
///     let password = "your_password";
///
///     let (tx, rx) = channel();
///
///     // Spawn a thread to handle the file upload
///     std::thread::spawn(move || {
///         if let Err(e) = upload_file_with_sftp(local_path, remote_user, remote_host, remote_path, password, tx) {
///             eprintln!("Error: {}", e);
///         }
///     });
///
///     // Handle progress updates
///     for progress in rx {
///         println!("Progress: {:.2}%", progress);
///     }
/// }
/// ```
pub fn upload_file_with_sftp(
    local_path: &str, remote_path: &str, file_id: u32, remote_host: &str, remote_user: &str, remote_password: &str, cancel_flag: &Arc<AtomicBool>,
    sender: &Sender<(&str, Option<UploadProgressPayload>)>,
) -> io::Result<()> {
    let mut progress_tracker = ProgressTracker::new();

    let tcp = TcpStream::connect(format!("{}:22", remote_host))?;

    let mut session = Session::new().unwrap();
    session.set_tcp_stream(tcp);
    session.handshake()?;

    session.userauth_password(remote_user, remote_password)?;

    if !session.authenticated() {
        return Err(io::Error::new(io::ErrorKind::Other, "Authentication failed"));
    }

    let mut file = File::open(&local_path)?;
    let metadata = file.metadata()?;
    let file_size = metadata.len();

    let sftp = session.sftp()?;

    let mut remote_file = sftp.create(&Path::new(remote_path))?;

    let mut buffer = vec![0u8; BUFFER_SIZE];
    let mut total_bytes_sent = 0;

    loop {
        if cancel_flag.load(Ordering::Relaxed) {
            info!("uploading operation aborted");

            if let Err(e) = sftp.unlink(&Path::new(remote_path)) {
                error!("Failed to delete remote file: {}", e);
            } else {
                info!("Remote file deleted successfully");
            }

            return Ok(());
        }

        let bytes_read = match file.read(&mut buffer) {
            Ok(0) => break,
            Ok(len) => len,
            Err(ref e) if e.kind() == io::ErrorKind::Interrupted => continue,
            Err(e) => return Err(e),
        };

        remote_file.write_all(&buffer[..bytes_read])?;
        total_bytes_sent += bytes_read as u64;

        let progress = total_bytes_sent as f32 / file_size as f32;
        progress_tracker.update(total_bytes_sent as f32, file_size as f32).unwrap();

        let payload = UploadProgressPayload { progress, eta: progress_tracker.get_eta(), step: file_id };
        sender.send(("progress", Some(payload))).unwrap();
    }

    sender.send(("done", None)).unwrap();
    Ok(())
}
