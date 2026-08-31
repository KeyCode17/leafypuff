pub mod clock;
#[cfg(feature = "test-support")]
pub mod memory;

pub use clock::SystemClock;
#[cfg(feature = "test-support")]
pub use memory::{FixedClock, InMemoryEntryRepository};
