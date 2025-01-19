use crate::{
    Value, Utterance, Item, Features, LtsRules, feature::Feature,
};
use alloc::{vec::Vec, string::String};

/// Sonority Hirearchy: https://en.wikipedia.org/wiki/Sonority_hierarchy
#[derive(Default)]
#[repr(u8)]
pub enum Sonority {
    /// Default value.
    #[default]
    Other = 1,
    /// Voiced obstruents
    VoicedObstruent = 2,
    /// Nasals
    Nasal = 3,
    /// Glide or liquid
    Glide = 4,
    /// Vowel
    Vowel = 5,
}

/// A str-extension trait to implement the same features used in Flite.
///
/// These are _mostly_ just hacks to make the US English parsing easier.
pub trait StrExt {
    /// Checks if first char is in the list: `aeiou`.
    fn is_vowel(&self) -> bool;
    /// Checks if == "pau"; no idea why.
    fn is_silence(&self) -> bool;
    /// Sonority: see [`Sonority`].
    fn sonority(&self) -> Sonority;
}
impl StrExt for str {
    fn is_vowel(&self) -> bool {
        let Some(c) = self.chars().nth(0) else {
            return false;
        };
        ['a', 'e', 'i', 'o', 'u'].contains(&c)
    }
    fn is_silence(&self) -> bool {
        self == "pau"
    }
    fn sonority(&self) -> Sonority {
        if self.is_vowel() || self.is_silence() {
            return Sonority::Vowel;
        }
        let Some(c) = self.chars().nth(0) else {
            return Sonority::Other;
        };
        match c {
            'w' | 'y' | 'l' | 'r' => Sonority::Glide,
            'n' | 'm' => Sonority::Nasal,
            'b' | 'd' | 'g' | 'j' | 'l' | 'm' | 'n' | 'r' | 'v' | 'w' | 'y' | 'z' => Sonority::VoicedObstruent,
            _ => Sonority::Other,
        }
    }
}

pub trait LexTrait {
    fn syl_boundary<'a>(item: &mut Item, val: &mut Value<'a>) -> usize;
    fn lts_function<'a>(&self, name: &'a str, pos: &'a str, features: Features<'a>) -> Value<'a>;
    fn postlext<'a>(utt: Utterance<'a>) -> Utterance<'a>;
}

pub struct Lexicon<'a> {
    name: String,
    num_entries: usize,
    /// Entries are centered around bytes with value 255 */
    /// entries and forward (compressed) pronunciations and backwards */
    /// each are terminated (preceeded in pron case) by 0 */
    /// This saves 4 bytes per entry for an index */
    data: Vec<u8>,
    phone_table: Vec<String>,
    lts_rules: LtsRules<'a>,
    addenda: Vec<Vec<String>>,
    phone_hufftable: Vec<Vec<String>>,
    entry_hufftable: Vec<Vec<String>>,
    lex_addenda: Value<'a>
}

impl Lexicon<'_> {
    /// Lookup a word in the lexicon.
    ///
    /// NOTE: `pos` is represented as a str here, but it should be something else.
    pub fn lookup(&self, word: &str, pos: &str, features: &[Feature<'_>]) -> String {
        todo!()
    }
    /// Lookup a value in the addenda. Addenda are able to be added dynamically.
    /// The [`Self::lookup`] function internally calls this function.
    pub fn lookup_addenda(&self) -> Option<Value<'_>> {
        todo!()
    }
}

