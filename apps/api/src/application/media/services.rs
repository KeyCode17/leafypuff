use std::sync::Arc;

use crate::domain::media::{MediaRepository, ObjectStore};

use super::forget_object::ForgetObject;
use super::read_object::ReadObject;
use super::store_object::StoreObject;

#[derive(Clone)]
pub struct MediaServices {
    pub objects: Arc<dyn ObjectStore>,
    pub media: Arc<dyn MediaRepository>,
}

impl MediaServices {
    pub fn store(&self) -> StoreObject {
        StoreObject::new(Arc::clone(&self.objects), Arc::clone(&self.media))
    }

    pub fn read(&self) -> ReadObject {
        ReadObject::new(Arc::clone(&self.objects), Arc::clone(&self.media))
    }

    pub fn forget(&self) -> ForgetObject {
        ForgetObject::new(Arc::clone(&self.objects), Arc::clone(&self.media))
    }
}
