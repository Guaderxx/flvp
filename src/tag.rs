pub mod audio;
pub mod video;
pub mod amf0;

pub const TAG_HEADER_SIZE: usize = 11;
pub const TAG_TYPE_AUDIO: u8 = 8;
pub const TAG_TYPE_VIDEO: u8 = 9;
pub const TAG_TYPE_SCRIPT: u8 = 18;

// TagHeader: This part has a definite size, so as header
#[derive(Debug, PartialEq, Clone)]
pub struct TagHeader {
    // Type of this tag. Values are:
    // 8: audio
    // 9: video
    // 18: script data
    // all others: reserved
    pub tag_type: u8,
    // Length of the data in the data field,
    pub data_size: u32,
    // Time in milliseconds at which the data in this tag applies.
    // This value is relative to the first tag in the FLV file, which always has a timestamp of 0.
    pub timestamp: u32,
    // Always 0.
    pub stream_id: u32,
}

// FLV tags
#[derive(Debug,PartialEq)]
pub struct Tag {
    pub header: TagHeader,
    pub data: TagData, // Body of the tag
}

#[derive(Debug,PartialEq)]
pub enum TagData {
    Audio(audio::AudioData),
    Video(video::VideoData),
    Script(amf0::AMFObject),
}

pub fn tag_header(input: &[u8]) -> Result<TagHeader, String> {
    let data_size = u32::from_be_bytes([
        0,
        input[1],
        input[2],
        input[3],
    ]);
    let timestamp = u32::from_be_bytes([
        input[7],
        input[4],
        input[5],
        input[6],
    ]);
    let stream_id = u32::from_be_bytes([
        0,
        input[8],
        input[9],
        input[10],
    ]);

    Ok(TagHeader{
        tag_type: input[0],
        data_size,
        timestamp,
        stream_id,
    })
}

pub fn tag(input: &[u8]) -> Result<(Tag, &[u8]), String> {
    let header = tag_header(&input[..TAG_HEADER_SIZE])?;
    let data = match header.tag_type {
        TAG_TYPE_AUDIO => {
            let data = audio::audio_data(&input[TAG_HEADER_SIZE..], header.data_size as usize)?;
            TagData::Audio(data)
        },
        TAG_TYPE_VIDEO => {
            let data = video::video_data(&input[TAG_HEADER_SIZE..], header.data_size as usize)?;
            TagData::Video(data)
        },
        TAG_TYPE_SCRIPT => {
            let (data, _) = amf0::amf_data(&input[TAG_HEADER_SIZE..])?;
            TagData::Script(data)
        },
        _ => {
            // TODO:
            let data = video::video_data(&input[TAG_HEADER_SIZE..], header.data_size as usize)?;
            TagData::Video(data)
        }
    };

    let offset = header.data_size as usize;
    Ok((
        Tag{
            header,
            data,
        },
        &input[offset + TAG_HEADER_SIZE..]
        ))
}
