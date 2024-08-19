//! Voices and their representations.

use serde::{Serialize, Deserialize};

/// This is the header (the first few bytes of a file) that must be present for a file to be
/// identified as an Flite VoxData file.
const FLITE_FILE_HEADER: &str = "CMU_FLITE_CG_VOXDATA-v2.0";

/// Representation of all supported languages
#[derive(Serialize, Deserialize)]
pub enum Language {
    /// [Bengali](https://en.wikipedia.org/wiki/Bengali_language)
    #[serde(rename = "ben")]
    Bengali,
    /// [English](https://en.wikipedia.org/wiki/English_language)
    #[serde(rename = "eng")]
    English,
    /// [Hindi](https://en.wikipedia.org/wiki/Hindi)
    #[serde(rename = "hin")]
    Hindi,
    /// [Tamil](https://en.wikipedia.org/wiki/Tamil_language)
    #[serde(rename = "tam")]
    Tamil,
    /// [Telugu](https://en.wikipedia.org/wiki/Telugu_language)
    #[serde(rename = "tel")]
    Telugu,
}

/*
pub struct ClusterGenDb {
    name: String,
    types: Vec<String>,
    sample_rate: isize,
    f0_mean: f32,
    f0_stddev: f32,

}
*/

#[test]
fn test_read_file() {
    let f = include_bytes!("../../../flite/voices/cmu_us_aew.flitevox");
    let first_bytes = &f[0..2048];
    if first_bytes.starts_with(FLITE_FILE_HEADER.as_bytes()) {
        std::println!("{:?}", first_bytes);
    }
}
