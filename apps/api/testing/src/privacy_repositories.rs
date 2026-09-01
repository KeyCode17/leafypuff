use std::sync::{Arc, Mutex};

use async_trait::async_trait;
use leafypuff_api::domain::privacy::{
    DataRequest, DataRequestStore, Eraser, PrivacyError, RequestStatus,
};
use uuid::Uuid;

#[derive(Clone, Default)]
pub struct InMemoryRequests {
    rows: Arc<Mutex<Vec<DataRequest>>>,
}

impl InMemoryRequests {
    pub fn snapshot(&self) -> Vec<DataRequest> {
        self.rows.lock().expect("the request lock holds").clone()
    }
}

#[async_trait]
impl DataRequestStore for InMemoryRequests {
    async fn open(&self) -> Result<Vec<DataRequest>, PrivacyError> {
        Ok(self
            .snapshot()
            .into_iter()
            .filter(|row| row.status == RequestStatus::Received)
            .collect())
    }

    async fn record(&self, request: DataRequest) -> Result<DataRequest, PrivacyError> {
        self.rows
            .lock()
            .expect("the request lock holds")
            .push(request.clone());
        Ok(request)
    }

    async fn find(&self, request_id: Uuid) -> Result<DataRequest, PrivacyError> {
        self.snapshot()
            .into_iter()
            .find(|row| row.id == request_id)
            .ok_or(PrivacyError::NotFound)
    }

    async fn mark_fulfilled(
        &self,
        request_id: Uuid,
        actor_id: Uuid,
        at_ms: i64,
    ) -> Result<(), PrivacyError> {
        let mut rows = self.rows.lock().expect("the request lock holds");
        let Some(row) = rows
            .iter_mut()
            .find(|row| row.id == request_id && row.status == RequestStatus::Received)
        else {
            return Err(PrivacyError::AlreadyFulfilled);
        };
        row.status = RequestStatus::Fulfilled;
        row.fulfilled_at_ms = Some(at_ms);
        row.fulfilled_by = Some(actor_id);
        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct RecordingEraser {
    erased: Arc<Mutex<Vec<Uuid>>>,
}

impl RecordingEraser {
    pub fn erased(&self) -> Vec<Uuid> {
        self.erased.lock().expect("the eraser lock holds").clone()
    }
}

#[async_trait]
impl Eraser for RecordingEraser {
    async fn erase(&self, account_id: Uuid) -> Result<(), PrivacyError> {
        self.erased
            .lock()
            .expect("the eraser lock holds")
            .push(account_id);
        Ok(())
    }
}
