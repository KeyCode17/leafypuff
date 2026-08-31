use crate::domain::error::{ERR_ENTRY_EMPTY, ERR_ENTRY_ID_INVALID};
use crate::domain::{Clock, CoreError, Entry, EntryRepository};

pub struct SaveEntry<R: EntryRepository, C: Clock> {
    repository: R,
    clock: C,
}

impl<R: EntryRepository, C: Clock> SaveEntry<R, C> {
    pub const fn new(repository: R, clock: C) -> Self {
        Self { repository, clock }
    }

    pub async fn execute(&self, draft: Entry) -> Result<Entry, CoreError> {
        if draft.id.is_nil() {
            return Err(CoreError::Invalid(ERR_ENTRY_ID_INVALID.to_owned()));
        }
        if draft.title.trim().is_empty() && draft.body.trim().is_empty() {
            return Err(CoreError::Invalid(ERR_ENTRY_EMPTY.to_owned()));
        }

        let now = self.clock.now();
        let stamped = match self.repository.by_id(draft.id).await? {
            Some(existing) => Entry {
                created_at: existing.created_at,
                updated_at: now,
                ..draft
            },
            None => Entry {
                created_at: now,
                updated_at: now,
                ..draft
            },
        };

        self.repository.save(stamped).await
    }
}
