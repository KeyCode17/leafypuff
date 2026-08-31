use std::io::Cursor;

use chrono::NaiveDate;
use exif::{Error as ExifError, In, Reader, Tag, Value};

use crate::domain::error::ERR_EXIF_UNREADABLE;
use crate::domain::{CoreError, ExifReader};

/// Reads `DateTimeOriginal` out of the bytes the picker handed over.
///
/// A file that carries no Exif block, a truncated or malformed block and a
/// payload that is not an image at all all resolve to `Ok(None)`: the prompt
/// simply does not fire. Only a read failure of the byte source itself is
/// reported as [`CoreError::Exif`].
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

        let Some(field) = block.get_field(Tag::DateTimeOriginal, In::PRIMARY) else {
            return Ok(None);
        };
        let Value::Ascii(ref parts) = field.value else {
            return Ok(None);
        };
        let Some(first) = parts.first() else {
            return Ok(None);
        };
        let Ok(stamp) = exif::DateTime::from_ascii(first) else {
            return Ok(None);
        };

        Ok(NaiveDate::from_ymd_opt(
            i32::from(stamp.year),
            u32::from(stamp.month),
            u32::from(stamp.day),
        ))
    }
}
