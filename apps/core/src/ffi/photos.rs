use crate::application::ImportPhoto;
use crate::domain::{ExifReader, PhotoKind, PhotoStore};

use super::LeafyPuffCore;
use super::error::LeafyPuffCoreError;
use super::records::{FfiPhoto, ISO_DATE};

#[uniffi::export]
impl LeafyPuffCore {
    /// Copies picked bytes into app storage, builds the top-anchored 3:2 cover
    /// beside them and hands back the reference an entry carries.
    ///
    /// `taken_at` on the returned record is RFC3339 at midnight UTC of the day
    /// the exif block reported, or null when the photo carries no readable one.
    pub fn import_photo(&self, bytes: Vec<u8>) -> Result<FfiPhoto, LeafyPuffCoreError> {
        let imported =
            ImportPhoto::new(&self.photos, self.exif, self.thumbnails).execute(&bytes)?;
        Ok(FfiPhoto::from(imported))
    }

    /// The day the picked bytes were taken, as `YYYY-MM-DD`, or null when the
    /// photo carries no readable `DateTimeOriginal`. This is what the exif
    /// prompt asks about; it never fails on an untagged or unreadable file.
    pub fn photo_taken_on(&self, bytes: Vec<u8>) -> Result<Option<String>, LeafyPuffCoreError> {
        let day = self.exif.taken_on(&bytes)?;
        Ok(day.map(|found| found.format(ISO_DATE).to_string()))
    }

    /// The jpeg bytes of a photo's cover derivative, ready to decode and draw.
    pub fn cover_thumbnail(&self, photo_id: String) -> Result<Vec<u8>, LeafyPuffCoreError> {
        Ok(self.photos.read(&photo_id, PhotoKind::Cover)?)
    }
}
