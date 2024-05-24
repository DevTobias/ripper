pub fn parse_csv_line(line: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_field = String::new();
    let mut in_quotes = false;

    for c in line.chars() {
        match c {
            '"' => in_quotes = !in_quotes,
            ',' if in_quotes => current_field.push(c),
            ',' => result.push(std::mem::take(&mut current_field)),
            _ => current_field.push(c),
        }
    }

    result.push(current_field);
    result
}

pub fn parse_duration_to_seconds(duration: &str) -> Result<u32, &'static str> {
    let parts: Vec<&str> = duration.split(':').collect();

    if parts.len() != 3 {
        return Err("Invalid duration format");
    }

    let hours: u32 = parts[0].parse().map_err(|_| "Invalid hour format")?;
    let minutes: u32 = parts[1].parse().map_err(|_| "Invalid minute format")?;
    let seconds: u32 = parts[2].parse().map_err(|_| "Invalid second format")?;

    Ok(hours * 3600 + minutes * 60 + seconds)
}
