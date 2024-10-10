/// Ordinal number pattern.
///
/// NOTE: this will match invalid patterns like "19st" and "12nd", the test is currently ignored
///
/// ```rust,ignore
/// use fry_en_us::regex::ORDINAL_NUMBER_PATTERN;
/// use fry_common::regex::RegexExt;
/// use regex::Regex;
/// let ordinal_regex = Regex::new(ORDINAL_NUMBER_PATTERN).expect("Valid regex pattern");
/// assert_eq!(ordinal_regex.match_perfect("20th"), true, "20th should match the pattern");
/// assert_eq!(ordinal_regex.match_perfect("12nd"), false, "12nd shoud not motch the pattern");
/// assert_eq!(ordinal_regex.match_perfect("19st"), false, "19st should not match the pattern");
/// assert_eq!(ordinal_regex.match_perfect("19ST"), false, "19ST should not match the pattern");
/// ```
pub static ORDINAL_NUMBER_PATTERN: &str = r"[0-9][0-9,]*(th|TH|st|ST|nd|ND|rd|RD)";

/// Matches any vowel (with any case).
///
/// NOTE: This pattern will match an _entire string_ if a single vowel is contained in it
///
/// ```rust
/// use fry_en_us::regex::HAS_VOWEL_PATTERN;
/// use fry_common::regex::RegexExt;
/// use regex::Regex;
/// let has_vowel_regex = Regex::new(HAS_VOWEL_PATTERN).expect("Valid regex pattern");
/// assert_eq!(has_vowel_regex.match_perfect("ABC"), true);
/// assert_eq!(has_vowel_regex.match_perfect("BCD"), false);
/// assert_eq!(has_vowel_regex.match_perfect("123"), false);
/// assert_eq!(has_vowel_regex.match_perfect("Good"), true);
/// assert_eq!(has_vowel_regex.match_perfect("open"), true);
/// assert_eq!(has_vowel_regex.match_perfect("ctl"), false);
/// ```
pub static HAS_VOWEL_PATTERN: &str = ".*[aeiouAEIOU].*";

/// A pattern which matches the format of money in the US.
/// This will not match the European style of money: with the commas and periods swapped.
/// Neither will it match any other currency sign, only the dollar sign will be recognized.
///
/// ```rust
/// use fry_en_us::regex::US_MONEY_PATTERN;
/// use fry_common::regex::RegexExt;
/// use regex::Regex;
/// let us_money_regex = Regex::new(US_MONEY_PATTERN).expect("Valid regex pattern");
/// assert_eq!(us_money_regex.match_perfect("$10"), true);
/// assert_eq!(us_money_regex.match_perfect("$10.10"), true);
/// assert_eq!(us_money_regex.match_perfect("$1,998.99"), true);
/// assert_eq!(us_money_regex.match_perfect("$1,998"), true);
/// assert_eq!(us_money_regex.match_perfect("£1,998"), false);
/// // only the dollar sign notation is matched
/// assert_eq!(us_money_regex.match_perfect("two dollars"), false);
/// ```
pub static US_MONEY_PATTERN: &str = r"\$[0-9,]+(\.[0-9]+)?";

/// Mathes a word that ends in -"illion".
/// Lower case only.
///
/// ```rust
/// use fry_en_us::regex::ILLION_PATTERN;
/// use fry_common::regex::RegexExt;
/// use regex::Regex;
/// let illion_regex = Regex::new(ILLION_PATTERN).expect("Valid regex pattern");
/// assert_eq!(illion_regex.match_perfect("Billion"), true);
/// assert_eq!(illion_regex.match_perfect("billion"), true);
/// assert_eq!(illion_regex.match_perfect("septillion"), true);
/// // lowercase match only
/// assert_eq!(illion_regex.match_perfect("BILLION"), false);
/// // fake words are ok
/// assert_eq!(illion_regex.match_perfect("kajillion"), true);
/// ```
pub static ILLION_PATTERN: &str = ".*illion";

/// Matches Roman numerals.
/// Uppercase only.
///
/// ```rust,ignore
/// use fry_en_us::regex::ROMAN_NUMS_PATTERN;
/// use fry_common::regex::RegexExt;
/// use regex::Regex;
/// let roman_nums_regex = Regex::new(ROMAN_NUMS_PATTERN).expect("Valid regex pattern");
/// assert_eq!(roman_nums_regex.match_perfect("III"), true);
/// assert_eq!(roman_nums_regex.match_perfect("IV"), true);
/// assert_eq!(roman_nums_regex.match_perfect("IX"), true);
/// assert_eq!(roman_nums_regex.match_perfect("LVIII"), true);
/// assert_eq!(roman_nums_regex.match_perfect("XIV"), true);
/// assert_eq!(roman_nums_regex.match_perfect("XV"), true);
/// assert_eq!(roman_nums_regex.match_perfect("L"), true);
/// assert_eq!(roman_nums_regex.match_perfect("XL"), true);
/// assert_eq!(roman_nums_regex.match_perfect("LX"), true);
/// // the year I wrote this code!
/// assert_eq!(roman_nums_regex.match_perfect("MMXXIV"), true);
/// assert_eq!(roman_nums_regex.match_perfect("CM"), true, "CM?");
/// // TODO: this should technically fail: IV is V minus one, VI is V plus one...
/// assert_eq!(roman_nums_regex.match_perfect("IVI"), true);
/// ```
pub static ROMAN_NUMS_PATTERN: &str =
    r"(CM|[M]{0,3})(CD|[D]{0,3})(XC|[C]{0,3})(XL|[L]{0,3})(IX|[X]{0,3})(IV|[V]{0,3})[I]{0,3}";
// ORIGINAL, completely defective: pub static ROMAN_NUMS_PATTERN: &str = r"(II?I?|IV|VI?I?I?|IX|X[VIX]*)";

/// "Dr and "St", case insensitive.
/// No idea why this exists.
///
/// ```rust
/// use fry_en_us::regex::DR_ST_PATTERN;
/// use fry_common::regex::RegexExt;
/// use regex::Regex;
/// let dr_st_regex = Regex::new(DR_ST_PATTERN).expect("Valid regex pattern");
/// assert_eq!(dr_st_regex.match_perfect("dr"), true);
/// assert_eq!(dr_st_regex.match_perfect("Dr"), true);
/// assert_eq!(dr_st_regex.match_perfect("St"), true);
/// assert_eq!(dr_st_regex.match_perfect("ST"), true);
/// ```
pub static DR_ST_PATTERN: &str = r"([dD][Rr]|[Ss][tT])";

/// Plural numbers, think "100s of rolls of toilet paper!"
///
/// ```rust
/// use fry_en_us::regex::PLURAL_NUMBER_PATTERN;
/// use fry_common::regex::RegexExt;
/// use regex::Regex;
/// let plural_number_regex = Regex::new(PLURAL_NUMBER_PATTERN).expect("Valid regex pattern");
/// assert_eq!(plural_number_regex.match_perfect("100s"), true);
/// // pronounced "dozens"
/// assert_eq!(plural_number_regex.match_perfect("12s"), true);
/// assert_eq!(plural_number_regex.match_perfect("10s"), true);
/// ```
pub static PLURAL_NUMBER_PATTERN: &str = "[0-9]+s";

/// Numbers with a single leading zero.
///
/// ```rust
/// use fry_en_us::regex::LEADING_ZERO_DIGITS_PATTERN;
/// use fry_common::regex::RegexExt;
/// use regex::Regex;
/// let leading_zero_digits_regex = Regex::new(LEADING_ZERO_DIGITS_PATTERN).expect("Valid regex pattern");
/// assert_eq!(leading_zero_digits_regex.match_perfect("09"), true);
/// assert_eq!(leading_zero_digits_regex.match_perfect("010"), true);
/// assert_eq!(leading_zero_digits_regex.match_perfect("099"), true);
/// // must have at least one other number
/// assert_eq!(leading_zero_digits_regex.match_perfect("0"), false);
/// ```
pub static LEADING_ZERO_DIGITS_PATTERN: &str = "0[0-9]+";

/// A seven digit phone number.
/// This is common in North America when listing a local number (without an area code).
///
/// ```rust
/// use fry_en_us::regex::SEVEN_DIGIT_PHONE_NUMBER_PATTERN;
/// use fry_common::regex::RegexExt;
/// use regex::Regex;
/// let short_phone_regex = Regex::new(SEVEN_DIGIT_PHONE_NUMBER_PATTERN).expect("Valid regex pattern");
/// assert_eq!(short_phone_regex.match_perfect("555-5555"), true);
/// assert_eq!(short_phone_regex.match_perfect("009-1234"), true);
/// // no area code allowed
/// assert_eq!(short_phone_regex.match_perfect("403-888-8888"), false);
/// ```
pub static SEVEN_DIGIT_PHONE_NUMBER_PATTERN: &str = "[0-9][0-9][0-9]-[0-9][0-9][0-9][0-9]";

/// Four digit pattern. Matches four numbers in a row.
///
/// I have not figured out why this exists.
///
/// ```rust
/// use fry_en_us::regex::FOUR_DIGITS_PATTERN;
/// use fry_common::regex::RegexExt;
/// use regex::Regex;
/// let four_digits_regex = Regex::new(FOUR_DIGITS_PATTERN).expect("Valid regex pattern");
/// assert_eq!(four_digits_regex.match_perfect("1234"), true);
/// assert_eq!(four_digits_regex.match_perfect("123456"), false);
/// assert_eq!(four_digits_regex.match_perfect("999"), false);
/// assert_eq!(four_digits_regex.match_perfect("8766"), true);
/// ```
pub static FOUR_DIGITS_PATTERN: &str = "[0-9][0-9][0-9][0-9]";

/// Three digit pattern. Matches three numbers in a row.
///
/// I have not figured out why this exists.
///
/// ```rust
/// use fry_en_us::regex::THREE_DIGITS_PATTERN;
/// use fry_common::regex::RegexExt;
/// use regex::Regex;
/// let three_digits_regex = Regex::new(THREE_DIGITS_PATTERN).expect("Valid regex pattern");
/// assert_eq!(three_digits_regex.match_perfect("123"), true);
/// assert_eq!(three_digits_regex.match_perfect("12356"), false);
/// assert_eq!(three_digits_regex.match_perfect("999"), true);
/// assert_eq!(three_digits_regex.match_perfect("87"), false);
/// ```
pub static THREE_DIGITS_PATTERN: &str = "[0-9][0-9][0-9]";

/// Matches the time (with or without a leading zero).
/// Supports both 12 and 24 hour formats.
///
/// ```rust
/// use fry_en_us::regex::TIME_PATTERN;
/// use fry_common::regex::RegexExt;
/// use regex::Regex;
/// let time_regex = Regex::new(TIME_PATTERN).expect("Valid regex pattern");
/// assert_eq!(time_regex.match_perfect("10:22"), true);
/// assert_eq!(time_regex.match_perfect("2:18"), true);
/// assert_eq!(time_regex.match_perfect("02:18"), true);
/// assert_eq!(time_regex.match_perfect("18:18"), true);
/// assert_eq!(time_regex.match_perfect("18:18"), true);
/// assert_eq!(time_regex.match_perfect("11:00"), true);
/// assert_eq!(time_regex.match_perfect("11:59"), true);
/// // anything over 59 is not allowed for the minute portion
/// assert_eq!(time_regex.match_perfect("11:60"), false);
/// assert_eq!(time_regex.match_perfect("11:99"), false);
/// // invalid hours are permitted
/// assert_eq!(time_regex.match_perfect("25:99"), false);
/// ```
pub static TIME_PATTERN: &str = "[0-9]?[0-9]:[0-5][0-9]";

/// Time pattern with the am or pm suffix.
/// Lowercase only.
///
/// ```rust
/// use fry_en_us::regex::TIME_AMPM_PATTERN;
/// use fry_common::regex::RegexExt;
/// use regex::Regex;
/// let time_regex = Regex::new(TIME_AMPM_PATTERN).expect("Valid regex pattern");
/// assert_eq!(time_regex.match_perfect("10:22am"), true);
/// assert_eq!(time_regex.match_perfect("2:18pm"), true);
/// assert_eq!(time_regex.match_perfect("02:18am"), true);
/// assert_eq!(time_regex.match_perfect("18:18pm"), true);
/// assert_eq!(time_regex.match_perfect("18:18am"), true);
/// assert_eq!(time_regex.match_perfect("11:00am"), true);
/// assert_eq!(time_regex.match_perfect("11:59pm"), true);
/// // anything over 59 is not allowed for the minute portion
/// assert_eq!(time_regex.match_perfect("11:60pm"), false);
/// assert_eq!(time_regex.match_perfect("11:99pm"), false);
/// // invalid hours are permitted
/// assert_eq!(time_regex.match_perfect("25:99pm"), false);
/// ```
pub static TIME_AMPM_PATTERN: &str = r"[0-9]?[0-9][:\.][0-5][0-9][ap]m";

/// Matches abbreviations seperated by periods.
///
/// ```rust
/// use fry_en_us::regex::DOTTED_ABBREVIATIONS_PATTERN;
/// use fry_common::regex::RegexExt;
/// use regex::Regex;
/// let abbr_regex = Regex::new(DOTTED_ABBREVIATIONS_PATTERN).expect("Valid regex pattern");
/// assert_eq!(abbr_regex.match_perfect("A.M."), true);
/// assert_eq!(abbr_regex.match_perfect("C.I.A."), true);
/// // There must be a period between each character
/// assert_eq!(abbr_regex.match_perfect("Ph.D."), false);
/// // Band of Canada
/// assert_eq!(abbr_regex.match_perfect("B.o.C."), true);
/// ```
pub static DOTTED_ABBREVIATIONS_PATTERN: &str = r"([A-Za-z]\.)+[A-Za-z]\.?";

/// Plain-text fraction pattern. Think "5/8" or "1/2".
///
/// ```rust
/// use fry_en_us::regex::DIGITS_SLASH_DIGITS_PATTERN;
/// use fry_common::regex::RegexExt;
/// use regex::Regex;
/// let frac_regex = Regex::new(DIGITS_SLASH_DIGITS_PATTERN).expect("Valid regex pattern");
/// assert_eq!(frac_regex.match_perfect("5/8"), true);
/// assert_eq!(frac_regex.match_perfect("1/2"), true);
/// assert_eq!(frac_regex.match_perfect("11/13"), true);
/// // only matches the first three characters
/// assert_eq!(frac_regex.match_perfect("5/8ths"), false);
/// ```
pub static DIGITS_SLASH_DIGITS_PATTERN: &str = "[0-9]+/[0-9]+";

/// Matches digits separated by a dash.
///
/// ```rust
/// use fry_en_us::regex::DIGITS_TO_DASH_PATTERN;
/// use fry_common::regex::RegexExt;
/// use regex::Regex;
/// let d2d_regex = Regex::new(DIGITS_TO_DASH_PATTERN).expect("Valid regex pattern");
/// // tennis score
/// assert_eq!(d2d_regex.match_perfect("20-0"), true);
/// // sports score
/// assert_eq!(d2d_regex.match_perfect("12-8"), true);
/// // wins-losses-overtime losses hockey stat
/// assert_eq!(d2d_regex.match_perfect("12-8-9"), true);
/// // double+ dashes are permitted
/// assert_eq!(d2d_regex.match_perfect("99--99"), true);
/// assert_eq!(d2d_regex.match_perfect("99---99"), true);
/// ```
pub static DIGITS_TO_DASH_PATTERN: &str = r"([0-9]+-+)+[0-9]+";

/// Matches common unit patterns like "4GHz", "16lbs", or "128GB".
/// No spaces are allowed between the number and the unit.
///
/// NOTE: this is slightly modified from the original so that for example "MHz" would match before
/// "Hz".
///
/// ```rust
/// use fry_en_us::regex::UNIT_PATTERN;
/// use fry_common::regex::RegexExt;
/// use regex::Regex;
/// let unit_regex = Regex::new(UNIT_PATTERN).expect("Valid regex pattern");
/// assert_eq!(unit_regex.match_perfect("3GHz"), true);
/// assert_eq!(unit_regex.match_perfect("220lbs"), true);
/// assert_eq!(unit_regex.match_perfect("100TB"), true);
/// assert_eq!(unit_regex.match_perfect("6ft"), true);
/// ```
pub static UNIT_PATTERN: &str =
    r"[0-9,]*[0-9]+(lbs|lb|LBS|LB|ft|FT|kg|km|oz|hz|KHz|MHz|GHz|Hz|HZ|cm|mm|ml|KB|MB|GB|TB)";
