
// [bit;4]
#[derive(Debug, PartialEq)]
pub enum SoundFormat {
    LinearPCMPE, // Linear PCM, platform endian
    ADPCM,
    MP3,
    LinearPCMLE, // Linear PCM, little endian
    Nellymoser16KHZMono,
    Nellymoser8KHZMono,
    Nellymoser,
    AAC,
    Speex,
    Reserved,
    InternalUse,
    Invalid,
}

// [bit;2]
#[derive(Debug, PartialEq)]
pub enum SoundRate {
    _5_5KHZ, // 5.5khz
    _11KHZ,  // 11khz
    _22KHZ,  // 22khz
    _44KHZ,  // 44khz, aac always this
}

// [bit;1]
#[derive(Debug, PartialEq)]
pub enum SoundSize {
    _8Bit,
    _16Bit,
}

// [bit;1]
#[derive(Debug, PartialEq)]
pub enum SoundType {
    Mono,
    Stereo,
}

// The SoundFormat, SoundRate, SoundSize, SoundType is a u8
pub fn audio_header(b: u8) -> (SoundFormat, SoundRate, SoundSize, SoundType) {
    let format = (b >> 4) & 0b1111;
    let format = match format {
        0 => SoundFormat::LinearPCMPE,
        1 => SoundFormat::ADPCM,
        2 => SoundFormat::MP3,
        3 => SoundFormat::LinearPCMLE,
        4 => SoundFormat::Nellymoser16KHZMono,
        5 => SoundFormat::Nellymoser8KHZMono,
        6 => SoundFormat::Nellymoser,
        10 => SoundFormat::AAC,
        11 => SoundFormat::Speex,
        7 | 8 | 14 | 15 => SoundFormat::InternalUse,
        9 => SoundFormat::Reserved,
        _ => SoundFormat::Invalid,
    };
    let rate = (b >> 2) & 0b11;
    let rate = match rate {
        0 => SoundRate::_5_5KHZ,
        1 => SoundRate::_11KHZ,
        2 => SoundRate::_22KHZ,
        3 => SoundRate::_44KHZ,
        _ => SoundRate::_44KHZ,
    };
    let size = (b >> 1) & 0b1;
    let size = match size {
        0 => SoundSize::_8Bit,
        1 => SoundSize::_16Bit,
        _ => SoundSize::_8Bit,
    };
    let sound_type = b & 0b1;
    let sound_type = match sound_type {
        0 => SoundType::Mono,
        1 => SoundType::Stereo,
        _ => SoundType::Mono,
    };

    (format, rate, size, sound_type)
}

#[derive(Debug, PartialEq)]
pub struct AudioData {
    pub sound_format: SoundFormat,
    pub sound_rate: SoundRate,
    pub sound_size: SoundSize,
    pub sound_type: SoundType,
    // if sound_format == 10, AACAudioData
    // else varies by format
    pub sound_data: Vec<u8>,
}

pub fn audio_data(input: &[u8], size: usize) -> Result<AudioData, String> {
    let (sound_format, sound_rate, sound_size, sound_type) =
        audio_header(input[0]);

    Ok(AudioData{
        sound_format,
        sound_rate,
        sound_size,
        sound_type,
        sound_data: Vec::from(&input[1..size]),
    })
}

pub struct AACAudioData {
    // 0: AAC sequence header
    // 1: AAC raw
    pub aac_packet_type: u8,
    // if type == 0, AudioSpecificConfig
    // else 1: AAC frame data
    // The AudioSpecificConfig is explained in ISO 14496-3
    pub data: Vec<u8>,
}

pub fn aac_audio_packet(input: &[u8], size: usize) -> Result<AACAudioData, String> {
    if input.len() < size {
        return Err("aac audio packet need more size".to_string());
    }
    let aac_packet_type = input[0];
    Ok(AACAudioData{
        aac_packet_type,
        data: Vec::from(&input[1..size]),
    })
}
