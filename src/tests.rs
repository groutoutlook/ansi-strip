#[cfg(test)]
mod tests {
    use ansi_strip::{strip_ansi, strip_ansi_with_stats};

    #[test]
    fn test_basic_color_codes() {
        let input = "\x1b[31mRed text\x1b[0m";
        assert_eq!(strip_ansi(input), "Red text");
    }

    #[test]
    fn test_multiple_sequences() {
        let input = "\x1b[1m\x1b[31mBold Red\x1b[0m Normal";
        assert_eq!(strip_ansi(input), "Bold Red Normal");
    }

    #[test]
    fn test_cursor_movement() {
        let input = "Text\x1b[2J\x1b[H with cursor codes";
        assert_eq!(strip_ansi(input), "Text with cursor codes");
    }

    #[test]
    fn test_256_colors() {
        let input = "\x1b[38;5;208mOrange\x1b[0m";
        assert_eq!(strip_ansi(input), "Orange");
    }

    #[test]
    fn test_rgb_colors() {
        let input = "\x1b[38;2;255;0;0mRGB Red\x1b[0m";
        assert_eq!(strip_ansi(input), "RGB Red");
    }

    #[test]
    fn test_no_ansi_codes() {
        let input = "Plain text without codes";
        assert_eq!(strip_ansi(input), "Plain text without codes");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(strip_ansi(""), "");
    }

    #[test]
    fn test_stats_counting() {
        let input = "\x1b[31mRed\x1b[0m \x1b[32mGreen\x1b[0m";
        let (result, count) = strip_ansi_with_stats(input);
        assert_eq!(result, "Red Green");
        assert_eq!(count, 4);
    }

    #[test]
    fn test_malformed_sequences() {
        let input = "Text \x1b[99 incomplete \x1b more text";
        let result = strip_ansi(input);
        assert!(result.contains("incomplete"));
        assert!(result.contains("more text"));
    }

    #[test]
    fn test_preserves_whitespace() {
        let input = "  \x1b[31mIndented\x1b[0m  ";
        assert_eq!(strip_ansi(input), "  Indented  ");
    }
}
