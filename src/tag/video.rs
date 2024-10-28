use crate::tag::video::VideoDataByFrame::VideoFramePayload;

// [bit;4]
#[derive(Debug, PartialEq)]
pub enum FrameType {
    // 1, for AVC, a seekable frame
    Key,
    // 2, for AVC, a non-seekable frame
    Inter,
    // 3, H.264 only
    DisposableInter,
    // 4, reserved for serve use only
    Generated,
    // 5, video info/command frame
    Video,
}

// [bit;4]
#[derive(Debug, PartialEq)]
pub enum CodecID {
    // 1, currently unused
    JPEG,
    // 2, H263
    Sorenson,
    // 3
    ScreenVideo,
    // 4, On2 VP6
    VP6,
    // 5, On2 VP6 with alpha channel
    VP6A,
    // 6, Screen video version 2
    ScreenVideo2,
    // 7
    AVC,
}

// FrameType and CodecID is a u8
pub fn video_header(b: u8) -> Result<(FrameType, CodecID), String> {
    let frame = (b >> 4) & 0b1111;
    let frame = match frame {
        1 => FrameType::Key,
        2 => FrameType::Inter,
        3 => FrameType::DisposableInter,
        4 => FrameType::Generated,
        5 => FrameType::Video,
        _ => return Err("video_header".to_string()),
    };
    let codecid = b & 0b1111;
    let codecid = match codecid {
        1 => CodecID::JPEG,
        2 => CodecID::Sorenson,
        3 => CodecID::ScreenVideo,
        4 => CodecID::VP6,
        5 => CodecID::VP6A,
        6 => CodecID::ScreenVideo2,
        7 => CodecID::AVC,
        _ => return Err("video_header".to_string()),
    };
    Ok((frame, codecid))
}

#[derive(Debug, PartialEq)]
pub enum VideoDataByFrame {
    VideoFramePayload(VideoPacketData),
    // 0: Start of client-side seeking video frame sequence
    // 1: End of client-side seeking video frame sequence
    U8(u8),
}
#[derive(Debug, PartialEq)]
pub enum VideoPacketData {
    // codecid == 2
    H263VideoPacket(Vec<u8>),
    // codecid == 3
    ScreenVideoPacket(Vec<u8>),
    // codecid == 4
    VP6FLVVideoPacket(Vec<u8>),
    // codecid == 5
    VP6FLVAlphaVideoPacket(Vec<u8>),
    // codecid == 6
    ScreenV2VideoPacket(Vec<u8>),
    // codecid == 7
    AVCVideoPacket(Vec<u8>),
}

#[derive(Debug, PartialEq)]
pub struct VideoData {
    pub frame_type: FrameType,
    pub codec_id: CodecID,
    pub video_data: VideoDataByFrame,
}

pub fn video_data(input: &[u8], size: usize) -> Result<VideoData, String> {
    if input.len() < size {
        return Err("invalid video data length".to_string());
    }
    if size < 1 {
        return Err("video data length less than 1".to_string());
    }
    let (frame_type, codec_id) = video_header(input[0])?;

    let data = Vec::from(&input[1..size]);
    let video_data = match frame_type {
        FrameType::Video => VideoDataByFrame::U8(0),
        _ => match codec_id {
            CodecID::JPEG => return Err("jpeg should not used".to_string()),
            CodecID::Sorenson => {
                VideoFramePayload(VideoPacketData::H263VideoPacket(data))
            },
            CodecID::ScreenVideo => {
                VideoFramePayload(VideoPacketData::ScreenVideoPacket(data))
            },
            CodecID::VP6 => {
                VideoFramePayload(VideoPacketData::VP6FLVVideoPacket(data))
            },
            CodecID::VP6A => {
                VideoFramePayload(VideoPacketData::VP6FLVAlphaVideoPacket(data))
            },
            CodecID::ScreenVideo2 => {
                VideoFramePayload(VideoPacketData::ScreenV2VideoPacket(data))
            },
            CodecID::AVC => {
                VideoFramePayload(VideoPacketData::AVCVideoPacket(data))
            },
        },
    };

    Ok(VideoData{
        frame_type,
        codec_id,
        video_data,
    })
}

pub enum AVCPacketType {
    // 0
    SequenceHeader,
    // 1
    NALU,
    // 2
    // lower level NALU sequence ender is not required or supported
    EndOfSequence,
}

fn avc_packet_type(input: u8) -> Result<AVCPacketType, String> {
    match input {
        0 => Ok(AVCPacketType::SequenceHeader),
        1 => Ok(AVCPacketType::NALU),
        2 => Ok(AVCPacketType::EndOfSequence),
        _ => Err("invalid avc packet type".to_string()),
    }
}

// TODO:
pub struct AVCVideoPacket {
    pub avc_packet_type: AVCPacketType,
    // SI24
    // if AVCPacketType == 1, composition time offset
    // else 0
    pub composition_time: i32,
    // if AVCPacketType == 0, AVCDecoderConfigurationRecord
    // else if AVCPacketType == 1, one or more NALU,
    //   (can be individual slices per FLV packets; that is, full frames are not strictly required)
    // else if AVCPacketType == 2, Empty
    pub data: Vec<u8>,
}

pub fn avc_video_packet(input: &[u8], size: usize) -> Result<AVCVideoPacket, String> {
    if input.len() < size {
        return Err("avc video packet need more size".to_string());
    }
    if size < 4 {
        return Err("avc video packet need more than 4 length".to_string());
    }
    let avc_packet_type = avc_packet_type(input[0])?;
    let composition_time = (input[1] as i32) << 16 |
        (input[2] as i32) << 8 |
        input[3] as i32;

    Ok(AVCVideoPacket{
        avc_packet_type,
        composition_time,
        data: Vec::from(&input[4..size]),
    })
}

