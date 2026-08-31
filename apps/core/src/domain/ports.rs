use super::entry::{Entry, EntryId};
use super::error::CoreError;

pub trait EntryRepository {
    fn save(&self, entry: Entry) -> impl Future<Output = Result<Entry, CoreError>>;

    fn find(&self, id: EntryId) -> impl Future<Output = Result<Option<Entry>, CoreError>>;
}
