//! Regex pattern recognition.
//! 
//! 
//! Defenitions and character sets required for TTS pattern recognition.
//! TODO: a lot of these are very simple patterns that could be replaced with much simpler
//! functions that do not require a regex engine (although some still do).

use regex::Regex;

/* Taken and modified from Festival: `src/modules/Text/token.cc`
 * Some of these are right in the source code of Festival, but others were reverse engineered from
 * comparing the output of the `./bin/compile_regexes` tool from `flite` with the source code in
 * `src/cst_regex_defs.h`.
 * That said, these are (mostly) simple patterns that could probably be replaced with much simpler
 * logic.
 * */

/// Matches sections of text that are contain whitespace.
/// ```
/// use fry_common::regex::{WHITESPACE_PATTERN, RegexExt};
/// use regex::Regex;
/// // initialize the regex
/// let whitespace_regex = Regex::new(WHITESPACE_PATTERN).expect("Valid regex pattern");
/// assert_eq!(whitespace_regex.match_perfect(" \n "), true);
/// assert_eq!(whitespace_regex.match_perfect(" "), true);
/// assert_eq!(whitespace_regex.match_perfect("\t\n\n\t"), true);
/// assert_eq!(whitespace_regex.match_perfect("\tt"), false);
/// assert_eq!(whitespace_regex.match_perfect("abc"), false);
/// ```
pub static WHITESPACE_PATTERN: &str = r"[ \t\n\r]+";

/// Matches any letters of the alphabet.
/// ```
/// use fry_common::regex::{ALPHABETIC_PATTERN, RegexExt};
/// use regex::Regex;
/// // initialize the regex
/// let alphabetic_regex = Regex::new(ALPHABETIC_PATTERN).expect("A valid regex");
/// assert_eq!(alphabetic_regex.match_perfect("abc"), true);
/// assert_eq!(alphabetic_regex.match_perfect("Quantum"), true);
/// // this matches both 'open' and 'source', but not the entire string
/// assert_eq!(alphabetic_regex.match_perfect("open source"), false);
/// // matches each word individually, but not the sentence
/// assert_eq!(alphabetic_regex.match_perfect("A small sentence"), false);
/// // matches eveything which is not "12" or the space
/// assert_eq!(alphabetic_regex.match_perfect("12 Days of Chrismas"), false);
/// // matches "etc"
/// assert_eq!(alphabetic_regex.match_perfect("etc..."), false);
/// assert_eq!(alphabetic_regex.match_perfect("12"), false);
/// assert_eq!(alphabetic_regex.match_perfect("$100"), false);
/// assert_eq!(alphabetic_regex.match_perfect("$100,000"), false);
/// assert_eq!(alphabetic_regex.match_perfect("99"), false);
/// ```
pub static ALPHABETIC_PATTERN: &str = r"[A-Za-z]+";

/// Matches any uppercase letters.
/// ```
/// use fry_common::regex::{UPPERCASE_PATTERN, RegexExt};
/// use regex::Regex;
/// // initialize the regex
/// let uppercase_regex = Regex::new(UPPERCASE_PATTERN).expect("Valid regex pattern");
/// assert_eq!(uppercase_regex.match_perfect("hello"), false);
/// assert_eq!(uppercase_regex.match_perfect("Hello"), false);
/// assert_eq!(uppercase_regex.match_perfect("beep"), false);
/// assert_eq!(uppercase_regex.match_perfect("BEEP"), true);
/// assert_eq!(uppercase_regex.match_perfect("123"), false);
/// ```
pub static UPPERCASE_PATTERN: &str = r"[A-Z]+";

/// Matches any lowercase letters.
/// ```
/// use fry_common::regex::{LOWERCASE_PATTERN, RegexExt};
/// use regex::Regex;
/// // initialize regex pattern
/// let lowercase_regex = Regex::new(LOWERCASE_PATTERN).expect("A valid regex pattern");
/// assert_eq!(lowercase_regex.match_perfect("hello"), true);
/// // matches "ello"
/// assert_eq!(lowercase_regex.match_perfect("Hello"), false);
/// assert_eq!(lowercase_regex.match_perfect("beep"), true);
/// assert_eq!(lowercase_regex.match_perfect("BEEP"), false);
/// assert_eq!(lowercase_regex.match_perfect("123"), false);
/// ```
pub static LOWERCASE_PATTERN: &str = r"[a-z]+";

/// Matches any digits.
/// 
/// NOTE: I am unsure of the reason the pattern used is `[0-9][0-9]*`.
/// It seems that this is useless and it could just be `[0-9]+`.
/// But for now I don't want to deviate from the pattern used in flite.
/// As mentioned in the module docs, many of these may be able to be simplified.
/// ```
/// use fry_common::regex::{DIGIT_PATTERN, RegexExt};
/// use regex::Regex;
/// // initialize the regex
/// let digit_regex = Regex::new(DIGIT_PATTERN).expect("A valid regex pattern");
/// assert_eq!(digit_regex.match_perfect("12"), true);
/// assert_eq!(digit_regex.match_perfect("9"), true);
/// // matches "9", but not "9th"
/// assert_eq!(digit_regex.match_perfect("9th"), false);
/// assert_eq!(digit_regex.match_perfect("hello"), false);
/// // matches "89", and "99", but not "89.99"
/// assert_eq!(digit_regex.match_perfect("89.99"), false);
/// ```
pub static DIGIT_PATTERN: &str = r"[0-9][0-9]*";

/// Matches an "identifier" which has a similar syntax to C varable names:
///
/// - The first character can be an alphabetic character, or `_`
/// - Any following characters can be any alphanumeric character, or `_`
/// ```
/// use fry_common::regex::{IDENTIFIER_PATTERN, RegexExt};
/// use regex::Regex;
/// // initialize the regex
/// let identifier_regex = Regex::new(IDENTIFIER_PATTERN).expect("A vaild regex pattern");
/// assert_eq!(identifier_regex.match_perfect("x"), false);
/// assert_eq!(identifier_regex.match_perfect("_x"), true);
/// assert_eq!(identifier_regex.match_perfect("_abc"), true);
/// assert_eq!(identifier_regex.match_perfect("_abc1"), true);
/// assert_eq!(identifier_regex.match_perfect("bb"), true);
/// assert_eq!(identifier_regex.match_perfect("1"), false);
/// assert_eq!(identifier_regex.match_perfect("123b"), false);
/// // with match on "bb", but not "123bb"
/// assert_eq!(identifier_regex.match_perfect("123bb"), false);
/// ```
pub static IDENTIFIER_PATTERN: &str = r"[A-Za-z_][0-9A-Za-z_]+";

/// Matches any digits or letters of the alphabet.
/// ```
/// use fry_common::regex::{ALPHANUMERIC_PATTERN, RegexExt};
/// use regex::Regex;
/// // initialize the regex
/// let alphanumeric_regex = Regex::new(ALPHANUMERIC_PATTERN).expect("A valid regex pattern");
/// assert_eq!(alphanumeric_regex.match_perfect("Hello"), true);
/// assert_eq!(alphanumeric_regex.match_perfect("world"), true);
/// assert_eq!(alphanumeric_regex.match_perfect("2nd"), true);
/// assert_eq!(alphanumeric_regex.match_perfect("2ND"), true);
/// assert_eq!(alphanumeric_regex.match_perfect("57"), true);
/// assert_eq!(alphanumeric_regex.match_perfect(","), false);
/// assert_eq!(alphanumeric_regex.match_perfect(";'"), false);
/// assert_eq!(alphanumeric_regex.match_perfect("Parents'"), false);
/// ```
pub static ALPHANUMERIC_PATTERN: &str = r"[0-9A-Za-z]+";

/// Matches a word that ends with either "'s" or "'S"
/// ```
/// use fry_common::regex::{APOSTROPHES_PATTERN, RegexExt};
/// use regex::Regex;
/// // initialize the regex
/// let apostrophes_regex = Regex::new(APOSTROPHES_PATTERN).expect("A valid regex pattern");
/// assert_eq!(apostrophes_regex.match_perfect("child"), false);
/// assert_eq!(apostrophes_regex.match_perfect("child's"), true);
/// assert_eq!(apostrophes_regex.match_perfect("PARENT'S"), true);
/// assert_eq!(apostrophes_regex.match_perfect("Parent's"), true);
/// assert_eq!(apostrophes_regex.match_perfect("Parents'"), false);
/// ```
pub static APOSTROPHES_PATTERN: &str = r".*'[sS]$";

/// Matches any integer that ends in "1st", "2nd", "3rd", or "th"
/// ```
/// use fry_common::regex::{ORDINAL_PATTERN, RegexExt};
/// use regex::Regex;
/// // initialize the regex
/// let ordinal_regex = Regex::new(ORDINAL_PATTERN).expect("A valid regex pattern");
/// assert_eq!(ordinal_regex.match_perfect("1st"), true);
/// assert_eq!(ordinal_regex.match_perfect("2nd"), true);
/// assert_eq!(ordinal_regex.match_perfect("3rd"), true);
/// assert_eq!(ordinal_regex.match_perfect("25th"), true);
/// assert_eq!(ordinal_regex.match_perfect("85th"), true);
/// assert_eq!(ordinal_regex.match_perfect("260th"), true);
/// assert_eq!(ordinal_regex.match_perfect("261st"), true);
/// assert_eq!(ordinal_regex.match_perfect("262nd"), true);
/// assert_eq!(ordinal_regex.match_perfect("263rd"), true);
/// assert_eq!(ordinal_regex.match_perfect("264th"), true);
/// assert_eq!(ordinal_regex.match_perfect("3nd"), false);
/// assert_eq!(ordinal_regex.match_perfect("th"), false);
/// assert_eq!(ordinal_regex.match_perfect("1th"), false);
/// assert_eq!(ordinal_regex.match_perfect("1nd"), false);
/// assert_eq!(ordinal_regex.match_perfect("1rd"), false);
/// ```
pub static ORDINAL_PATTERN: &str = r"[0-9]*(1st|2nd|3rd|[4-90]th)";

/// Matches any abbreviation which TODO
pub static ABBR_PATTERN: &str = r"([A-Za-z]\.)+[A-Za-z]\.?";

/// Matches American-style numbers, including punctuation (although it will not match just "any"
/// number)
/// ```
/// use fry_common::regex::{COMMA_NUMBER_PATTERN, RegexExt};
/// use regex::Regex;
/// // initialize the regex
/// let comma_number_regex = Regex::new(COMMA_NUMBER_PATTERN).expect("A valid regex pattern");
/// assert_eq!(comma_number_regex.match_perfect("512,622"), true);
/// assert_eq!(comma_number_regex.match_perfect("1,000"), true);
/// assert_eq!(comma_number_regex.match_perfect("1,000.99"), true);
/// assert_eq!(comma_number_regex.match_perfect("1.000,99"), false);
/// assert_eq!(comma_number_regex.match_perfect("1.000,99"), false);
/// assert_eq!(comma_number_regex.match_perfect("12.99"), false);
/// assert_eq!(comma_number_regex.match_perfect("55"), false);
/// assert_eq!(comma_number_regex.match_perfect("1,2345"), false);
/// ```
pub static COMMA_NUMBER_PATTERN: &str =
    r"[0-9][0-9]?[0-9]?,([0-9][0-9][0-9],)*[0-9][0-9][0-9](\.[0-9]+)?";

/// Matches punctuation clusters
/// ```
/// use fry_common::regex::{PUNCTUATION_PATTERN, RegexExt};
/// use regex::Regex;
/// let  punctuation_regex = Regex::new(PUNCTUATION_PATTERN).expect("A valid regex pattern");
/// assert_eq!(punctuation_regex.match_perfect("(]"), true);
/// assert_eq!(punctuation_regex.match_perfect("[.?!..,.,.,,[]"), true);
/// assert_eq!(punctuation_regex.match_perfect(",.!?"), true);
/// assert_eq!(punctuation_regex.match_perfect("]?"), true);
/// assert_eq!(punctuation_regex.match_perfect("a"), false);
/// assert_eq!(punctuation_regex.match_perfect("XD"), false);
/// assert_eq!(punctuation_regex.match_perfect("(hello)"), false);
/// ```
pub static PUNCTUATION_PATTERN: &str = r"[,\.\?\!\]\[\)\(]+";

/// Matches positive, negative decimals (and their scientific notation), as well as plain integers
/// without decimal points.
/// ```
/// use fry_common::regex::{DECIMAL_PATTERN, RegexExt};
/// use regex::Regex;
/// // initialize the regex
/// let decimal_regex = Regex::new(DECIMAL_PATTERN).expect("A valid regex pattern");
/// assert_eq!(decimal_regex.match_perfect("1"), true, "1 should match the decimal expression");
/// assert_eq!(decimal_regex.match_perfect("10"), true, "10 should match the decimal expression");
/// assert_eq!(decimal_regex.match_perfect("10.5"), true, "10.5 should match the decimal expression");
/// // matches entire expression
/// assert_eq!(decimal_regex.match_perfect("3.9e15"), true, "3.9e15 should match the decimal expression");
/// assert_eq!(decimal_regex.match_perfect("-3.9E15"), true, "-3.9E15 should match the decimal expression");
/// // no matches
/// assert_eq!(decimal_regex.match_perfect("one"), false, "one should NOT match the decimal expression");
/// // matches "1"
/// assert_eq!(decimal_regex.match_perfect("one1"), false, "one1 should NOT match the decimal expression");
/// assert_eq!(decimal_regex.match_perfect("1,2345"), false, "1,2345 should NOT match the decimal expression");
/// ```
pub static DECIMAL_PATTERN: &str = r"-?(([0-9]+\.[0-9]*)|([0-9]+)|(\.[0-9]+))([eE][-+]?[0-9]+)?";

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
    use regex::Regex;
    static ONE_NUMBER: &str = "1";
    static SPACE_NEWLINE_SPACE: &str = " \n ";
    static HELLO: &str = "hello";
    static HELLO_TITLE_CASE: &str = "Hello";
    static ONE_AND_TWO_NUMBER: &str = "1and2";
    static ONE_AND_TWO_TEXT: &str = "oneandtwo";
    static NEGATIVE_DECIMAL: &str = "-1.43";
    static INT_NUMBER: &str = "235";
    static ZERO_PADDED_INT: &str = "034";
    static MILLION_COMMAS: &str = "1,234,235";
    static COMMA_TOO_MANY_NUMS: &str = "1,2345";

    use super::*;
    macro_rules! assert_matches {
        ($pat:expr, $haystack:expr, $tf:expr) => {
            assert_eq!($pat.match_perfect($haystack), $tf);
        };
    }
    #[test]
    fn test_whitespace_pattern() {
        let whitespace_regex = Regex::new(WHITESPACE_PATTERN).expect("Valid regex pattern");
        assert_matches!(whitespace_regex, ONE_NUMBER, false);
        assert_matches!(whitespace_regex, SPACE_NEWLINE_SPACE, true);
        assert_matches!(whitespace_regex, HELLO, false);
        assert_matches!(whitespace_regex, HELLO_TITLE_CASE, false);
        assert_matches!(whitespace_regex, ONE_AND_TWO_NUMBER, false);
        assert_matches!(whitespace_regex, ONE_AND_TWO_TEXT, false);
        assert_matches!(whitespace_regex, NEGATIVE_DECIMAL, false);
        assert_matches!(whitespace_regex, INT_NUMBER, false);
        assert_matches!(whitespace_regex, ZERO_PADDED_INT, false);
        assert_matches!(whitespace_regex, MILLION_COMMAS, false);
        assert_matches!(whitespace_regex, COMMA_TOO_MANY_NUMS, false);
    }

    #[test]
    fn test_alphabetic_pattern() {
        let alphabetic_regex = Regex::new(ALPHABETIC_PATTERN).expect("A valid regex pattern");
        assert_matches!(alphabetic_regex, ONE_NUMBER, false);
        assert_matches!(alphabetic_regex, SPACE_NEWLINE_SPACE, false);
        assert_matches!(alphabetic_regex, HELLO, true);
        assert_matches!(alphabetic_regex, HELLO_TITLE_CASE, true);
        assert_matches!(alphabetic_regex, ONE_AND_TWO_NUMBER, false);
        assert_matches!(alphabetic_regex, ONE_AND_TWO_TEXT, true);
        assert_matches!(alphabetic_regex, NEGATIVE_DECIMAL, false);
        assert_matches!(alphabetic_regex, INT_NUMBER, false);
        assert_matches!(alphabetic_regex, ZERO_PADDED_INT, false);
        assert_matches!(alphabetic_regex, MILLION_COMMAS, false);
        assert_matches!(alphabetic_regex, COMMA_TOO_MANY_NUMS, false);
    }

    #[test]
    fn test_uppercase_pattern() {
        let uppercase_regex = Regex::new(UPPERCASE_PATTERN).expect("A valid regex pattern");
        assert_matches!(uppercase_regex, ONE_NUMBER, false);
        assert_matches!(uppercase_regex, SPACE_NEWLINE_SPACE, false);
        assert_matches!(uppercase_regex, HELLO, false);
        assert_matches!(uppercase_regex, HELLO_TITLE_CASE, false);
        assert_matches!(uppercase_regex, ONE_AND_TWO_NUMBER, false);
        assert_matches!(uppercase_regex, ONE_AND_TWO_TEXT, false);
        assert_matches!(uppercase_regex, NEGATIVE_DECIMAL, false);
        assert_matches!(uppercase_regex, INT_NUMBER, false);
        assert_matches!(uppercase_regex, ZERO_PADDED_INT, false);
        assert_matches!(uppercase_regex, MILLION_COMMAS, false);
        assert_matches!(uppercase_regex, COMMA_TOO_MANY_NUMS, false);
    }

    #[test]
    fn test_lowercase_pattern() {
        let lowercase_regex = Regex::new(LOWERCASE_PATTERN).expect("A valid regex pattern");
        assert_matches!(lowercase_regex, ONE_NUMBER, false);
        assert_matches!(lowercase_regex, SPACE_NEWLINE_SPACE, false);
        assert_matches!(lowercase_regex, HELLO, true);
        assert_matches!(lowercase_regex, HELLO_TITLE_CASE, false);
        assert_matches!(lowercase_regex, ONE_AND_TWO_NUMBER, false);
        assert_matches!(lowercase_regex, ONE_AND_TWO_TEXT, true);
        assert_matches!(lowercase_regex, NEGATIVE_DECIMAL, false);
        assert_matches!(lowercase_regex, INT_NUMBER, false);
        assert_matches!(lowercase_regex, ZERO_PADDED_INT, false);
        assert_matches!(lowercase_regex, MILLION_COMMAS, false);
        assert_matches!(lowercase_regex, COMMA_TOO_MANY_NUMS, false);
    }

    #[test]
    fn test_alphanumeric_pattern() {
        let alphanumeric_regex = Regex::new(ALPHANUMERIC_PATTERN).expect("A valid regex pattern");
        assert_matches!(alphanumeric_regex, ONE_NUMBER, true);
        assert_matches!(alphanumeric_regex, SPACE_NEWLINE_SPACE, false);
        assert_matches!(alphanumeric_regex, HELLO, true);
        assert_matches!(alphanumeric_regex, HELLO_TITLE_CASE, true);
        assert_matches!(alphanumeric_regex, ONE_AND_TWO_NUMBER, true);
        assert_matches!(alphanumeric_regex, ONE_AND_TWO_TEXT, true);
        assert_matches!(alphanumeric_regex, NEGATIVE_DECIMAL, false);
        assert_matches!(alphanumeric_regex, INT_NUMBER, true);
        assert_matches!(alphanumeric_regex, ZERO_PADDED_INT, true);
        assert_matches!(alphanumeric_regex, MILLION_COMMAS, false);
        assert_matches!(alphanumeric_regex, COMMA_TOO_MANY_NUMS, false);
    }

    #[test]
    fn test_identifier_pattern() {
        let identifier_regex = Regex::new(IDENTIFIER_PATTERN).expect("A valid regex pattern");
        assert_matches!(identifier_regex, ONE_NUMBER, false);
        assert_matches!(identifier_regex, SPACE_NEWLINE_SPACE, false);
        assert_matches!(identifier_regex, HELLO, true);
        assert_matches!(identifier_regex, HELLO_TITLE_CASE, true);
        assert_matches!(identifier_regex, ONE_AND_TWO_NUMBER, false);
        assert_matches!(identifier_regex, ONE_AND_TWO_TEXT, true);
        assert_matches!(identifier_regex, NEGATIVE_DECIMAL, false);
        assert_matches!(identifier_regex, INT_NUMBER, false);
        assert_matches!(identifier_regex, ZERO_PADDED_INT, false);
        assert_matches!(identifier_regex, MILLION_COMMAS, false);
        assert_matches!(identifier_regex, COMMA_TOO_MANY_NUMS, false);
    }

    #[test]
    fn test_digit_pattern() {
        let digit_regex = Regex::new(DIGIT_PATTERN).expect("A valid regex pattern");
        assert_matches!(digit_regex, ONE_NUMBER, true);
        assert_matches!(digit_regex, SPACE_NEWLINE_SPACE, false);
        assert_matches!(digit_regex, HELLO, false);
        assert_matches!(digit_regex, HELLO_TITLE_CASE, false);
        assert_matches!(digit_regex, ONE_AND_TWO_NUMBER, false);
        assert_matches!(digit_regex, ONE_AND_TWO_TEXT, false);
        assert_matches!(digit_regex, NEGATIVE_DECIMAL, false);
        assert_matches!(digit_regex, INT_NUMBER, true);
        assert_matches!(digit_regex, ZERO_PADDED_INT, true);
        assert_matches!(digit_regex, MILLION_COMMAS, false);
        assert_matches!(digit_regex, COMMA_TOO_MANY_NUMS, false);
    }

    #[test]
    fn test_decimal_pattern() {
        let decimal_regex = Regex::new(DECIMAL_PATTERN).expect("A valid regex pattern");
        assert_matches!(decimal_regex, ONE_NUMBER, true);
        assert_matches!(decimal_regex, SPACE_NEWLINE_SPACE, false);
        assert_matches!(decimal_regex, HELLO, false);
        assert_matches!(decimal_regex, HELLO_TITLE_CASE, false);
        assert_matches!(decimal_regex, ONE_AND_TWO_NUMBER, false);
        assert_matches!(decimal_regex, ONE_AND_TWO_TEXT, false);
        assert_matches!(decimal_regex, NEGATIVE_DECIMAL, true);
        assert_matches!(decimal_regex, INT_NUMBER, true);
        assert_matches!(decimal_regex, ZERO_PADDED_INT, true);
        assert_matches!(decimal_regex, MILLION_COMMAS, false);
        assert_matches!(decimal_regex, COMMA_TOO_MANY_NUMS, false);
    }

    #[test]
    fn test_comma_pattern() {
        let comma_number_regex = Regex::new(COMMA_NUMBER_PATTERN).expect("A valid regex pattern");
        assert_matches!(comma_number_regex, ONE_NUMBER, false);
        assert_matches!(comma_number_regex, SPACE_NEWLINE_SPACE, false);
        assert_matches!(comma_number_regex, HELLO, false);
        assert_matches!(comma_number_regex, HELLO_TITLE_CASE, false);
        assert_matches!(comma_number_regex, ONE_AND_TWO_NUMBER, false);
        assert_matches!(comma_number_regex, ONE_AND_TWO_TEXT, false);
        assert_matches!(comma_number_regex, NEGATIVE_DECIMAL, false);
        assert_matches!(comma_number_regex, INT_NUMBER, false);
        assert_matches!(comma_number_regex, ZERO_PADDED_INT, false);
        assert_matches!(comma_number_regex, MILLION_COMMAS, true);
        assert_matches!(comma_number_regex, COMMA_TOO_MANY_NUMS, false);
    }
}
