//! Defenitions and character sets required for TTS pattern recognition.
//! TODO: a lot of these are very simple patterns that could be replaced with much simpler
//! functions that do not require a regex engine (although some still do).

use lazy_regex::{Regex, Lazy, regex};

/* Taken and modified from Festival: `src/modules/Text/token.cc`
 * Some of these are right in the source code of Festival, but others were reverse engineered from
 * comparing the output of the `./bin/compile_regexes` tool from `flite` with the source code in
 * `src/cst_regex_defs.h`.
 * That said, these are (mostly) simple patterns that could probably be replaced with much simpler
 * logic.
 * */

/// Matches sections of text that are contain whitespace.
/// ```
/// # use fry_common::regex::WHITESPACE_PATTERN;
/// assert_eq!(WHITESPACE_PATTERN.is_match(" \n "), true);
/// assert_eq!(WHITESPACE_PATTERN.is_match(" "), true);
/// assert_eq!(WHITESPACE_PATTERN.is_match("\t\n\n\t"), true);
/// assert_eq!(WHITESPACE_PATTERN.is_match("\tt"), true);
/// assert_eq!(WHITESPACE_PATTERN.is_match("abc"), false);
/// ```
pub static WHITESPACE_PATTERN: &Lazy<Regex> = regex!(r"[ \t\n\r]+");

/// Matches any letters of the alphabet.
/// ```
/// # use fry_common::regex::ALPHABETIC_PATTERN;
/// assert_eq!(ALPHABETIC_PATTERN.is_match("abc"), true);
/// assert_eq!(ALPHABETIC_PATTERN.is_match("Quantum"), true);
/// // this matches both 'open' and 'source'
/// assert_eq!(ALPHABETIC_PATTERN.is_match("open source"), true);
/// // matches each word individually
/// assert_eq!(ALPHABETIC_PATTERN.is_match("A small sentence"), true);
/// // matches eveything which is not "12" or the space
/// assert_eq!(ALPHABETIC_PATTERN.is_match("12 Days of Chrismas"), true);
/// // matches "etc"
/// assert_eq!(ALPHABETIC_PATTERN.is_match("etc..."), true);
/// assert_eq!(ALPHABETIC_PATTERN.is_match("12"), false);
/// assert_eq!(ALPHABETIC_PATTERN.is_match("$100"), false);
/// assert_eq!(ALPHABETIC_PATTERN.is_match("$100,000"), false);
/// assert_eq!(ALPHABETIC_PATTERN.is_match("99"), false);
/// ```
pub static ALPHABETIC_PATTERN: &Lazy<Regex> = regex!("[A-Za-z]+");


/// Matches any uppercase letters.
/// ```
/// # use fry_common::regex::UPPERCASE_PATTERN;
/// assert_eq!(UPPERCASE_PATTERN.is_match("hello"), false);
/// // matches "H"
/// assert_eq!(UPPERCASE_PATTERN.is_match("Hello"), true);
/// assert_eq!(UPPERCASE_PATTERN.is_match("beep"), false);
/// assert_eq!(UPPERCASE_PATTERN.is_match("BEEP"), true);
/// assert_eq!(UPPERCASE_PATTERN.is_match("123"), false);
/// ```
pub static UPPERCASE_PATTERN: &Lazy<Regex> = regex!("[A-Z]+");

/// Matches any lowercase letters.
/// ```
/// # use fry_common::regex::LOWERCASE_PATTERN;
/// assert_eq!(LOWERCASE_PATTERN.is_match("hello"), true);
/// // matches "ello"
/// assert_eq!(LOWERCASE_PATTERN.is_match("Hello"), true);
/// assert_eq!(LOWERCASE_PATTERN.is_match("beep"), true);
/// assert_eq!(LOWERCASE_PATTERN.is_match("BEEP"), false);
/// assert_eq!(LOWERCASE_PATTERN.is_match("123"), false);
/// ```
pub static LOWERCASE_PATTERN: &Lazy<Regex> = regex!("[a-z]+");

/// Matches any digits.
/// NOTE: I am unsure of the reason the pattern used is `[0-9][0-9]*`.
/// It seems that this is useless and it could just be `[0-9]+`.
/// But for now I don't want to deviate from the pattern used in flite.
/// As mentioned in the module docs, many of these may be able to be simplified.
/// ```
/// # use fry_common::regex::DIGIT_PATTERN;
/// assert_eq!(DIGIT_PATTERN.is_match("12"), true);
/// assert_eq!(DIGIT_PATTERN.is_match("9"), true);
/// // matches "9"
/// assert_eq!(DIGIT_PATTERN.is_match("9th"), true);
/// assert_eq!(DIGIT_PATTERN.is_match("hello"), false);
/// // matches "89", and "99"
/// assert_eq!(DIGIT_PATTERN.is_match("89.99"), true);
/// ```
pub static DIGIT_PATTERN: &Lazy<Regex> = regex!("[0-9][0-9]*");

/// Matches an "identifier" which has a similar syntax to C varable names:
///
/// - The first character can be an alphabetic character, or `_`
/// - Any following characters can be any alphanumeric character, or `_`
/// ```
/// # use fry_common::regex::IDENTIFIER_PATTERN;
/// assert_eq!(IDENTIFIER_PATTERN.is_match("x"), false);
/// assert_eq!(IDENTIFIER_PATTERN.is_match("_x"), true);
/// assert_eq!(IDENTIFIER_PATTERN.is_match("_abc"), true);
/// assert_eq!(IDENTIFIER_PATTERN.is_match("_abc1"), true);
/// assert_eq!(IDENTIFIER_PATTERN.is_match("1"), false);
/// assert_eq!(IDENTIFIER_PATTERN.is_match("123b"), false);
/// // with match on "bb", which is valid
/// assert_eq!(IDENTIFIER_PATTERN.is_match("123bb"), true);
/// ```
pub static IDENTIFIER_PATTERN: &Lazy<Regex> = regex!("[A-Za-z_][0-9A-Za-z_]+");

/// Matches any digits or letters of the alphabet.
/// ```
/// # use fry_common::regex::ALPHANUMERIC_PATTERN;
/// assert_eq!(ALPHANUMERIC_PATTERN.is_match("Hello world"), true);
/// assert_eq!(ALPHANUMERIC_PATTERN.is_match("2nd"), true);
/// assert_eq!(ALPHANUMERIC_PATTERN.is_match("2ND"), true);
/// assert_eq!(ALPHANUMERIC_PATTERN.is_match("57"), true);
/// assert_eq!(ALPHANUMERIC_PATTERN.is_match(","), false);
/// assert_eq!(ALPHANUMERIC_PATTERN.is_match(";'"), false);
/// ```
pub static ALPHANUMERIC_PATTERN: &Lazy<Regex> = regex!("[0-9A-Za-z]+");

/// Matches a word that ends with either "'s" or "'S"
/// ```
/// # use fry_common::regex::APOSTROPHES_PATTERN;
/// assert_eq!(APOSTROPHES_PATTERN.is_match("child"), false);
/// assert_eq!(APOSTROPHES_PATTERN.is_match("child's"), true);
/// assert_eq!(APOSTROPHES_PATTERN.is_match("PARENT'S"), true);
/// assert_eq!(APOSTROPHES_PATTERN.is_match("Parent's"), true);
/// assert_eq!(APOSTROPHES_PATTERN.is_match("Parents'"), false);
/// ```
pub static APOSTROPHES_PATTERN: &Lazy<Regex> = regex!(r".*'[sS]$");
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
pub static ABBR_PATTERN: &Lazy<Regex> = regex!(r"([A-Za-z]\.)+[A-Za-z]\.?");
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
pub static COMMA_NUMBER_PATTERN: &Lazy<Regex> = regex!(r"[0-9][0-9]?[0-9]?,([0-9][0-9][0-9],)*[0-9][0-9][0-9](\.[0-9]+)?");
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
pub static PUNCTUATION_PATTERN: &Lazy<Regex> = regex!(r"(\]|[-\[.,!?])+");

/// All characters considered punctuation for the purposes of the TTS engine.
/// Note how these are trimmed down from the list of [all ASCII pronunciation in the standard
/// library](char::is_ascii_punctuation)
pub const PUNCTUATION_CHARACTERS: &str = "'`.,:;!?{}[]()-\"";

/// A series of tests adapated from flite: `testsuite/regex_test_main.c`
#[cfg(test)]
pub mod test {
    const REGEX_TESTS: [&'static str; 11] = [
        "1",
        " \n ",
        "hello",
        "Hello",
        "1and2",
        "oneandtwo",
        "-1.43",
        "235",
        "034",
        "1,234,235",
        "1,2345",
    ];

    use super::*;
    macro_rules! assert_matches {
        ($pat:expr, $haystack:expr, $tf:expr) => {
            assert_eq!($pat.is_match($haystack), $tf);
        }
    }
    #[test]
    fn test_one_number() {
        assert_matches!(WHITESPACE_PATTERN, REGEX_TESTS[0], false);
        assert_matches!(COMMA_NUMBER_PATTERN, REGEX_TESTS[0], false);
        assert_matches!(ORDINAL_PATTERN, REGEX_TESTS[0], false);
    }
}
