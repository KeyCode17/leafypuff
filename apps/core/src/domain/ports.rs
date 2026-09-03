use chrono::{DateTime, NaiveDate, Utc};

use super::crop::Framing;
use super::entry::{Entry, EntryId};
use super::error::CoreError;
use super::photo::PhotoKind;

pub trait EntryRepository {
    fn save(&self, entry: Entry) -> impl Future<Output = Result<Entry, CoreError>>;

    fn by_id(&self, id: EntryId) -> impl Future<Output = Result<Option<Entry>, CoreError>>;

    fn list_desc(&self, limit: u32) -> impl Future<Output = Result<Vec<Entry>, CoreError>>;

    fn in_range(
        &self,
        from: NaiveDate,
        to: NaiveDate,
    ) -> impl Future<Output = Result<Vec<Entry>, CoreError>>;

    fn on_date(&self, date: NaiveDate) -> impl Future<Output = Result<Vec<Entry>, CoreError>>;

    fn delete(&self, id: EntryId) -> impl Future<Output = Result<(), CoreError>>;

    fn delete_all(&self) -> impl Future<Output = Result<(), CoreError>>;
}

impl<R: EntryRepository> EntryRepository for &R {
    fn save(&self, entry: Entry) -> impl Future<Output = Result<Entry, CoreError>> {
        (*self).save(entry)
    }

    fn by_id(&self, id: EntryId) -> impl Future<Output = Result<Option<Entry>, CoreError>> {
        (*self).by_id(id)
    }

    fn list_desc(&self, limit: u32) -> impl Future<Output = Result<Vec<Entry>, CoreError>> {
        (*self).list_desc(limit)
    }

    fn in_range(
        &self,
        from: NaiveDate,
        to: NaiveDate,
    ) -> impl Future<Output = Result<Vec<Entry>, CoreError>> {
        (*self).in_range(from, to)
    }

    fn on_date(&self, date: NaiveDate) -> impl Future<Output = Result<Vec<Entry>, CoreError>> {
        (*self).on_date(date)
    }

    fn delete(&self, id: EntryId) -> impl Future<Output = Result<(), CoreError>> {
        (*self).delete(id)
    }

    fn delete_all(&self) -> impl Future<Output = Result<(), CoreError>> {
        (*self).delete_all()
    }
}

pub trait Clock {
    fn now(&self) -> DateTime<Utc>;

    fn today(&self) -> NaiveDate {
        self.now().date_naive()
    }
}

pub trait ExifReader {
    fn taken_on(&self, bytes: &[u8]) -> Result<Option<NaiveDate>, CoreError>;
}

pub trait ThumbnailMaker {
    fn cover(&self, bytes: &[u8]) -> Result<Vec<u8>, CoreError>;

    fn framed_cover(&self, bytes: &[u8], framing: Framing) -> Result<Vec<u8>, CoreError>;

    fn framed_square(&self, bytes: &[u8], framing: Framing) -> Result<Vec<u8>, CoreError>;
}

pub trait PhotoStore {
    fn write(&self, id: &str, kind: PhotoKind, bytes: &[u8]) -> Result<String, CoreError>;

    fn read(&self, id: &str, kind: PhotoKind) -> Result<Vec<u8>, CoreError>;
}

impl<S: PhotoStore> PhotoStore for &S {
    fn write(&self, id: &str, kind: PhotoKind, bytes: &[u8]) -> Result<String, CoreError> {
        (*self).write(id, kind, bytes)
    }

    fn read(&self, id: &str, kind: PhotoKind) -> Result<Vec<u8>, CoreError> {
        (*self).read(id, kind)
    }
}

pub trait FieldSealer {
    fn seal_field(
        &self,
        entry_id: crate::domain::EntryId,
        field_name: &str,
        field_updated_at_ms: i64,
        plain: &str,
    ) -> Result<crate::domain::crypto::SealedField, CoreError>;

    fn open_field(
        &self,
        entry_id: crate::domain::EntryId,
        field_name: &str,
        field_updated_at_ms: i64,
        ciphertext: &[u8],
        nonce: &[u8],
    ) -> Result<String, CoreError>;
}

pub trait ContentSealer {
    fn seal(&self, label: &str, plain: &[u8]) -> Result<Vec<u8>, CoreError>;

    fn open(&self, label: &str, sealed: &[u8]) -> Result<Vec<u8>, CoreError>;
}
