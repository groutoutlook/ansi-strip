use ansi_strip::{process_stream, strip_ansi};
use std::io::Cursor;

#[test]
fn test_complex_ansi_sequences() {
    let input = "\x1B[1;31;40mBold Red on Black\x1B[0m\x1B[2J\x1B[H\x1B[4mUnderline\x1B[0m";
    let result = strip_ansi(input);
    assert_eq!(result, "Bold Red on BlackUnderline");
}

#[test]
fn test_256_color_codes() {
    let input = "\x1B[38;5;208mOrange\x1B[0m \x1B[48;5;21mBlue BG\x1B[0m";
    let result = strip_ansi(input);
    assert_eq!(result, "Orange Blue BG");
}

#[test]
fn test_rgb_color_codes() {
    let input = "\x1B[38;2;255;0;0mRGB Red\x1B[0m";
    let result = strip_ansi(input);
    assert_eq!(result, "RGB Red");
}

#[test]
fn test_multiline_stream_processing() {
    let input = "\x1B[31mRed line\x1B[0m\n\x1B[32mGreen line\x1B[0m\n\x1B[34mBlue line\x1B[0m\n";
    let reader = Cursor::new(input);
    let mut writer = Vec::new();

    let stats = process_stream(reader, &mut writer, false).unwrap();
    let output = String::from_utf8(writer).unwrap();

    assert_eq!(output, "Red line\nGreen line\nBlue line\n");
    assert_eq!(stats.sequences_removed, 6);
}

#[test]
fn test_large_input_performance() {
    let line = format!("\x1B[31m{}\x1B[0m", "x".repeat(1000));
    let input = format!("{}", line.repeat(100));
    let reader = Cursor::new(input);
    let mut writer = Vec::new();

    let result = process_stream(reader, &mut writer, false);
    assert!(result.is_ok());
    assert!(writer.len() > 0);
}

#[test]
fn test_mixed_content_preservation() {
    let input = "Normal \x1B[1mbold\x1B[0m text with\ttabs\nand\nnewlines";
    let result = strip_ansi(input);
    assert_eq!(result, "Normal bold text with\ttabs\nand\nnewlines");
}

#[test]
fn test_consecutive_ansi_codes() {
    let input = "\x1B[1m\x1B[31m\x1B[4mMultiple\x1B[0m\x1B[0m\x1B[0m";
    let result = strip_ansi(input);
    assert_eq!(result, "Multiple");
}
