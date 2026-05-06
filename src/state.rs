use std::sync::Arc;
use crate::core::workers::BoxManager;

pub struct AppState {
    pub box_manager: Arc<BoxManager>,
}

impl AppState {
    pub fn new(worker_count: i8) -> Self {
        AppState {
            box_manager: Arc::new(BoxManager::new(worker_count))
        }
    }
}
