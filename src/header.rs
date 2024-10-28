// The FLV header
#[derive(Debug, PartialEq)]
pub struct FLVHeader {
    // sig: [u8;3], // Signature byte always 'FLV' (0x46 0x4c 0x56)
    pub version: u8, // File version (For example, 0x01 for FLV version 1)
    // reserved: [bit;5], // Type flags reserved, must be 0
    // audio: [bit;1], // Type flags audio, Audio tags are prevent
    // reserved2: [bit;1], // Type flags reserved, must be 0
    // video: [bit;1], // Tyep flags video, Video tags are prevent
    pub audio: bool,
    pub video: bool,
    // Offset in bytes from start of file to start of body
    // (that is, size of header)
    // usually has a value of 9 for FLV version 1.
    // This field is present to accommodate larger headers in future versions.
    pub data_offset: u32,
}

pub fn flv_header(input: &[u8]) -> Result<FLVHeader, String> {
    if input.len() < 9 {
        return Err("header not enough length".to_string());
    }
    // FLV Signature
    if input[0] == 0x46 && input[1] == 0x4c && input[2] == 0x56 {
        return Ok(FLVHeader {
            version: input[3],
            audio: input[4] & 0b100 == 4,
            video: input[4] & 0b1 == 1,
            data_offset: u32::from_be_bytes(
                [input[5],
                    input[6],
                    input[7],
                    input[8],
                ]
            )

        });
    } else {
        return Err("Invalid Signature".to_string());
    }
}

