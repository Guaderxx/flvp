# FLV parser

**WIP**

- Test files: [zelda][zelda], [zeldaHQ][zelda_hq], [commercials][commercials]
- [specification][spec]

## File Format

```plain
All File

  +--------------------+
  | FLV header         |    Header part (9 bytes)
  +--------------------+------------------------------
  | PreviousTagSize0   |    uint32 (4 bytes)
  +--------------------+    Body part
  |  Tag1              |
  +--------------------+
  | PreviousTagSize1   |
  +--------------------+
  |  Tag2              |
  +--------------------+
  | PreviousTagSize2   |
  +--------------------+
  |  ...               |
  +--------------------+
  | PreviousTagSizeN-1 |
  +--------------------+
  |  TagN              |
  +--------------------+
  | PreviousTagSizeN   |
  +--------------------+

```

### FLV Header

 Field | Type | Comment
--- | --- | ---
Signature | uint8 | Signature byte always 'F' (0x46)
Signature | uint8 | Signature byte always 'L' (0x4C)
Signature | uint8 | Signature byte always 'V' (0x56)
Version | uint8 | File version (for example, 0x01 for FLV version 1)
TypeFlagsReserved | [bit;5] | Must be 0
TypeFlagsAudio | [bit;1] | Audio tags are prevent
TypeFlagsReserved | [bit;1] | Must be 0
TypeFlagsVideo | [bit;1] | Video tags are prevent
DataOffset | uint32 | Offset in bytes from start of file to start of body (that is, size of header)

*The DataOffset field usually has a value of 9 for FLV version 1. This field is present to accommodate larger headers in future versions.*


### FLV Tag

Field | Type   | Comment
--- |--------| ---
TagType | uint8  | Type of this tag. Values are:  <br> 8: audio <br> 9: video <br> 18: script data <br> all others: reserved
DataSize | uint24 | Length of the data in the Data field
Timestamp | uint24 | 
TimestampExtended | uint8  | 
StreamID | uint24 | Always 0
Data | if TagType == 8 <br> AUDIODATA <br> if TagType == 9 <br> VIDEODATA <br> if TagType == 18 <br> SCRIPTDATAOBJECT | Body of the tag 

**Afterall, the tag without Data is 11 bytes**



[spec]: https://rtmp.veriskope.com/pdf/video_file_format_spec_v10.pdf
[zelda]: https://streams.videolan.org/samples/FLV/zelda.flv
[zelda_hq]: https://streams.videolan.org/samples/FLV/zeldaHQ.flv
[commercials]: https://streams.videolan.org/samples/FLV/asian-commercials-are-weird.flv
