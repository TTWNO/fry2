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
/// # use fry_common::regex::{WHITESPACE_PATTERN, RegexExt};
/// assert_eq!(WHITESPACE_PATTERN.match_perfect(" \n "), true);
/// assert_eq!(WHITESPACE_PATTERN.match_perfect(" "), true);
/// assert_eq!(WHITESPACE_PATTERN.match_perfect("\t\n\n\t"), true);
/// assert_eq!(WHITESPACE_PATTERN.match_perfect("\tt"), false);
/// assert_eq!(WHITESPACE_PATTERN.match_perfect("abc"), false);
/// ```
pub static WHITESPACE_PATTERN: &Lazy<Regex> = regex!(r"[ \t\n\r]+");

/// Matches any letters of the alphabet.
/// ```
/// # use fry_common::regex::{ALPHABETIC_PATTERN, RegexExt};
/// assert_eq!(ALPHABETIC_PATTERN.match_perfect("abc"), true);
/// assert_eq!(ALPHABETIC_PATTERN.match_perfect("Quantum"), true);
/// // this matches both 'open' and 'source', but not the entire string
/// assert_eq!(ALPHABETIC_PATTERN.match_perfect("open source"), false);
/// // matches each word individually, but not the sentence
/// assert_eq!(ALPHABETIC_PATTERN.match_perfect("A small sentence"), false);
/// // matches eveything which is not "12" or the space
/// assert_eq!(ALPHABETIC_PATTERN.match_perfect("12 Days of Chrismas"), false);
/// // matches "etc"
/// assert_eq!(ALPHABETIC_PATTERN.match_perfect("etc..."), false);
/// assert_eq!(ALPHABETIC_PATTERN.match_perfect("12"), false);
/// assert_eq!(ALPHABETIC_PATTERN.match_perfect("$100"), false);
/// assert_eq!(ALPHABETIC_PATTERN.match_perfect("$100,000"), false);
/// assert_eq!(ALPHABETIC_PATTERN.match_perfect("99"), false);
/// ```
pub static ALPHABETIC_PATTERN: &Lazy<Regex> = regex!("[A-Za-z]+");


/// Matches any uppercase letters.
/// ```
/// # use fry_common::regex::{UPPERCASE_PATTERN, RegexExt};
/// assert_eq!(UPPERCASE_PATTERN.match_perfect("hello"), false);
/// assert_eq!(UPPERCASE_PATTERN.match_perfect("Hello"), false);
/// assert_eq!(UPPERCASE_PATTERN.match_perfect("beep"), false);
/// assert_eq!(UPPERCASE_PATTERN.match_perfect("BEEP"), true);
/// assert_eq!(UPPERCASE_PATTERN.match_perfect("123"), false);
/// ```
pub static UPPERCASE_PATTERN: &Lazy<Regex> = regex!("[A-Z]+");

/// Matches any lowercase letters.
/// ```
/// # use fry_common::regex::{LOWERCASE_PATTERN, RegexExt};
/// assert_eq!(LOWERCASE_PATTERN.match_perfect("hello"), true);
/// // matches "ello"
/// assert_eq!(LOWERCASE_PATTERN.match_perfect("Hello"), false);
/// assert_eq!(LOWERCASE_PATTERN.match_perfect("beep"), true);
/// assert_eq!(LOWERCASE_PATTERN.match_perfect("BEEP"), false);
/// assert_eq!(LOWERCASE_PATTERN.match_perfect("123"), false);
/// ```
pub static LOWERCASE_PATTERN: &Lazy<Regex> = regex!("[a-z]+");

/// Matches any digits.
/// NOTE: I am unsure of the reason the pattern used is `[0-9][0-9]*`.
/// It seems that this is useless and it could just be `[0-9]+`.
/// But for now I don't want to deviate from the pattern used in flite.
/// As mentioned in the module docs, many of these may be able to be simplified.
/// ```
/// # use fry_common::regex::{DIGIT_PATTERN, RegexExt};
/// assert_eq!(DIGIT_PATTERN.match_perfect("12"), true);
/// assert_eq!(DIGIT_PATTERN.match_perfect("9"), true);
/// // matches "9", but not "9th"
/// assert_eq!(DIGIT_PATTERN.match_perfect("9th"), false);
/// assert_eq!(DIGIT_PATTERN.match_perfect("hello"), false);
/// // matches "89", and "99", but not "89.99"
/// assert_eq!(DIGIT_PATTERN.match_perfect("89.99"), false);
/// ```
pub static DIGIT_PATTERN: &Lazy<Regex> = regex!("[0-9][0-9]*");

/// Matches an "identifier" which has a similar syntax to C varable names:
///
/// - The first character can be an alphabetic character, or `_`
/// - Any following characters can be any alphanumeric character, or `_`
/// ```
/// # use fry_common::regex::{IDENTIFIER_PATTERN, RegexExt};
/// assert_eq!(IDENTIFIER_PATTERN.match_perfect("x"), false);
/// assert_eq!(IDENTIFIER_PATTERN.match_perfect("_x"), true);
/// assert_eq!(IDENTIFIER_PATTERN.match_perfect("_abc"), true);
/// assert_eq!(IDENTIFIER_PATTERN.match_perfect("_abc1"), true);
/// assert_eq!(IDENTIFIER_PATTERN.match_perfect("bb"), true);
/// assert_eq!(IDENTIFIER_PATTERN.match_perfect("1"), false);
/// assert_eq!(IDENTIFIER_PATTERN.match_perfect("123b"), false);
/// // with match on "bb", but not "123bb"
/// assert_eq!(IDENTIFIER_PATTERN.match_perfect("123bb"), false);
/// ```
pub static IDENTIFIER_PATTERN: &Lazy<Regex> = regex!("[A-Za-z_][0-9A-Za-z_]+");

/// Matches any digits or letters of the alphabet.
/// ```
/// # use fry_common::regex::{ALPHANUMERIC_PATTERN, RegexExt};
/// assert_eq!(ALPHANUMERIC_PATTERN.match_perfect("Hello"), true);
/// assert_eq!(ALPHANUMERIC_PATTERN.match_perfect("world"), true);
/// assert_eq!(ALPHANUMERIC_PATTERN.match_perfect("2nd"), true);
/// assert_eq!(ALPHANUMERIC_PATTERN.match_perfect("2ND"), true);
/// assert_eq!(ALPHANUMERIC_PATTERN.match_perfect("57"), true);
/// assert_eq!(ALPHANUMERIC_PATTERN.match_perfect(","), false);
/// assert_eq!(ALPHANUMERIC_PATTERN.match_perfect(";'"), false);
/// assert_eq!(ALPHANUMERIC_PATTERN.match_perfect("Parents'"), false);
/// ```
pub static ALPHANUMERIC_PATTERN: &Lazy<Regex> = regex!("[0-9A-Za-z]+");

/// Matches a word that ends with either "'s" or "'S"
/// ```
/// # use fry_common::regex::{APOSTROPHES_PATTERN, RegexExt};
/// assert_eq!(APOSTROPHES_PATTERN.match_perfect("child"), false);
/// assert_eq!(APOSTROPHES_PATTERN.match_perfect("child's"), true);
/// assert_eq!(APOSTROPHES_PATTERN.match_perfect("PARENT'S"), true);
/// assert_eq!(APOSTROPHES_PATTERN.match_perfect("Parent's"), true);
/// assert_eq!(APOSTROPHES_PATTERN.match_perfect("Parents'"), false);
/// ```
pub static APOSTROPHES_PATTERN: &Lazy<Regex> = regex!(r".*'[sS]$");
/// Matches any integer that ends in "1st", "2nd", "3rd", or "th"
/// ```
/// # use fry_common::regex::{ORDINAL_PATTERN, RegexExt};
/// assert_eq!(ORDINAL_PATTERN.match_perfect("1st"), true);
/// assert_eq!(ORDINAL_PATTERN.match_perfect("2nd"), true);
/// assert_eq!(ORDINAL_PATTERN.match_perfect("3rd"), true);
/// assert_eq!(ORDINAL_PATTERN.match_perfect("25th"), true);
/// assert_eq!(ORDINAL_PATTERN.match_perfect("85th"), true);
/// assert_eq!(ORDINAL_PATTERN.match_perfect("260th"), true);
/// assert_eq!(ORDINAL_PATTERN.match_perfect("261st"), true);
/// assert_eq!(ORDINAL_PATTERN.match_perfect("262nd"), true);
/// assert_eq!(ORDINAL_PATTERN.match_perfect("263rd"), true);
/// assert_eq!(ORDINAL_PATTERN.match_perfect("264th"), true);
/// assert_eq!(ORDINAL_PATTERN.match_perfect("3nd"), false);
/// assert_eq!(ORDINAL_PATTERN.match_perfect("th"), false);
/// assert_eq!(ORDINAL_PATTERN.match_perfect("1th"), false);
/// assert_eq!(ORDINAL_PATTERN.match_perfect("1nd"), false);
/// assert_eq!(ORDINAL_PATTERN.match_perfect("1rd"), false);
/// ```
pub static ORDINAL_PATTERN: &Lazy<Regex> = regex!("[0-9]*(1st|2nd|3rd|[4-90]th)");
/// Matches any abbreviation which TODO
pub static ABBR_PATTERN: &Lazy<Regex> = regex!(r"([A-Za-z]\.)+[A-Za-z]\.?");
/// Matches American-style numbers, including punctuation (although it will not match just "any"
/// number)
/// ```
/// # use fry_common::regex::{COMMA_NUMBER_PATTERN, RegexExt};
/// assert_eq!(COMMA_NUMBER_PATTERN.match_perfect("512,622"), true);
/// assert_eq!(COMMA_NUMBER_PATTERN.match_perfect("1,000"), true);
/// assert_eq!(COMMA_NUMBER_PATTERN.match_perfect("1,000.99"), true);
/// assert_eq!(COMMA_NUMBER_PATTERN.match_perfect("1.000,99"), false);
/// assert_eq!(COMMA_NUMBER_PATTERN.match_perfect("1.000,99"), false);
/// assert_eq!(COMMA_NUMBER_PATTERN.match_perfect("12.99"), false);
/// assert_eq!(COMMA_NUMBER_PATTERN.match_perfect("55"), false);
/// ```
pub static COMMA_NUMBER_PATTERN: &Lazy<Regex> = regex!(r"[0-9][0-9]?[0-9]?,([0-9][0-9][0-9],)*[0-9][0-9][0-9](\.[0-9]+)?");

/// Matches punctuation clusters
/// ```
/// # use fry_common::regex::{PUNCTUATION_PATTERN, RegexExt};
/// assert_eq!(PUNCTUATION_PATTERN.match_perfect("(]"), true);
/// assert_eq!(PUNCTUATION_PATTERN.match_perfect("[.?!..,.,.,,[]"), true);
/// assert_eq!(PUNCTUATION_PATTERN.match_perfect(",.!?"), true);
/// assert_eq!(PUNCTUATION_PATTERN.match_perfect("]?"), true);
/// assert_eq!(PUNCTUATION_PATTERN.match_perfect("a"), false);
/// assert_eq!(PUNCTUATION_PATTERN.match_perfect("XD"), false);
/// assert_eq!(PUNCTUATION_PATTERN.match_perfect("(hello)"), false);
/// ```
pub static PUNCTUATION_PATTERN: &Lazy<Regex> = regex!(r"[,\.\?\!\]\[\)\(]+");

/// Matches positive, negative decimals (and their scientific notation), as well as plain integers
/// without decimal points.
/// ```
/// # use fry_common::regex::{DECIMAL_PATTERN, RegexExt};
/// assert_eq!(DECIMAL_PATTERN.match_perfect("1"), true);
/// assert_eq!(DECIMAL_PATTERN.match_perfect("10"), true);
/// assert_eq!(DECIMAL_PATTERN.match_perfect("10.5"), true);
/// // matches entire expression
/// assert_eq!(DECIMAL_PATTERN.match_perfect("3.9e15"), true);
/// assert_eq!(DECIMAL_PATTERN.match_perfect("-3.9E15"), true);
/// // no matches
/// assert_eq!(DECIMAL_PATTERN.match_perfect("one"), false);
/// // matches "1"
/// assert_eq!(DECIMAL_PATTERN.match_perfect("one1"), false);
/// ```
pub static DECIMAL_PATTERN: &Lazy<Regex> = regex!(r"-?(([0-9]+.[0-9]*)|([0-9]+)|(.[0-9]+))([eE][---+]?[0-9]+)?");

/// All characters considered punctuation for the purposes of the TTS engine.
/// Note how these are trimmed down from the list of [all ASCII pronunciation in the standard
/// library](char::is_ascii_punctuation)
pub const PUNCTUATION_CHARACTERS: &str = "'`.,:;!?{}[]()-\"";

/// Extension to regexes, made to simplify testing and development.
pub trait RegexExt {
    /// Return a reference to the [`Regex`].
    fn regex(&self) -> &Regex;
    /// Match the entire string instead of a portion of it.
    fn match_perfect(&self, haystack: &str) -> bool {
        let reg = self.regex();
        let Some(mtch) = reg.find(haystack) else {
            return false;
        };
        mtch.as_str() == haystack
    }
}
impl RegexExt for Regex {
    fn regex(&self) -> &Regex {
        self
    }
}

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
            assert_eq!($pat.match_perfect($haystack), $tf);
        }
    }
    #[test]
    fn test_one_number() {
        assert_matches!(WHITESPACE_PATTERN, REGEX_TESTS[0], false);
        assert_matches!(ALPHABETIC_PATTERN, REGEX_TESTS[0], false);
        assert_matches!(UPPERCASE_PATTERN, REGEX_TESTS[0], false);
        assert_matches!(LOWERCASE_PATTERN, REGEX_TESTS[0], false);
        assert_matches!(ALPHANUMERIC_PATTERN, REGEX_TESTS[0], true);
        assert_matches!(IDENTIFIER_PATTERN, REGEX_TESTS[0], false);
        assert_matches!(DIGIT_PATTERN, REGEX_TESTS[0], true);
        assert_matches!(DECIMAL_PATTERN, REGEX_TESTS[0], true);
        assert_matches!(COMMA_NUMBER_PATTERN, REGEX_TESTS[0], false);
    }
}
