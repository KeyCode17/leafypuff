use chrono::{DateTime, NaiveDate, Utc};
use uuid::Uuid;

use crate::domain::{
    COVER_ORDINAL, CoreError, ExifReader, PhotoKind, PhotoRef, PhotoStore, ThumbnailMaker,
};

pub struct ImportPhoto<S: PhotoStore, E: ExifReader, T: ThumbnailMaker> {
    store: S,
    exif: E,
    thumbnails: T,
}

impl<S: PhotoStore, E: ExifReader, T: ThumbnailMaker> ImportPhoto<S, E, T> {
    pub const fn new(store: S, exif: E, thumbnails: T) -> Self {
        Self {
            store,
            exif,
            thumbnails,
        }
    }

    pub fn execute(&self, bytes: &[u8]) -> Result<PhotoRef, CoreError> {
        let cover = self.thumbnails.cover(bytes)?;
        let taken_on = self.exif.taken_on(bytes)?;

        let id = Uuid::new_v4().hyphenated().to_string();
        let path = self.store.write(&id, PhotoKind::Original, bytes)?;
        self.store.write(&id, PhotoKind::Cover, &cover)?;

        Ok(PhotoRef {
            id,
            path,
            ordinal: COVER_ORDINAL,
            taken_at: taken_on.and_then(start_of_day),
        })
    }
}

fn start_of_day(day: NaiveDate) -> Option<DateTime<Utc>> {
    day.and_hms_opt(0, 0, 0).map(|at| at.and_utc())
}
