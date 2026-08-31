use chrono::{DateTime, NaiveDate, Utc};

use super::entry::{Entry, EntryId};
use super::error::CoreError;

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
