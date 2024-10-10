//! Basic Part of Speech Data
//!
//! Based on the "poors man part of speech tagger" in _Flite_

static POS_PREPOSITION: [&str; 28] = [
    "in", "of", "for", "on", "that", "with", "by", "at", "from", "as", "if", "against", "about",
    "before", "because", "under", "after", "over", "into", "while", "without", "through",
    "between", "among", "until", "per", "up", "down",
];

static POS_COUNT: [&str; 17] = [
    "det", "the", "a", "an", "some", "this", "each", "another", "those", "every", "all", "any",
    "these", "both", "neither", "no", "many",
];

static POS_POSSIBLE: [&str; 9] = [
    "will", "may", "would", "can", "could", "should", "must", "ought", "might",
];

static POS_AUXILLIARY: [&str; 9] = ["is", "am", "are", "was", "were", "has", "have", "had", "be"];

static POS_PUNCTUATION: [&str; 10] = [".", ",", ":", ";", "\"", "'", "(", "?", ")", "!"];
