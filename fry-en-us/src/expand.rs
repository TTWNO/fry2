//! Expand text like "$3.14" to "three dollars and fourteen cents".
//!
//! Contains expansions for:
//! - Roman numerals
//! - numbers
//! - ordinal numbers
//! - real numbers
//!

/// Read roman numerals into a `usize`.
/// Ignores all characters other than those in "IVXLCDM".
///
/// NOTE: this function can only take uppercase roman numerals
///
/// ```rust
/// # use fry_en_us::expand::expand_roman;
/// assert_eq!(expand_roman("III"), 3);
/// assert_eq!(expand_roman("IV"), 4);
/// assert_eq!(expand_roman("IX"), 9);
/// assert_eq!(expand_roman("LVIII"), 58);
/// assert_eq!(expand_roman("XIV"), 14);
/// assert_eq!(expand_roman("XV"), 15);
/// assert_eq!(expand_roman("L"), 50);
/// assert_eq!(expand_roman("XL"), 40);
/// assert_eq!(expand_roman("LX"), 60);
/// // the year I wrote this code!
/// assert_eq!(expand_roman("MMXXIV"), 2024);
/// ```
pub fn expand_roman<'a>(s: &'a str) -> usize {
    s.chars()
        // reverso the order of characters
        .rev()
        .filter_map(|c| match c {
            'I' => Some(1),
            'V' => Some(5),
            'X' => Some(10),
            'L' => Some(50),
            'C' => Some(100),
            'D' => Some(500),
            'M' => Some(1000),
            _ => None,
        })
        .fold(0, |acc, v| {
            // if the next value (v) multiplied by 4, is larger than the current value, then
            // subtract instead of add
            // see: https://dev.to/seanpgallivan/solution-roman-to-integer-567f
            if v << 2 < acc {
                acc - v
            } else {
                acc + v
            }
        })
}
