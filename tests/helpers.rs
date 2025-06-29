use std::fs::{read_to_string, write};
use std::io::Result;
use std::path::Path;

pub fn remove_line_at_index<P: AsRef<Path>>(file_path: P, line_index: usize) -> Result<Option<String>> {
    let file_path = file_path.as_ref();

    let contents = read_to_string(file_path)?;
    let mut lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    let removed = if line_index < lines.len() { Some(lines.remove(line_index)) } else { None };

    let updated = lines.join("\n");
    write(file_path, updated)?;

    Ok(removed)
}

/// Inserts a line back into the file at the given index.
/// If the index is out of bounds, the line is appended to the end.
pub fn insert_line_at_index<P: AsRef<Path>>(file_path: P, line_index: usize, line: &str) -> Result<()> {
    let file_path = file_path.as_ref();

    // Read file into lines
    let contents = read_to_string(file_path)?;
    let mut lines: Vec<String> = contents.lines().map(|s| s.to_string()).collect();

    // Insert line at the desired index, or append if index is too big
    if line_index <= lines.len() {
        lines.insert(line_index, line.to_string());
    } else {
        lines.push(line.to_string());
    }

    // Write modified content back
    let updated = lines.join("\n");
    write(file_path, updated)?;

    Ok(())
}
