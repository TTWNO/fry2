//! Expand text like "$3.14" to "three dollars and fourteen cents".
//!
//! Contains expansions for:
//! - Roman numerals
//! - numbers
//! - ordinal numbers
//! - real numbers
//!

use alloc::{
    string::String,
    vec::Vec,
};

use num2words::Num2Words;

static DIGIT_TO_NUM: [&str; 10] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
];

static DIGIT_TO_TEEN: [&str; 10] = [
    "ten", /* not used, but there to make indexing easier */
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "ninteen",
];

static DIGIT_TO_TENS: [&str; 10] = [
    "zero", /* not used, but there to make indexing easier */
    "ten",
    "twenty",
    "thirty",
    "fourty",
    "fifty",
    "sixty",
    "seventy",
    "eighty",
    "ninety",
];

static DIGIT_TO_ORDINAL: [&str; 10] = [
    "zeroth",
    "first",
    "second",
    "third",
    "fourth",
    "fifth",
    "sixth",
    "seventh",
    "eighth",
    "ninth",
];

static DIGIT_TO_ORDINAL_TEEN: [&str; 10] = [
    "tenth", /* not used, but there to make indexing easier */
    "eleventh",
    "twelveth",
    "thirteenth",
    "fourteenth",
    "fifteenth",
    "sixteenth",
    "seventeenth",
    "eighteenth",
    "ninteenth",
];

static DIGIT_TO_ORDINAL_TENS: [&str; 10] = [
    "zeroth", /* not used, but there to make indexing easier */
    "tenth",
    "twentieth",
    "thirtieth",
    "fourtieth",
    "fiftieth",
    "sixtieth",
    "seventieth",
    "eightieth",
    "ninetieth",
];

/// Read a number as an ordinal.
/// Expand to a list of words.
///
/// Returns None if the format of the number is invalid
/// ```rust
/// # use fry_en_us::expand::expand_ordinal;
/// assert_eq!(expand_ordinal("2,000"), Some("two thousandth".to_string()));
/// assert_eq!(expand_ordinal("832"), Some("eight hundred thirty-second".to_string()));
/// assert_eq!(expand_ordinal("33,093"), Some("thirty-three thousand and ninety-third".to_string()));
/// assert_eq!(expand_ordinal("10"), Some("tenth".to_string()));
/// // NOTE: you can not use a real numebr with ordinality
/// assert_eq!(expand_ordinal("10.8"), None);
/// ```
// TODO: optimize
pub fn expand_ordinal<'a>(s: &'a str) -> Option<String> {
    let only_num_chrs = s.chars()
        .filter(|c| *c >= '0' && *c <= '9' || *c == '.')
        .collect::<String>();
    Num2Words::parse(&only_num_chrs)?
        .ordinal()
        .to_words()
        .ok()
}

/// Read a number.
/// Expand to a list of words.
///
/// Returns None if the format of the number is invalid
/// ```rust
/// # use fry_en_us::expand::expand_number;
/// assert_eq!(expand_number("2,000"), Some("two thousand".to_string()));
/// assert_eq!(expand_number("832"), Some("eight hundred thirty-two".to_string()));
/// // NOTE: not sure if this is exactly how flite/festival does it, but it's good enought
/// assert_eq!(expand_number("33,093.88"), Some("thirty-three thousand and ninety-three point eight eight".to_string()));
/// ```
// TODO: optimize
pub fn expand_number<'a>(s: &'a str) -> Option<String> {
    let only_num_chrs = s.chars()
        .filter(|c| *c >= '0' && *c <= '9' || *c == '.')
        .collect::<String>();
    Num2Words::parse(&only_num_chrs)?
        .to_words()
        .ok()
}

/// Read a number from its real/scientific form.
/// Ignores all characters other than "0-9", "+-", "eE" and "."
/// ```rust
/// # use fry_en_us::expand::expand_real;
/// assert_eq!(expand_real("10.10"), Some("ten point one".to_string()));
/// assert_eq!(expand_real("6.022e-8"), Some("point zero zero zero zero zero zero zero six zero two two".to_string()));
/// assert_eq!(expand_real("2.03e3"), Some("two thousand and thirty".to_string()));
/// // NOTE: should this be "negative" instead of "minus"?
/// assert_eq!(expand_real("-3.09E6"), Some("minus three million ninety thousand".to_string()));
/// assert_eq!(expand_real("+8.09e-2"), Some("point zero eight zero nine".to_string()));
/// ```
pub fn expand_real<'a>(s: &'a str) -> Option<String> {
    let only_num_chrs = s.chars()
        .filter(|c| (*c >= '0' && *c <= '9') || *c == '.' || *c == 'e' || *c == 'E' || *c == '+' || *c == '-')
        .collect::<String>();
    Num2Words::parse(&only_num_chrs)?
        .to_words()
        .ok()
}

/// Read a number into individual digits.
/// Ignores all characters other than those in "012345689".
///
/// ```rust
/// # use fry_en_us::expand::expand_digits;
/// assert_eq!(expand_digits("123"), "one two three");
/// assert_eq!(expand_digits("894"), "eight nine four");
/// assert_eq!(expand_digits("670"), "six seven zero");
/// assert_eq!(expand_digits("aoeusnthaoeu8baoeuntaoeusnthaoeu"), "eight");
/// assert_eq!(expand_digits("beep5boop2"), "five two");
/// // test the ASCII characters on either side of 0 and 9
/// assert_eq!(expand_digits("./0:;9"), "zero nine");
/// ```
pub fn expand_digits<'a>(s: &'a str) -> String {
    s.chars()
        .filter_map(|c| if c >= '0' && c <= '9' {
            Some(DIGIT_TO_NUM[c as usize - '0' as usize])
        } else {
            None
        })
        .collect::<Vec<&str>>()
        .join(" ")
}

/// Read roman numerals into a `usize`.
/// Ignores all characters other than those in "IVXLCDM".
///
/// NOTE: this function can only take uppercase roman numerals
///
/// ```rust
/// # use fry_en_us::expand::parse_roman;
/// assert_eq!(parse_roman("III"), 3);
/// assert_eq!(parse_roman("IV"), 4);
/// assert_eq!(parse_roman("IX"), 9);
/// assert_eq!(parse_roman("LVIII"), 58);
/// assert_eq!(parse_roman("XIV"), 14);
/// assert_eq!(parse_roman("XV"), 15);
/// assert_eq!(parse_roman("L"), 50);
/// assert_eq!(parse_roman("XL"), 40);
/// assert_eq!(parse_roman("LX"), 60);
/// // the year I wrote this code!
/// assert_eq!(parse_roman("MMXXIV"), 2024);
/// ```
pub fn parse_roman<'a>(s: &'a str) -> usize {
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
