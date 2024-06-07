use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::mpsc::Sender;

const BUFFER_SIZE: usize = 16384;

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
