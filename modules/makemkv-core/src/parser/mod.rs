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
