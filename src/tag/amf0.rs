use std::f64;

#[derive(Debug, PartialEq)]
pub struct MetaData {
    pub duration: f64,
    pub width: f64,
    pub height: f64,
    // indicate the video bit rate in kilobits per second
    pub video_data_rate: f64,
    pub framerate: f64,
    pub video_codec_id: f64,
    pub audio_sample_rate: f64,
    pub audio_sample_size: f64,
    pub stereo: bool,
    pub audio_codec_id: f64,
    pub file_size: f64,
}

#[allow(dead_code)]
const AMF_END_OF_OBJECT: u8 =   0x09;

pub(crate) const AMF_DATA_TYPE_NUMBER: u8 = 0;
pub(crate) const AMF_DATA_TYPE_BOOL: u8 = 1;
pub(crate) const AMF_DATA_TYPE_STRING: u8 = 2;
pub(crate) const AMF_DATA_TYPE_OBJECT: u8 = 3;
pub(crate) const AMF_DATA_TYPE_NULL: u8 = 5;
pub(crate) const AMF_DATA_TYPE_UNDEFINED: u8 = 6;
pub(crate) const AMF_DATA_TYPE_REFERENCE: u8 = 7;
pub(crate) const AMF_DATA_TYPE_MIXEDARRAY: u8 = 8;
pub(crate) const AMF_DATA_TYPE_OBJECT_END: u8 = 9;
pub(crate) const AMF_DATA_TYPE_ARRAY: u8 = 10;
pub(crate) const AMF_DATA_TYPE_DATE: u8 = 11;
pub(crate) const AMF_DATA_TYPE_LONG_STRING: u8 = 12;
pub(crate) const AMF_DATA_TYPE_UNSUPPORTED: u8 = 13;

#[derive(Debug,PartialEq)]
pub enum AMFData {
    Number(f64),
    Bool(bool),
    String(String),
    Object(Vec<AMFObject>),
    Null,
    Undefined,
    Reference(u16),
    Mixedarray(Vec<AMFObject>),
    ObjectEnd,
    Array(Vec<AMFData>),
    Date(AMFDate),
    LongString(String),
    Unsupported,
}

#[derive(Debug, PartialEq)]
pub struct AMFObject {
    pub name: String,
    pub data: AMFData,
}

#[derive(Debug, PartialEq)]
pub struct AMFDate {
    pub milliseconds: f64,
    pub timezone: i16,
}

pub fn amf_data_value(input: &[u8]) -> Result<(AMFData, &[u8]), String> {
    match input[0] {
        AMF_DATA_TYPE_NUMBER => {
            Ok((AMFData::Number(f64::from_be_bytes([
                input[1],
                input[2],
                input[3],
                input[4],
                input[5],
                input[6],
                input[7],
                input[8],
            ])), &input[9..]))
        },
        AMF_DATA_TYPE_BOOL => {
            Ok((AMFData::Bool(input[1] != 0), &input[2..]))
        },
        AMF_DATA_TYPE_STRING => {
            let (res, last) = amf_string(&input[1..])?;
            Ok((AMFData::String(res), last))
        },
        AMF_DATA_TYPE_OBJECT => {
            let (res, last) = amf_objects(&input[1..])?;
            Ok((AMFData::Object(res), last))
        }, // 3
        AMF_DATA_TYPE_NULL => Ok((AMFData::Null, &input[1..])), // 5
        AMF_DATA_TYPE_UNDEFINED => Ok((AMFData::Undefined, &input[1..])), // 6
        AMF_DATA_TYPE_REFERENCE => {
            Ok((AMFData::Reference(u16::from_be_bytes([input[1], input[2]])), &input[3..]))
        },
        AMF_DATA_TYPE_MIXEDARRAY => {
            let (res, last) = amf_ecma_array(&input[1..])?;
            Ok((AMFData::Mixedarray(res), last))
        }, // 8
        AMF_DATA_TYPE_OBJECT_END => Ok((AMFData::ObjectEnd, &input[1..])), // 9
        AMF_DATA_TYPE_ARRAY => {
            let (res, last) = amf_strict_array(&input[1..])?;
            Ok((AMFData::Array(res), last))
        }, // 10
        AMF_DATA_TYPE_DATE => {
            let (date, last) = amf_date(&input[1..]);
            Ok((AMFData::Date(date), last))
        }, // 11
        AMF_DATA_TYPE_LONG_STRING => {
            let (res, last) = amf_long_string(&input[1..])?;
            Ok((AMFData::LongString(res), last))
        },
        AMF_DATA_TYPE_UNSUPPORTED => Err("unsupported".to_string()),
        other => Err(format!("reserved: {}", other).to_string())
    }
}

pub fn amf_string(input: &[u8]) -> Result<(String, &[u8]), String> {
    let string_length = u16::from_be_bytes([input[0], input[1]]);
    let string_data = String::from_utf8(Vec::from(&input[2..(2+string_length) as usize]))
        .map_err(|e| e.to_string())?;
    Ok((string_data, &input[(2+string_length) as usize..]))
}

pub fn amf_long_string(input: &[u8]) -> Result<(String, &[u8]), String> {
    let string_length = u32::from_be_bytes([input[0], input[1], input[2], input[3]]);
    let string_data = String::from_utf8(Vec::from(&input[4..(4 + string_length) as usize]))
        .map_err(|e| e.to_string())?;
    Ok((string_data, &input[(4 + string_length) as usize..]))
}

pub fn amf_date(input: &[u8]) -> (AMFDate, &[u8]) {
    let milliseconds = f64::from_be_bytes([
        input[0],
        input[1],
        input[2],
        input[3],
        input[4],
        input[5],
        input[6],
        input[7],
    ]);
    let timezone = i16::from_be_bytes([input[8],input[9]]);
    (AMFDate{
        milliseconds,
        timezone,
    }, &input[10..])
}

pub fn amf_object(input: &[u8]) -> Result<(AMFObject, &[u8]), String> {
    let (name, last) = amf_string(input)?;
    let (data, last) = amf_data_value(last)?;
    Ok((AMFObject{
        name,
        data,
    }, last))
}

pub fn amf_objects(input: &[u8]) -> Result<(Vec<AMFObject>, &[u8]), String> {
    let mut res: Vec<AMFObject> = Vec::new();
    let mut last = input;
    let mut obj = AMFObject{
        name: "".to_string(),
        data: AMFData::Null,
    };

    loop {
        (obj, last) = amf_object(last)?;
        if obj.data != AMFData::ObjectEnd {
            res.push(obj);
        } else {
            return Ok((res, last))
        }
    }
}

pub fn amf_ecma_array(input: &[u8]) -> Result<(Vec<AMFObject>, &[u8]), String> {
    let _arr_len = u32::from_be_bytes([
        input[0],
        input[1],
        input[2],
        input[3],
    ]);
    // println!("ecma array length: {}", arr_len);
    Ok(amf_objects(&input[4..])?)
}

pub fn amf_strict_array(input: &[u8]) -> Result<(Vec<AMFData>, &[u8]), String> {
    let arr_len = u32::from_be_bytes([
        input[0],
        input[1],
        input[2],
        input[3],
    ]);
    // println!("strict array length: {}", arr_len);

    let mut idx = 0;
    let mut last = &input[4..];
    let mut data = AMFData::Null;
    let mut res: Vec<AMFData> = Vec::new();

    while idx < arr_len {
        (data, last) = amf_data_value(last)?;
        res.push(data);
        idx += 1;
    }

    Ok((res, last))
}

// TODO: I don't get the spec.
// This should be a SCRIPTDATAOBJECT, but before the data, still need the [2] as string marker.
pub fn amf_data(input: &[u8]) -> Result<(AMFObject, &[u8]), String> {
    if input[0] == AMF_DATA_TYPE_STRING {
        Ok(amf_object(&input[1..])?)
    } else {
        Err("invalid data type".to_string())
    }
}

