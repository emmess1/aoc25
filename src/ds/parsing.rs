//! Parsing helpers commonly used in AoC: grids, ints, and tokenization.

/// Parse a grid of characters; each line becomes a Vec<char>.
pub fn parse_grid_chars(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|l| l.chars().collect()).collect()
}

/// Parse a grid of digits ('0'..'9') into i64 values; ignores other chars.
pub fn parse_grid_digits(input: &str) -> Vec<Vec<i64>> {
    input
        .lines()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_ascii_digit())
                .map(|c| (c as u8 - b'0') as i64)
                .collect()
        })
        .collect()
}

/// Parse all signed integers from whitespace-separated input into i64.
pub fn parse_ints_whitespace(input: &str) -> Vec<i64> {
    input
        .split_whitespace()
        .filter_map(|t| t.parse::<i64>().ok())
        .collect()
}

/// Parse each line into i64 (one number per line, trimming whitespace). Skips empty lines.
pub fn parse_lines_i64(input: &str) -> Vec<i64> {
    input
        .lines()
        .filter_map(|l| {
            let s = l.trim();
            if s.is_empty() {
                None
            } else {
                s.parse::<i64>().ok()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn grid_chars() {
        let g = parse_grid_chars("ab\ncd\n");
        assert_eq!(g, vec![vec!['a', 'b'], vec!['c', 'd']]);
    }
    #[test]
    fn grid_digits() {
        let g = parse_grid_digits("12\n90x\n");
        assert_eq!(g, vec![vec![1, 2], vec![9, 0]]);
    }
    #[test]
    fn ints_ws_and_lines() {
        let v = parse_ints_whitespace("1 -2 3\n4");
        assert_eq!(v, vec![1, -2, 3, 4]);
        let v2 = parse_lines_i64("\n10\n 20 \n\n-5\n");
        assert_eq!(v2, vec![10, 20, -5]);
    }
}
