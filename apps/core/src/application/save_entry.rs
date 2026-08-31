use crate::domain::{CoreError, Entry, EntryRepository};

pub struct SaveEntry<R: EntryRepository> {
    repository: R,
}

impl<R: EntryRepository> SaveEntry<R> {
    pub const fn new(repository: R) -> Self {
        Self { repository }
    }

    pub async fn execute(&self, entry: Entry) -> Result<Entry, CoreError> {
        if entry.title.trim().is_empty() && entry.body.trim().is_empty() {
            return Err(CoreError::Invalid(
                "An entry needs a title or a body".to_owned(),
            ));
        }
        self.repository.save(entry).await
    }
}
