pub mod bundle;
pub mod error;
pub mod ports;

pub const MAX_PAYLOAD_BYTES: usize = 256 * 1024;

pub use bundle::CatalogBundle;
pub use error::CatalogError;
pub use ports::CatalogStore;
