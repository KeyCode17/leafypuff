use std::io::Cursor;

use chrono::NaiveDate;
use exif::{Error as ExifError, In, Reader, Tag, Value};

use crate::domain::error::ERR_EXIF_UNREADABLE;
use crate::domain::{CoreError, ExifReader};

#[derive(Debug, Default, Clone, Copy)]
pub struct KamadakExifReader;

impl ExifReader for KamadakExifReader {
    fn taken_on(&self, bytes: &[u8]) -> Result<Option<NaiveDate>, CoreError> {
        let mut cursor = Cursor::new(bytes);
        let block = match Reader::new().read_from_container(&mut cursor) {
            Ok(block) => block,
            Err(ExifError::Io(cause)) => {
                return Err(CoreError::Exif(format!("{ERR_EXIF_UNREADABLE}: {cause}")));
            }
            Err(_) => return Ok(None),
        };

        Ok(DAY_TAGS
            .iter()
            .find_map(|tag| day_of(block.get_field(*tag, In::PRIMARY)?)))
    }
}

const DAY_TAGS: [Tag; 2] = [Tag::DateTimeOriginal, Tag::DateTimeDigitized];

fn day_of(field: &exif::Field) -> Option<NaiveDate> {
    let Value::Ascii(ref parts) = field.value else {
        return None;
    };
    let stamp = exif::DateTime::from_ascii(parts.first()?).ok()?;
    NaiveDate::from_ymd_opt(
        i32::from(stamp.year),
        u32::from(stamp.month),
        u32::from(stamp.day),
    )
}
