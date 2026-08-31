use std::fs;
use std::path::{Path, PathBuf};

use crate::domain::error::{ERR_PHOTO_ID_INVALID, ERR_PHOTO_MISSING, ERR_PHOTO_STORE};
use crate::domain::{ContentSealer, CoreError, PhotoKind, PhotoStore};

pub const PHOTO_DIR: &str = "photos";

/// App storage for picked photos. Every blob crosses [`ContentSealer`] on the
/// way in and on the way out, so nothing above this adapter has to know whether
/// what lands on disk is plaintext or ciphertext.
pub struct FilePhotoStore<S: ContentSealer> {
    root: PathBuf,
    sealer: S,
}

impl<S: ContentSealer> FilePhotoStore<S> {
    pub const fn new(root: PathBuf, sealer: S) -> Self {
        Self { root, sealer }
    }

    /// The photo directory the core owns, beside the database file it opened.
    pub fn beside(database: &str, sealer: S) -> Self {
        let root = Path::new(database)
            .parent()
            .map_or_else(|| PathBuf::from(PHOTO_DIR), |dir| dir.join(PHOTO_DIR));
        Self::new(root, sealer)
    }

    pub fn root(&self) -> &Path {
        &self.root
    }

    fn blob_path(&self, id: &str, kind: PhotoKind) -> Result<PathBuf, CoreError> {
        let safe = !id.is_empty()
            && id
                .chars()
                .all(|letter| letter.is_ascii_alphanumeric() || letter == '-');
        if !safe {
            return Err(CoreError::Invalid(format!("{ERR_PHOTO_ID_INVALID}: {id}")));
        }
        Ok(self.root.join(format!("{id}{}", kind.suffix())))
    }
}

fn seal_label(id: &str, kind: PhotoKind) -> String {
    format!("photo:{id}:{}", kind.label())
}

impl<S: ContentSealer> PhotoStore for FilePhotoStore<S> {
    fn write(&self, id: &str, kind: PhotoKind, bytes: &[u8]) -> Result<String, CoreError> {
        let path = self.blob_path(id, kind)?;
        let sealed = self.sealer.seal(&seal_label(id, kind), bytes)?;
        fs::create_dir_all(&self.root)
            .map_err(|cause| CoreError::Storage(format!("{ERR_PHOTO_STORE}: {cause}")))?;
        fs::write(&path, sealed)
            .map_err(|cause| CoreError::Storage(format!("{ERR_PHOTO_STORE}: {cause}")))?;
        Ok(path.to_string_lossy().into_owned())
    }

    fn read(&self, id: &str, kind: PhotoKind) -> Result<Vec<u8>, CoreError> {
        let path = self.blob_path(id, kind)?;
        let sealed = fs::read(&path)
            .map_err(|cause| CoreError::Photo(format!("{ERR_PHOTO_MISSING}: {id} ({cause})")))?;
        self.sealer.open(&seal_label(id, kind), &sealed)
    }
}
