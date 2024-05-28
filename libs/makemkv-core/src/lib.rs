use std::{
    error::Error,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

pub use models::{Device, Disc, Title};
use tmdb_client::TmdbClient;

mod models;
mod parser;

pub fn detect_devices(command: &str) -> Result<Vec<Device>, Box<dyn Error>> {
    let process = Command::new(command).args(["-r", "--cache=1", "info", "disc:999"]).stdout(Stdio::piped()).spawn()?;

    let stdout = BufReader::new(process.stdout.ok_or("failed to capture stdout")?);

    let mut devices: Vec<Device> = Vec::new();

    for line in stdout.lines() {
        let columns = parser::parse_csv_line(&line?);

        if columns[0].starts_with("DRV:") {
            let device_type = columns[4].trim().to_string();
            let name: String = columns[5].trim().to_string();
            let path = columns[6].trim().to_string();

            if name.len() > 0 && device_type.len() > 0 && path.len() > 0 {
                devices.push(Device { name, device_type, path });
            }
        }
    }

    if devices.len() == 0 {
        return Err("no devices found".into());
    }

    Ok(devices)
}

pub fn rip_titles(command: &str, device: &str, ids: Vec<usize>, output_dir: &str, progress: &dyn Fn(&str, &str, f32, usize) -> ()) -> Result<(), Box<dyn Error>> {
    for (i, id) in ids.iter().enumerate() {
        let process = Command::new(command)
            .args([
                "--messages=-stdout",
                "--progress=-same",
                "-r",
                "mkv",
                format!("dev:{}", device).as_str(),
                id.to_string().as_str(),
                output_dir,
            ])
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = BufReader::new(process.stdout.ok_or("failed to capture stdout")?);

        let mut current_step = String::new();
        let mut current_step_details = String::new();
        let mut current_progress = 0.0;

        for line in stdout.lines() {
            let line = line?;

            let columns = parser::parse_csv_line(&line);

            match columns[0].as_str() {
                x if x.starts_with("PRGT:") => {
                    current_step = columns.get(2).ok_or("missing value")?.to_string();
                }
                x if x.starts_with("PRGC:") => {
                    current_step_details = columns.get(2).ok_or("missing value")?.to_string();
                }
                x if x.starts_with("PRGV:") => {
                    let curr: f32 = columns.get(1).ok_or("missing value")?.parse()?;
                    let total: f32 = columns.get(2).ok_or("missing value")?.parse()?;
                    current_progress = curr / total;
                }
                _ => {}
            }

            progress(current_step.as_str(), current_step_details.as_str(), current_progress, i);
        }
    }

    Ok(())
}

pub fn read_properties(command: &str, device: &str) -> Result<Disc, Box<dyn Error>> {
    enum StreamType {
        Video,
        Audio,
        Subtitle,
    }

    let process = Command::new(command.to_owned())
        .args(["-r", "info", format!("dev:{}", device).as_str()])
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = BufReader::new(process.stdout.ok_or("failed to capture stdout")?);

    let mut disc = Disc::default();
    let mut stream_type = StreamType::Video;

    let mut audio_stream_id: isize = -1;
    let mut subtitle_stream_id: isize = -1;

    for line in stdout.lines() {
        let columns = parser::parse_csv_line(&line?);

        match columns[0].as_str() {
            x if x.starts_with("CINFO:") => {
                let code: usize = x.trim_start_matches("CINFO:").parse()?;
                let value = columns.get(2).ok_or("missing value")?.to_string();

                match code {
                    1 => disc.disc_type = value,
                    2 => disc.name = value,
                    28 => disc.metadata_language_code = value,
                    29 => disc.metadata_language_name = value,
                    30 => disc.tree_info = value,
                    31 => disc.panel_title = value,
                    32 => disc.volume_name = value,
                    33 => disc.order_weight = value.parse()?,
                    _ => println!("unhandled disc code: {}", code),
                }
            }
            x if x.starts_with("TINFO:") => {
                let id: usize = x.trim_start_matches("TINFO:").parse()?;
                let code: usize = columns.get(1).ok_or("missing code")?.parse()?;
                let value = columns.get(3).ok_or("missing value")?.to_string();

                if disc.titles.len() <= id {
                    disc.titles.resize(id + 1, Title::default());
                    disc.titles[id].id = id;
                }

                match code {
                    2 => disc.titles[id].name = value,
                    8 => disc.titles[id].chapter_count = value.parse()?,
                    9 => disc.titles[id].duration = parser::parse_duration_to_seconds(value.as_str()).unwrap_or(0),
                    10 => disc.titles[id].disk_size = value,
                    11 => disc.titles[id].disk_size_bytes = value.parse()?,
                    16 => disc.titles[id].source_file_name = value,
                    25 => disc.titles[id].segments_count = value.parse()?,
                    26 => disc.titles[id].segments_map = value,
                    27 => disc.titles[id].output_file_name = value,
                    28 => disc.titles[id].metadata_language_code = value,
                    29 => disc.titles[id].metadata_language_name = value,
                    30 => disc.titles[id].tree_info = value,
                    31 => disc.titles[id].panel_title = value,
                    33 => disc.titles[id].order_weight = value.parse()?,
                    _ => println!("unhandled title code: {}", code),
                }
            }
            x if x.starts_with("SINFO:") => {
                let title_id: usize = x.trim_start_matches("SINFO:").parse()?;
                let code: usize = columns.get(2).ok_or("missing code")?.parse()?;
                let value = columns.get(4).ok_or("missing value")?.to_string();

                if code == 1 {
                    match value.as_str() {
                        "Video" => {
                            stream_type = StreamType::Video;
                            audio_stream_id = -1;
                            subtitle_stream_id = -1;
                        }
                        "Audio" => {
                            stream_type = StreamType::Audio;
                            audio_stream_id += 1;
                        }
                        "Subtitles" => {
                            stream_type = StreamType::Subtitle;
                            subtitle_stream_id += 1;
                        }
                        _ => println!("unhandled stream type: {}", value),
                    }
                }

                match stream_type {
                    StreamType::Video => {
                        let stream_ref = &mut disc.titles[title_id].video_stream;

                        match code {
                            01 => stream_ref.stream_type = value,
                            05 => stream_ref.codec_id = value,
                            06 => stream_ref.codec_short = value,
                            07 => stream_ref.codec_long = value,
                            19 => stream_ref.video_size = value,
                            20 => stream_ref.video_aspect_ratio = value,
                            21 => stream_ref.video_frame_rate = value,
                            22 => stream_ref.stream_flags = value,
                            28 => stream_ref.metadata_language_code = value,
                            29 => stream_ref.metadata_language_name = value,
                            30 => stream_ref.tree_info = value,
                            31 => stream_ref.panel_title = value,
                            33 => stream_ref.order_weight = value.parse()?,
                            38 => stream_ref.mkv_flags = value,
                            42 => stream_ref.output_conversion_type = value,
                            _ => println!("unhandled subtitle code: {}", code),
                        }
                    }
                    StreamType::Audio => {
                        let audio_stream_id = audio_stream_id as usize;

                        let streams_ref = &mut disc.titles[title_id].audio_streams;

                        if streams_ref.len() <= audio_stream_id {
                            streams_ref.resize(audio_stream_id + 1, models::AudioStream::default());
                        }

                        let stream_ref = &mut streams_ref[audio_stream_id];

                        match code {
                            01 => stream_ref.stream_type = value,
                            02 => stream_ref.name = value,
                            03 => stream_ref.lang_code = value,
                            04 => stream_ref.lang_name = value,
                            05 => stream_ref.codec_id = value,
                            06 => stream_ref.codec_short = value,
                            07 => stream_ref.codec_long = value,
                            13 => stream_ref.bitrate = value,
                            14 => stream_ref.audio_channels_count = value.parse()?,
                            17 => stream_ref.audio_sample_rate = value.parse()?,
                            18 => stream_ref.audio_sample_size = value.parse()?,
                            22 => stream_ref.stream_flags = value,
                            28 => stream_ref.metadata_language_code = value,
                            29 => stream_ref.metadata_language_name = value,
                            30 => stream_ref.tree_info = value,
                            31 => stream_ref.panel_title = value,
                            33 => stream_ref.order_weight = value.parse()?,
                            38 => stream_ref.mkv_flags = value,
                            39 => stream_ref.mkv_flags_text = value,
                            40 => stream_ref.audio_channel_layout_name = value,
                            42 => stream_ref.output_conversion_type = value,
                            _ => println!("unhandled subtitle code: {}", code),
                        }
                    }
                    StreamType::Subtitle => {
                        let subtitle_stream_id = subtitle_stream_id as usize;

                        let streams_ref = &mut disc.titles[title_id].subtitle_streams;

                        if streams_ref.len() <= subtitle_stream_id {
                            streams_ref.resize(subtitle_stream_id + 1, models::SubtitleStream::default());
                        }

                        let stream_ref = &mut streams_ref[subtitle_stream_id];

                        match code {
                            01 => stream_ref.stream_type = value,
                            03 => stream_ref.lang_code = value,
                            04 => stream_ref.lang_name = value,
                            05 => stream_ref.codec_id = value,
                            06 => stream_ref.codec_short = value,
                            07 => stream_ref.codec_long = value,
                            22 => stream_ref.stream_flags = value,
                            28 => stream_ref.metadata_language_code = value,
                            29 => stream_ref.metadata_language_name = value,
                            30 => stream_ref.tree_info = value,
                            31 => stream_ref.panel_title = value,
                            33 => stream_ref.order_weight = value.parse()?,
                            38 => stream_ref.mkv_flags = value,
                            39 => stream_ref.mkv_flags_text = value,
                            42 => stream_ref.output_conversion_type = value,
                            _ => println!("unhandled subtitle code: {}", code),
                        }
                    }
                }
            }
            _ => {}
        }
    }

    Ok(disc)
}

pub async fn filter_movie_candidates(disc: Disc, langs: Vec<&str>, tmdb_id: u32, tmdb_key: &str) -> Result<Disc, Box<dyn Error>> {
    let movie = TmdbClient::new(tmdb_key).get_movie(tmdb_id).await?;

    let filtered_titles: Vec<Title> = disc
        .titles
        .iter()
        .filter(|title| {
            if title.audio_streams.len() == 0 {
                return false;
            }

            let satisfies_language = title.audio_streams.iter().any(|stream| langs.contains(&stream.lang_code.as_str()));

            if !satisfies_language {
                return false;
            }

            let title_runtime = title.duration as f32;
            let actual_runtime = (movie.runtime * 60) as f32;

            title_runtime >= actual_runtime * 0.9 && title_runtime <= actual_runtime * 1.1
        })
        .cloned()
        .collect::<Vec<_>>();

    let mut filtered_disc = disc.clone();
    filtered_disc.titles = filtered_titles;

    Ok(filtered_disc)
}

pub async fn filter_tv_series_candidates(disc: Disc, langs: Vec<&str>, season: u16, episodes: Vec<u16>, tmdb_id: u32, tmdb_key: &str) -> Result<Disc, Box<dyn Error>> {
    let tv_series = TmdbClient::new(tmdb_key).get_tv_series(tmdb_id).await?;

    let episode_runtimes = tv_series.seasons[(season - 1) as usize]
        .episodes
        .iter()
        .filter(|episode| episodes.contains(&episode.episode_number))
        .cloned()
        .map(|episode| (episode.runtime * 60) as f32)
        .collect::<Vec<_>>();

    let filtered_titles = disc
        .titles
        .iter()
        .filter(|title| {
            if title.audio_streams.len() == 0 {
                return false;
            }

            let satisfies_language = title.audio_streams.iter().any(|stream| langs.contains(&stream.lang_code.as_str()));

            if !satisfies_language {
                return false;
            }

            let title_runtime = title.duration as f32;

            episode_runtimes
                .iter()
                .any(|runtime| title_runtime >= *runtime * 0.9 && title_runtime <= *runtime * 1.1)
        })
        .cloned()
        .collect::<Vec<_>>();

    let mut filtered_disc = disc.clone();
    filtered_disc.titles = filtered_titles;

    Ok(filtered_disc)
}
