const SOUND_MAGIC: u16 = 0x2e736e64;
/// TODO: this should be an enum
const SOUND_ULAW: u16 = 1
const SOUND_UCHAR: u16 = 2;
const SOUND_SHORT: u16 = 3;

pub struct WaveHeader {
    /// Type
    typ: String,
    hsize: usize,
    num_bytes: usize,
    sample_rate: usize,
    num_samples: usize,
    num_channels: usize,
}

pub struct Wave {
    /// Type
    typ: String,
    sample_rate: usize,
    num_channels: usize,
    samples: Vec<u16>,
}

pub struct SoundHeader {
    magic: u16,
    hdr_size: u16,
    data_size: i16,
    encoding: u16,
    sample_rate: u16,
    channels: u16,
}
