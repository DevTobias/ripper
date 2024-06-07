use anyhow::{anyhow, Context, Result};
use serde::Serialize;
use std::io::{BufRead, BufReader};
use std::process::{Command, Stdio};
use std::sync::{Arc, Mutex};
use tracing::{error, info};

use utils::{parse_csv_line, parse_duration_to_seconds};

#[derive(Debug, Default, Clone, Serialize)]
pub struct Title {
    pub id: usize,
    pub name: String,
    pub chapter_count: i8,
    pub duration: u32,
    pub disk_size: String,
    pub disk_size_bytes: i64,
    pub source_file_name: String,
    pub segments_count: i8,
    pub segments_map: String,
    pub output_file_name: String,
    pub metadata_language_code: String,
    pub metadata_language_name: String,
    pub tree_info: String,
    pub panel_title: String,
    pub order_weight: i32,
    pub video_stream: VideoStream,
    pub audio_streams: Vec<AudioStream>,
    pub subtitle_streams: Vec<SubtitleStream>,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct VideoStream {
    pub stream_type: String,
    pub codec_id: String,
    pub codec_short: String,
    pub codec_long: String,
    pub video_size: String,
    pub video_aspect_ratio: String,
    pub video_frame_rate: String,
    pub stream_flags: String,
    pub metadata_language_code: String,
    pub metadata_language_name: String,
    pub tree_info: String,
    pub panel_title: String,
    pub order_weight: i8,
    pub mkv_flags: String,
    pub output_conversion_type: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct AudioStream {
    pub stream_type: String,
    pub name: String,
    pub lang_code: String,
    pub lang_name: String,
    pub codec_id: String,
    pub codec_short: String,
    pub codec_long: String,
    pub bitrate: String,
    pub audio_channels_count: i8,
    pub audio_sample_rate: i32,
    pub audio_sample_size: i8,
    pub stream_flags: String,
    pub metadata_language_code: String,
    pub metadata_language_name: String,
    pub tree_info: String,
    pub panel_title: String,
    pub order_weight: i8,
    pub mkv_flags: String,
    pub mkv_flags_text: String,
    pub audio_channel_layout_name: String,
    pub output_conversion_type: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct SubtitleStream {
    pub stream_type: String,
    pub lang_code: String,
    pub lang_name: String,
    pub codec_id: String,
    pub codec_short: String,
    pub codec_long: String,
    pub stream_flags: String,
    pub metadata_language_code: String,
    pub metadata_language_name: String,
    pub tree_info: String,
    pub panel_title: String,
    pub order_weight: i8,
    pub mkv_flags: String,
    pub mkv_flags_text: String,
    pub output_conversion_type: String,
}

#[derive(Debug, Default, Clone, Serialize)]
pub struct Disc {
    pub disc_type: String,
    pub name: String,
    pub metadata_language_code: String,
    pub metadata_language_name: String,
    pub tree_info: String,
    pub panel_title: String,
    pub volume_name: String,
    pub order_weight: i8,
    pub titles: Vec<Title>,
}

enum StreamType {
    Video,
    Audio,
    Subtitle,
}

const CINFO_PREFIX: &str = "CINFO:";
const TINFO_PREFIX: &str = "TINFO:";
const SINFO_PREFIX: &str = "SINFO:";

/// Reads properties of a disc device by executing a given command and parsing its output.
///
/// This function spawns a new process to run the specified command with arguments
/// `-r info dev:<device>`. It captures the standard output of this process and reads it
/// line by line, parsing each line as CSV. The function extracts various properties
/// and streams (video, audio, subtitles) from the output and populates a `Disc` struct
/// with this information.
///
/// # Arguments
///
/// * `command` - A string slice that holds the command to be executed (path of makemkvcon).
/// * `device` - A string slice that represents the device identifier (path of blue ray / dvd drive).
/// * `makemkv_mutex` - An `Arc<Mutex<()>>` that ensures mutual exclusion when accessing shared resources.
///
/// # Returns
///
/// This function returns a `Result` containing a `Disc` struct if successful, or an error
/// if any error occurs during execution or parsing.
///
/// # Errors
///
/// This function will return an error if:
/// - The process cannot be spawned.
/// - The standard output of the process cannot be captured.
/// - Any required property or value is missing or cannot be parsed.
///
/// # Example
///
/// ```rust
/// use makemkv_core::{read_properties};
/// use std::sync::{Arc, Mutex};
///
/// let command = "makemkvcon";
/// let device = "/dev/sr0";
/// let makemkv_mutex = Arc::new(Mutex::new(()));
///
/// match read_properties(command, device, makemkv_mutex) {
///     Ok(disc) => println!("Disc properties: {:?}", disc),
///     Err(e) => eprintln!("Error reading properties: {}", e),
/// }
/// ```
pub fn read_disc_properties(command: &str, device: &str, makemkv_mutex: &Arc<Mutex<()>>) -> Result<Disc> {
    let _lock = makemkv_mutex.lock().map_err(|e| anyhow!("failed to lock makemkv_mutex: {}", e))?;

    info!("reading properties for device: {} with command: {}", device, command);

    let process = Command::new(command.to_owned())
        .args(&["-r", "info", format!("dev:{}", device).as_str()])
        .stdout(Stdio::piped())
        .spawn()
        .context("failed to spawn disc properties process")?;

    let stdout = BufReader::new(process.stdout.context("failed to capture stdout")?);

    let mut disc = Disc::default();
    let mut stream_type = StreamType::Video;

    let mut audio_stream_id: isize = -1;
    let mut subtitle_stream_id: isize = -1;

    for line in stdout.lines() {
        let columns = parse_csv_line(&line.context("failed to read line")?);

        match columns.get(0).map(|s| s.as_str()) {
            Some(x) if x.starts_with(CINFO_PREFIX) => handle_cinfo(&mut disc, x, &columns).context("failed to handle cinfo")?,
            Some(x) if x.starts_with(TINFO_PREFIX) => handle_tinfo(&mut disc, x, &columns).context("failed to handle tinfo")?,
            Some(x) if x.starts_with(SINFO_PREFIX) => {
                handle_sinfo(&mut disc, &mut stream_type, &mut audio_stream_id, &mut subtitle_stream_id, x, &columns).context("failed to handle sinfo")?
            }
            _ => {}
        }
    }

    Ok(disc)
}

/// Handles disc information based on a provided code and updates the disc properties accordingly.
fn handle_cinfo(disc: &mut Disc, x: &str, columns: &[String]) -> Result<()> {
    let code: usize = x.trim_start_matches(CINFO_PREFIX).parse().context("failed to parse disc code")?;
    let value = columns.get(2).context("missing disc value")?.to_string();

    match code {
        1 => disc.disc_type = value,
        2 => disc.name = value,
        28 => disc.metadata_language_code = value,
        29 => disc.metadata_language_name = value,
        30 => disc.tree_info = value,
        31 => disc.panel_title = value,
        32 => disc.volume_name = value,
        33 => disc.order_weight = value.parse()?,
        _ => error!("unhandled disc code: {}", code),
    }
    Ok(())
}

/// Handles title information based on a provided code and updates the title properties accordingly.
fn handle_tinfo(disc: &mut Disc, x: &str, columns: &[String]) -> Result<()> {
    let id: usize = x.trim_start_matches(TINFO_PREFIX).parse().context("failed to parse title id")?;
    let code: usize = columns.get(1).context("missing title code")?.parse().context("failed to parse title code")?;
    let value = columns.get(3).context("missing title value")?.to_string();

    if disc.titles.len() <= id {
        disc.titles.resize(id + 1, Title::default());
        disc.titles[id].id = id;
    }

    let title = &mut disc.titles[id];

    match code {
        2 => title.name = value,
        8 => title.chapter_count = value.parse()?,
        9 => title.duration = parse_duration_to_seconds(&value).unwrap_or(0),
        10 => title.disk_size = value,
        11 => title.disk_size_bytes = value.parse().context("failed to parse disk_size_bytes")?,
        16 => title.source_file_name = value,
        25 => title.segments_count = value.parse().context("failed to parse segments_count")?,
        26 => title.segments_map = value,
        27 => title.output_file_name = value,
        28 => title.metadata_language_code = value,
        29 => title.metadata_language_name = value,
        30 => title.tree_info = value,
        31 => title.panel_title = value,
        33 => title.order_weight = value.parse().context("failed to parse order_weight")?,
        _ => error!("unhandled title code: {}", code),
    }

    Ok(())
}

/// Handles stream information based on a provided code and updates the stream properties accordingly.
fn handle_sinfo(disc: &mut Disc, stream_type: &mut StreamType, audio_stream_id: &mut isize, subtitle_stream_id: &mut isize, x: &str, columns: &[String]) -> Result<()> {
    let title_id: usize = x.trim_start_matches(SINFO_PREFIX).parse().context("failed to parse title id")?;
    let code: usize = columns.get(2).context("missing stream code")?.parse().context("failed to parse stream code")?;
    let value = columns.get(4).context("missing stream value")?.to_string();

    if code == 1 {
        match value.as_str() {
            "Video" => {
                *stream_type = StreamType::Video;
                *audio_stream_id = -1;
                *subtitle_stream_id = -1;
            }
            "Audio" => {
                *stream_type = StreamType::Audio;
                *audio_stream_id += 1;
            }
            "Subtitles" => {
                *stream_type = StreamType::Subtitle;
                *subtitle_stream_id += 1;
            }
            _ => error!("unhandled stream type: {}", value),
        }
    }

    match stream_type {
        StreamType::Video => handle_video_stream(&mut disc.titles[title_id].video_stream, code, value),
        StreamType::Audio => handle_audio_stream(&mut disc.titles[title_id].audio_streams, *audio_stream_id as usize, code, value),
        StreamType::Subtitle => handle_subtitle_stream(&mut disc.titles[title_id].subtitle_streams, *subtitle_stream_id as usize, code, value),
    }
}

/// Handles video stream information based on a provided code and updates the stream properties accordingly.
fn handle_video_stream(stream: &mut VideoStream, code: usize, value: String) -> Result<()> {
    match code {
        1 => stream.stream_type = value,
        5 => stream.codec_id = value,
        6 => stream.codec_short = value,
        7 => stream.codec_long = value,
        19 => stream.video_size = value,
        20 => stream.video_aspect_ratio = value,
        21 => stream.video_frame_rate = value,
        22 => stream.stream_flags = value,
        28 => stream.metadata_language_code = value,
        29 => stream.metadata_language_name = value,
        30 => stream.tree_info = value,
        31 => stream.panel_title = value,
        33 => stream.order_weight = value.parse()?,
        38 => stream.mkv_flags = value,
        42 => stream.output_conversion_type = value,
        _ => error!("unhandled video stream code: {}", code),
    }

    Ok(())
}

/// Handles audio stream information based on a provided code and updates the stream properties accordingly.
fn handle_audio_stream(streams: &mut Vec<AudioStream>, index: usize, code: usize, value: String) -> Result<()> {
    if streams.len() <= index {
        streams.resize(index + 1, AudioStream::default());
    }

    let stream = &mut streams[index];

    match code {
        1 => stream.stream_type = value,
        2 => stream.name = value,
        3 => stream.lang_code = value,
        4 => stream.lang_name = value,
        5 => stream.codec_id = value,
        6 => stream.codec_short = value,
        7 => stream.codec_long = value,
        13 => stream.bitrate = value,
        14 => stream.audio_channels_count = value.parse()?,
        17 => stream.audio_sample_rate = value.parse()?,
        18 => stream.audio_sample_size = value.parse()?,
        22 => stream.stream_flags = value,
        28 => stream.metadata_language_code = value,
        29 => stream.metadata_language_name = value,
        30 => stream.tree_info = value,
        31 => stream.panel_title = value,
        33 => stream.order_weight = value.parse()?,
        38 => stream.mkv_flags = value,
        39 => stream.mkv_flags_text = value,
        40 => stream.audio_channel_layout_name = value,
        42 => stream.output_conversion_type = value,
        _ => error!("unhandled audio stream code: {}", code),
    }

    Ok(())
}

/// Handles subtitle stream information based on a provided code and updates the stream properties accordingly.
fn handle_subtitle_stream(streams: &mut Vec<SubtitleStream>, index: usize, code: usize, value: String) -> Result<()> {
    if streams.len() <= index {
        streams.resize(index + 1, SubtitleStream::default());
    }

    let stream = &mut streams[index];

    match code {
        1 => stream.stream_type = value,
        3 => stream.lang_code = value,
        4 => stream.lang_name = value,
        5 => stream.codec_id = value,
        6 => stream.codec_short = value,
        7 => stream.codec_long = value,
        22 => stream.stream_flags = value,
        28 => stream.metadata_language_code = value,
        29 => stream.metadata_language_name = value,
        30 => stream.tree_info = value,
        31 => stream.panel_title = value,
        33 => stream.order_weight = value.parse()?,
        38 => stream.mkv_flags = value,
        39 => stream.mkv_flags_text = value,
        42 => stream.output_conversion_type = value,
        _ => error!("unhandled subtitle stream code: {}", code),
    }

    Ok(())
}
