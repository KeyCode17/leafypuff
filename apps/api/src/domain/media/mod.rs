pub mod error;
pub mod media_object;
pub mod object_key;
pub mod ports;
pub mod variant;

pub const MAX_OBJECT_BYTES: usize = 12 * 1024 * 1024;

pub use error::MediaError;
pub use media_object::MediaObject;
pub use object_key::ObjectKey;
pub use ports::{MediaRepository, ObjectStore};
pub use variant::Variant;
