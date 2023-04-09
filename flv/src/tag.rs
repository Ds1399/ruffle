use crate::reader::FlvReader;
use crate::script::ScriptData;
use crate::sound::AudioData;
use crate::video::VideoData;

#[repr(u8)]
pub enum TagData<'a> {
    Audio(AudioData<'a>) = 8,
    Video(VideoData<'a>) = 9,
    Script(ScriptData<'a>) = 18,
}

pub struct Tag<'a> {
    timestamp: i32,
    stream_id: u32, //24 bits max
    data: TagData<'a>,
}

impl<'a> Tag<'a> {
    /// Parse a single FLV tag structure.
    ///
    /// This assumes the reader is currently pointing at the tag itself, not
    /// the back-pointers in between the tags.
    pub fn parse(reader: &mut FlvReader<'a>) -> Option<Self> {
        let tag_type = reader.read_u8()?;
        let data_size = reader.read_u24()?;
        let timestamp = reader.read_u24()?;
        let timestamp_extended = reader.read_u8()?;
        let stream_id = reader.read_u24()?;

        let timestamp = ((timestamp_extended as u32) << 24 | timestamp) as i32;

        match tag_type {
            8 => Some(Tag {
                timestamp,
                stream_id,
                data: TagData::Audio(AudioData::parse(reader, data_size)?),
            }),
            9 => Some(Tag {
                timestamp,
                stream_id,
                data: TagData::Video(VideoData::parse(reader, data_size)?),
            }),
            18 => Some(Tag {
                timestamp,
                stream_id,
                data: TagData::Script(ScriptData::parse(reader)?),
            }),
            _ => None,
        }
    }
}