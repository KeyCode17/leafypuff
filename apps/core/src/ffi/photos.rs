use crate::application::ImportPhoto;
use crate::domain::{ExifReader, PhotoKind, PhotoStore};

use super::LeafyPuffCore;
use super::error::LeafyPuffCoreError;
use super::records::{FfiPhoto, ISO_DATE};

#[uniffi::export]
impl LeafyPuffCore {
    pub fn import_photo(&self, bytes: Vec<u8>) -> Result<FfiPhoto, LeafyPuffCoreError> {
        let imported =
            ImportPhoto::new(&self.photos, self.exif, self.thumbnails).execute(&bytes)?;
        Ok(FfiPhoto::from(imported))
    }

    pub fn photo_taken_on(&self, bytes: Vec<u8>) -> Result<Option<String>, LeafyPuffCoreError> {
        let day = self.exif.taken_on(&bytes)?;
        Ok(day.map(|found| found.format(ISO_DATE).to_string()))
    }

    pub fn cover_thumbnail(&self, photo_id: String) -> Result<Vec<u8>, LeafyPuffCoreError> {
        Ok(self.photos.read(&photo_id, PhotoKind::Cover)?)
    }
}
