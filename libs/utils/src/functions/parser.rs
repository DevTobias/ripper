/// Parses a single line of CSV-formatted text into a vector of fields.
///
/// This function takes a string slice representing a line of text in CSV format
/// and returns a vector of strings, where each string represents a field from
/// the line. It correctly handles fields enclosed in double quotes, allowing
/// for commas within quoted fields.
///
/// # Arguments
///
/// * `line` - A string slice that holds a single line of CSV-formatted text.
///
/// # Returns
///
/// * `Vec<String>` - A vector containing the parsed fields from the input line.
///
/// # Examples
///
/// ```
/// let line = r#""John, Doe",28,"New York, USA""#;
/// let parsed = parse_csv_line(line);
/// assert_eq!(parsed, vec!["John, Doe", "28", "New York, USA"]);
/// ```
///
/// The example above demonstrates parsing a CSV line where fields are quoted,
/// and commas within quotes are preserved as part of the field values.
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

/// Parses a duration string in the format "HH:MM:SS" and converts it to the total number of seconds.
///
/// This function takes a string slice representing a duration in "HH:MM:SS" format and returns
/// the total number of seconds as a `u32` value. If the input string does not match the required
/// format or contains invalid numerical values, an error is returned.
///
/// # Arguments
///
/// * `duration` - A string slice that holds the duration in "HH:MM:SS" format.
///
/// # Returns
///
/// * `Result<u32, &'static str>` - A result containing the total number of seconds if parsing
///   is successful, or an error message if the format is invalid.
///
/// # Errors
///
/// * Returns `"Invalid duration format"` if the input string does not contain exactly three parts
///   separated by colons.
/// * Returns `"Invalid hour format"`, `"Invalid minute format"`, or `"Invalid second format"`
///   if the respective parts of the duration string cannot be parsed into `u32` values.
///
/// # Examples
///
/// ```
/// let duration = "01:23:45";
/// match parse_duration_to_seconds(duration) {
///     Ok(seconds) => assert_eq!(seconds, 5025),
///     Err(e) => eprintln!("Error: {}", e),
/// }
/// ```
///
/// The example above demonstrates parsing a valid duration string and converting it to the
/// total number of seconds.
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
