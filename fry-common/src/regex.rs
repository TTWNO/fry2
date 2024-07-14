//! Defenitions and character sets required for TTS pattern recognition.

use lazy_regex::{Regex, Lazy, regex};

/* Taken and modified from Festival: src/modules/Text/token.cc */

/// Matches a word that ends with either "'s" or "'S"
/// ```
/// # use fry_common::regex::APOSTROPHES_PATTERN;
/// assert_eq!(APOSTROPHES_PATTERN.is_match("child"), false);
/// assert_eq!(APOSTROPHES_PATTERN.is_match("child's"), true);
/// assert_eq!(APOSTROPHES_PATTERN.is_match("PARENT'S"), true);
/// assert_eq!(APOSTROPHES_PATTERN.is_match("Parent's"), true);
/// assert_eq!(APOSTROPHES_PATTERN.is_match("Parents'"), false);
/// ```
pub static APOSTROPHES_PATTERN: &Lazy<Regex> = regex!(".*'[sS]$");
/// Matches any integer that ends in "1st", "2nd", "3rd", or "th"
/// ```
/// # use fry_common::regex::ORDINAL_PATTERN;
/// assert_eq!(ORDINAL_PATTERN.is_match("1st"), true);
/// assert_eq!(ORDINAL_PATTERN.is_match("2nd"), true);
/// assert_eq!(ORDINAL_PATTERN.is_match("3rd"), true);
/// assert_eq!(ORDINAL_PATTERN.is_match("25th"), true);
/// assert_eq!(ORDINAL_PATTERN.is_match("85th"), true);
/// assert_eq!(ORDINAL_PATTERN.is_match("260th"), true);
/// assert_eq!(ORDINAL_PATTERN.is_match("261st"), true);
/// assert_eq!(ORDINAL_PATTERN.is_match("262nd"), true);
/// assert_eq!(ORDINAL_PATTERN.is_match("263rd"), true);
/// assert_eq!(ORDINAL_PATTERN.is_match("264th"), true);
/// assert_eq!(ORDINAL_PATTERN.is_match("3nd"), false);
/// assert_eq!(ORDINAL_PATTERN.is_match("th"), false);
/// assert_eq!(ORDINAL_PATTERN.is_match("1th"), false);
/// assert_eq!(ORDINAL_PATTERN.is_match("1nd"), false);
/// assert_eq!(ORDINAL_PATTERN.is_match("1rd"), false);
/// ```
pub static ORDINAL_PATTERN: &Lazy<Regex> = regex!("[0-9]*(1st|2nd|3rd|[4-90]th)");
/// Matches any abbreviation which TODO
pub static ABBR_PATTERN: &Lazy<Regex> = regex!("([A-Za-z]\\.)+[A-Za-z]\\.?");
/// Matches American-style numbers, including punctuation (although it will not match just "any"
/// number)
/// ```
/// # use fry_common::regex::COMMA_NUMBER_PATTERN;
/// assert_eq!(COMMA_NUMBER_PATTERN.is_match("512,622"), true);
/// assert_eq!(COMMA_NUMBER_PATTERN.is_match("1,000"), true);
/// assert_eq!(COMMA_NUMBER_PATTERN.is_match("1,000.99"), true);
/// assert_eq!(COMMA_NUMBER_PATTERN.is_match("1.000,99"), false);
/// assert_eq!(COMMA_NUMBER_PATTERN.is_match("1.000,99"), false);
/// assert_eq!(COMMA_NUMBER_PATTERN.is_match("12.99"), false);
/// assert_eq!(COMMA_NUMBER_PATTERN.is_match("55"), false);
/// ```
pub static COMMA_NUMBER_PATTERN: &Lazy<Regex> = regex!("[0-9][0-9]?[0-9]?,([0-9][0-9][0-9],)*[0-9][0-9][0-9](\\.[0-9]+)?");
/// Matches punctuation clusters
/// ```
/// # use fry_common::regex::PUNCTUATION_PATTERN;
/// assert_eq!(PUNCTUATION_PATTERN.is_match("(]"), true);
/// assert_eq!(PUNCTUATION_PATTERN.is_match("[.?!..,.,.,,[]"), true);
/// assert_eq!(PUNCTUATION_PATTERN.is_match(",.!?"), true);
/// assert_eq!(PUNCTUATION_PATTERN.is_match("]?"), true);
/// assert_eq!(PUNCTUATION_PATTERN.is_match("a"), false);
/// assert_eq!(PUNCTUATION_PATTERN.is_match("XD"), false);
/// assert_eq!(PUNCTUATION_PATTERN.is_match("(hello)"), false);
/// ```
pub static PUNCTUATION_PATTERN: &Lazy<Regex> = regex!("(\\]|[-\\[.,!?])+");

/// All characters considered punctuation for the purposes of the TTS engine.
/// Note how these are trimmed down from the list of [all ASCII pronunciation in the standard
/// library](char::is_ascii_punctuation)
pub const PUNCTUATION_CHARACTERS: &str = "'`.,:;!?{}[]()-\"";
