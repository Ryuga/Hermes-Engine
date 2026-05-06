use std::sync::Arc;
use crate::core::workers::BoxManager;

pub struct AppState {
    pub box_manager: Arc<BoxManager>,
}

impl AppState {
    pub fn new(box_manager: Arc<BoxManager>) -> Self {
        AppState { box_manager }
    }
}
