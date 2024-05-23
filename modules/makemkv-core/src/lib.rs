use std::{
    collections::HashMap,
    error::Error,
    hash::Hash,
    io::{BufRead, BufReader},
    process::{Command, Stdio},
};

use models::{Disc, Title};
use parser::parse_csv_line;

mod models;
mod parser;

pub fn read_disc_properties(command: &str) -> Result<Disc, Box<dyn Error>> {
    let process = Command::new(command)
        .args(["-r", "info", "dev:/dev/rdisk5"])
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = BufReader::new(process.stdout.ok_or("failed to capture stdout")?);

    let mut disc_map: HashMap<String, String> = HashMap::new();

    let mut titles: Vec<HashMap<String, String>> = Vec::new();
    let mut streams: HashMap<usize, Vec<HashMap<String, String>>> = HashMap::new();

    for line in stdout.lines() {
        let columns = parse_csv_line(&line?);

        match columns[0].as_str() {
            x if x.starts_with("CINFO:") => {
                let code = x.trim_start_matches("CINFO:").to_string();
                let value = columns.get(2).ok_or("missing value")?;
                disc_map.insert(code, value.to_string());
            }
            x if x.starts_with("TINFO:") => {
                let id: usize = x.trim_start_matches("TINFO:").parse()?;
                let code = columns.get(1).ok_or("missing code")?.to_string();
                let value = columns.get(3).ok_or("missing value")?.to_string();

                if titles.len() <= id {
                    titles.resize(id + 1, HashMap::new());
                    streams.insert(id, Vec::new());
                }

                titles[id].insert(code, value);
            }
            x if x.starts_with("SINFO:") => {
                let title_id: usize = x.trim_start_matches("SINFO:").parse()?;
                let id: usize = columns.get(1).ok_or("missing id")?.parse()?;
                let code = columns.get(2).ok_or("missing code")?.to_string();
                let value = columns.get(4).ok_or("missing value")?.to_string();

                let title = streams.get_mut(&title_id).ok_or("missing title")?;

                if title.len() <= id {
                    title.resize(id + 1, HashMap::new());
                }

                title[id].insert(code, value);
            }
            _ => {}
        }
    }

    println!("{:#?}", streams);

    let title_info_from_map =
        |mut title: HashMap<String, String>| -> Result<Title, Box<dyn Error>> {
            Ok(Title {
                name: title.remove("2").ok_or("missing value")?,
                chapter_count: title.remove("8").unwrap_or(String::from("0")),
                duration: title.remove("9").ok_or("missing value")?,
                disk_size: title.remove("10").ok_or("missing value")?,
                disk_size_bytes: title.remove("11").ok_or("missing value")?,
                source_file_name: title.remove("16").ok_or("missing value")?,
                segments_count: title.remove("25").ok_or("missing value")?,
                segments_map: title.remove("26").ok_or("missing value")?,
                output_file_name: title.remove("27").ok_or("missing value")?,
                metadata_language_code: title.remove("28").ok_or("missing value")?,
                metadata_language_name: title.remove("29").ok_or("missing value")?,
                tree_info: title.remove("30").ok_or("missing value")?,
                panel_title: title.remove("31").ok_or("missing value")?,
                order_weight: title.remove("33").ok_or("missing value")?,
                video_stream,
            })
        };

    Ok(Disc {
        disc_type: disc_map.remove("1").ok_or("missing value")?,
        name: disc_map.remove("2").ok_or("missing value")?,
        metadata_language_code: disc_map.remove("28").ok_or("missing value")?,
        metadata_language_name: disc_map.remove("29").ok_or("missing value")?,
        tree_info: disc_map.remove("30").ok_or("missing value")?,
        panel_title: disc_map.remove("31").ok_or("missing value")?,
        volume_name: disc_map.remove("32").ok_or("missing value")?,
        order_weight: disc_map.remove("33").ok_or("missing value")?,
        titles: titles
            .into_iter()
            .map(title_info_from_map)
            .collect::<Result<_, _>>()?,
    })
}
