use std::sync::Arc;

use crate::domain::media::{MediaRepository, ObjectStore};

use super::forget_object::ForgetObject;
use super::read_avatar::ReadAvatar;
use super::read_object::ReadObject;
use super::store_avatar::StoreAvatar;
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

    pub fn store_avatar(&self) -> StoreAvatar {
        StoreAvatar::new(Arc::clone(&self.objects))
    }

    pub fn read_avatar(&self) -> ReadAvatar {
        ReadAvatar::new(Arc::clone(&self.objects))
    }

    pub fn forget(&self) -> ForgetObject {
        ForgetObject::new(Arc::clone(&self.objects), Arc::clone(&self.media))
    }
}
