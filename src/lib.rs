use lazy_static::lazy_static;
use regex::Regex;
use std::io::{self, BufRead, Write};

lazy_static! {
    static ref ANSI_REGEX: Regex = Regex::new(
        r"\x1B(?:[@-Z\\-_]|\[[0-?]*[ -/]*[@-~])"
    ).unwrap();
}

pub struct StripStats {
    pub bytes_read: usize,
    pub bytes_written: usize,
    pub sequences_removed: usize,
}

impl StripStats {
    pub fn new() -> Self {
        Self {
            bytes_read: 0,
            bytes_written: 0,
            sequences_removed: 0,
        }
    }
}

pub fn strip_ansi(input: &str) -> String {
    ANSI_REGEX.replace_all(input, "").to_string()
}

pub fn strip_ansi_with_stats(input: &str) -> (String, usize) {
    let matches = ANSI_REGEX.find_iter(input).count();
    let result = ANSI_REGEX.replace_all(input, "").to_string();
    (result, matches)
}

pub fn process_stream<R: BufRead, W: Write>(
    reader: R,
    writer: &mut W,
    verbose: bool,
) -> io::Result<StripStats> {
    let mut stats = StripStats::new();

    for line in reader.lines() {
        let line = line?;
        stats.bytes_read += line.len() + 1;

        let (cleaned, sequences) = strip_ansi_with_stats(&line);
        stats.sequences_removed += sequences;
        stats.bytes_written += cleaned.len() + 1;

        writeln!(writer, "{}", cleaned)?;
    }

    if verbose {
        eprintln!(
            "Processed {} bytes, wrote {} bytes, removed {} ANSI sequences",
            stats.bytes_read, stats.bytes_written, stats.sequences_removed
        );
    }

    Ok(stats)
}

pub fn process_string(input: &str, verbose: bool) -> (String, StripStats) {
    let mut stats = StripStats::new();
    stats.bytes_read = input.len();

    let (result, sequences) = strip_ansi_with_stats(input);
    stats.bytes_written = result.len();
    stats.sequences_removed = sequences;

    if verbose {
        eprintln!(
            "Processed {} bytes, wrote {} bytes, removed {} ANSI sequences",
            stats.bytes_read, stats.bytes_written, stats.sequences_removed
        );
    }

    (result, stats)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    #[test]
    fn test_strip_ansi_basic_colors() {
        let input = "\x1B[31mRed text\x1B[0m";
        let result = strip_ansi(input);
        assert_eq!(result, "Red text");
    }

    #[test]
    fn test_strip_ansi_multiple_sequences() {
        let input = "\x1B[1m\x1B[32mBold Green\x1B[0m Normal";
        let result = strip_ansi(input);
        assert_eq!(result, "Bold Green Normal");
    }

    #[test]
    fn test_strip_ansi_cursor_movement() {
        let input = "Text\x1B[2J\x1B[H with cursor codes";
        let result = strip_ansi(input);
        assert_eq!(result, "Text with cursor codes");
    }

    #[test]
    fn test_strip_ansi_preserves_plain_text() {
        let input = "Plain text with no codes";
        let result = strip_ansi(input);
        assert_eq!(result, "Plain text with no codes");
    }

    #[test]
    fn test_strip_ansi_empty_string() {
        let result = strip_ansi("");
        assert_eq!(result, "");
    }

    #[test]
    fn test_strip_ansi_with_stats_counts_sequences() {
        let input = "\x1B[31mRed\x1B[0m \x1B[32mGreen\x1B[0m";
        let (result, count) = strip_ansi_with_stats(input);
        assert_eq!(result, "Red Green");
        assert_eq!(count, 4);
    }

    #[test]
    fn test_strip_ansi_with_stats_no_sequences() {
        let input = "Plain text";
        let (result, count) = strip_ansi_with_stats(input);
        assert_eq!(result, "Plain text");
        assert_eq!(count, 0);
    }

    #[test]
    fn test_process_stream_basic() {
        let input = "\x1B[31mLine 1\x1B[0m\nLine 2\n";
        let reader = Cursor::new(input);
        let mut writer = Vec::new();

        let stats = process_stream(reader, &mut writer, false).unwrap();
        let output = String::from_utf8(writer).unwrap();

        assert_eq!(output, "Line 1\nLine 2\n");
        assert_eq!(stats.sequences_removed, 2);
    }

    #[test]
    fn test_process_stream_empty_input() {
        let reader = Cursor::new("");
        let mut writer = Vec::new();

        let stats = process_stream(reader, &mut writer, false).unwrap();
        assert_eq!(stats.bytes_read, 0);
        assert_eq!(stats.sequences_removed, 0);
    }

    #[test]
    fn test_process_string_with_stats() {
        let input = "\x1B[1mBold\x1B[0m text";
        let (result, stats) = process_string(input, false);

        assert_eq!(result, "Bold text");
        assert_eq!(stats.sequences_removed, 2);
        assert!(stats.bytes_read > stats.bytes_written);
    }

    #[test]
    fn test_strip_ansi_malformed_sequences() {
        let input = "Text \x1B[incomplete";
        let result = strip_ansi(input);
        assert!(result.contains("Text"));
    }

    #[test]
    fn test_strip_ansi_preserves_whitespace() {
        let input = "  \x1B[31mIndented\x1B[0m  ";
        let result = strip_ansi(input);
        assert_eq!(result, "  Indented  ");
    }
}
