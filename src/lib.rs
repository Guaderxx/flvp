pub mod header;
pub mod tag;

/*
   FLV File Format
   - FLV Header - 9 bytes
   - TAG - 4 bytes
   - TagHeader - 11 bytes
*/

#[allow(dead_code)]
static SCRIPT_DATA_NAME_TAG: &[u8] = &[2];

#[cfg(test)]
mod test {
    use super::*;
    use crate::tag::audio::{SoundFormat, SoundRate, SoundSize, SoundType};
    use crate::tag::{TAG_TYPE_AUDIO, TAG_TYPE_SCRIPT, TAG_TYPE_VIDEO};
    use crate::tag::video::{CodecID, FrameType, VideoData, VideoDataByFrame, VideoPacketData};

    const ZELDA: &[u8] = include_bytes!("../assets/zelda.flv");
    const ZELDA_HQ: &[u8] = include_bytes!("../assets/zeldaHQ.flv");
    const COMMERCIAL: &[u8] = include_bytes!("../assets/asian-commercials-are-weird.flv");
    #[test]
    fn header_test() {
        assert_eq!(
            header::flv_header(&ZELDA[..9]),
            Ok(header::FLVHeader {
                version: 1,
                audio: true,
                video: true,
                data_offset: 9,
            })
        );
        assert_eq!(
            header::flv_header(&ZELDA_HQ[..9]),
            Ok(header::FLVHeader {
                version: 1,
                audio: true,
                video: true,
                data_offset: 9,
            })
        );
        assert_eq!(
            header::flv_header(&COMMERCIAL[..9]),
            Ok(header::FLVHeader {
                version: 1,
                audio: true,
                video: true,
                data_offset: 9,
            })
        );
    }

    #[test]
    fn first_tag_header() {
        // starts at 9 bytes (header)
        // + 4 (size of previous tag)
        // header is 11 bytes long
        assert_eq!(
            tag::tag_header(&ZELDA[13..24]),
            Ok(tag::TagHeader {
                tag_type: TAG_TYPE_VIDEO,
                data_size: 537,
                timestamp: 0,
                stream_id: 0,
            })
        );
        assert_eq!(
            tag::tag_header(&ZELDA_HQ[13..24]),
            Ok(tag::TagHeader {
                tag_type: TAG_TYPE_VIDEO,
                data_size: 2984,
                timestamp: 0,
                stream_id: 0,
            })
        );
        assert_eq!(
            tag::tag_header(&COMMERCIAL[13..24]),
            Ok(tag::TagHeader {
                tag_type: TAG_TYPE_SCRIPT,
                data_size: 273,
                timestamp: 0,
                stream_id: 0,
            })
        );
    }

    #[test]
    fn audio_tags() {
        /*
        header 9
        size 4
        tagHeader 11
        tagData 537
        size 4
         */
        let tag_start: usize = 24 + 537 + 4;
        let idx: usize = 24 + 537;
        let size = u32::from_be_bytes([ZELDA[idx], ZELDA[idx + 1], ZELDA[idx + 2], ZELDA[idx + 3]]);
        println!("size of previous tag: {:?}", size);

        assert_eq!(
            tag::tag_header(&ZELDA[tag_start..tag_start + 11]),
            Ok(tag::TagHeader {
                tag_type: TAG_TYPE_AUDIO,
                data_size: 642,
                timestamp: 0,
                stream_id: 0,
            })
        );

        println!(
            "data: {:?}",
            tag::audio::audio_data(&ZELDA[tag_start + 11..tag_start + 11 + 642], 642)
        );

        assert_eq!(
            tag::audio::audio_data(&ZELDA[tag_start + 11..tag_start + 11 + 642], 642,),
            Ok(tag::audio::AudioData {
                sound_format: SoundFormat::ADPCM,
                sound_rate: SoundRate::_22KHZ,
                sound_size: SoundSize::_16Bit,
                sound_type: SoundType::Mono,
                sound_data: Vec::from(&ZELDA[tag_start + 12..tag_start + 11 + 642]),
            })
        );

        let tag_start = 24 + 2984 + 4;
        let idx = 24 + 2984;
        let size = u32::from_be_bytes([
            ZELDA_HQ[idx],
            ZELDA_HQ[idx + 1],
            ZELDA_HQ[idx + 2],
            ZELDA_HQ[idx + 3],
        ]);
        println!("size of previous tag: {:?}", size,);
        assert_eq!(
            tag::tag_header(&ZELDA_HQ[tag_start..tag_start + 11]),
            Ok(tag::TagHeader {
                tag_type: TAG_TYPE_AUDIO,
                data_size: 642,
                timestamp: 0,
                stream_id: 0,
            })
        );

        println!(
            "data: {:?}",
            tag::audio::audio_data(&ZELDA_HQ[tag_start + 11..tag_start + 11 + 642], 642)
        );

        assert_eq!(
            tag::audio::audio_data(&ZELDA_HQ[tag_start + 11..tag_start + 11 + 642], 642,),
            Ok(tag::audio::AudioData {
                sound_format: SoundFormat::ADPCM,
                sound_rate: SoundRate::_22KHZ,
                sound_size: SoundSize::_16Bit,
                sound_type: SoundType::Mono,
                sound_data: Vec::from(&ZELDA_HQ[tag_start + 12..tag_start + 11 + 642]),
            })
        );
    }

    #[test]
    fn video_tags() {
        let tag_start = 24;
        assert_eq!(
            tag::video::video_data(&ZELDA[tag_start..tag_start + 537], 537),
            Ok(tag::video::VideoData {
                frame_type: tag::video::FrameType::Key,
                codec_id: tag::video::CodecID::Sorenson,
                video_data: tag::video::VideoDataByFrame::VideoFramePayload(
                    tag::video::VideoPacketData::H263VideoPacket(Vec::from(
                        &ZELDA[tag_start + 1..tag_start + 537]
                    )),
                ),
            })
        );

        assert_eq!(
            tag::video::video_data(&ZELDA_HQ[tag_start..tag_start + 537], 537),
            Ok(tag::video::VideoData {
                frame_type: tag::video::FrameType::Key,
                codec_id: tag::video::CodecID::Sorenson,
                video_data: tag::video::VideoDataByFrame::VideoFramePayload(
                    tag::video::VideoPacketData::H263VideoPacket(Vec::from(
                        &ZELDA_HQ[tag_start + 1..tag_start + 537]
                    )),
                ),
            })
        )
    }


    #[test]
    fn script_tags() {
        let tag_start = 24;
        let tag_end = tag_start + 273;

        let (script_data, last) = tag::amf0::amf_data(&COMMERCIAL[tag_start..tag_end]).unwrap();
        assert_eq!(last.len(), 0);
        assert_eq!(script_data,tag::amf0::AMFObject {
                name: "onMetaData".to_string(),
                data: tag::amf0::AMFData::Mixedarray(vec![
                    tag::amf0::AMFObject {
                        name: "duration".to_string(),
                        data: tag::amf0::AMFData::Number(28.133),
                    },
                    tag::amf0::AMFObject {
                        name: "width".to_string(),
                        data: tag::amf0::AMFData::Number(464.0),
                    },
                    tag::amf0::AMFObject {
                        name: "height".to_string(),
                        data: tag::amf0::AMFData::Number(348.0),
                    },
                    tag::amf0::AMFObject {
                        name: "videodatarate".to_string(),
                        data: tag::amf0::AMFData::Number(368.0),
                    },
                    tag::amf0::AMFObject {
                        name: "framerate".to_string(),
                        data: tag::amf0::AMFData::Number(30.0),
                    },
                    tag::amf0::AMFObject {
                        name: "videocodecid".to_string(),
                        data: tag::amf0::AMFData::Number(4.0),
                    },
                    tag::amf0::AMFObject {
                        name: "audiodatarate".to_string(),
                        data: tag::amf0::AMFData::Number(56.0),
                    },
                    tag::amf0::AMFObject {
                        name: "audiodelay".to_string(),
                        data: tag::amf0::AMFData::Number(0.0),
                    },
                    tag::amf0::AMFObject {
                        name: "audiocodecid".to_string(),
                        data: tag::amf0::AMFData::Number(2.0),
                    },
                    tag::amf0::AMFObject {
                        name: "canSeekToEnd".to_string(),
                        data: tag::amf0::AMFData::Number(1.0),
                    },
                    tag::amf0::AMFObject {
                        name: "creationdate".to_string(),
                        data: tag::amf0::AMFData::String("Thu Oct 04 18:37:42 2007\n".to_string()),
                    },
                ]),
            },
        );
    }

    #[test]
    fn complete_video_tags() {
        let tag_start = 13;
        let tag_data_start = tag_start + 11;

        assert_eq!(
            tag::tag(&ZELDA[tag_start..tag_data_start+537]),
            Ok((tag::Tag{
                header: tag::TagHeader{
                    tag_type: TAG_TYPE_VIDEO,
                    data_size: 537,
                    timestamp: 0,
                    stream_id: 0,
                },
                data: tag::TagData::Video(VideoData{
                    frame_type: tag::video::FrameType::Key,
                    codec_id: tag::video::CodecID::Sorenson,
                    video_data: tag::video::VideoDataByFrame::VideoFramePayload(tag::video::VideoPacketData::H263VideoPacket(Vec::from(&ZELDA[tag_data_start+1..tag_data_start+537]))),
                })
            }, &b""[..]))
        );

        assert_eq!(tag::tag(&ZELDA_HQ[tag_start..tag_data_start+2984]),
            Ok((tag::Tag{
                header: tag::TagHeader{
                    tag_type: tag::TAG_TYPE_VIDEO,
                    data_size: 2984,
                    timestamp: 0,
                    stream_id: 0,
                },
                data: tag::TagData::Video(
                    tag::video::VideoData{
                        frame_type: FrameType::Key,
                        codec_id: CodecID::Sorenson,
                        video_data: VideoDataByFrame::VideoFramePayload(VideoPacketData::H263VideoPacket(Vec::from(&ZELDA_HQ[tag_data_start+1..tag_data_start+2984]))),
                    }
                ),
            }, &b""[..]))
        );
    }
}
