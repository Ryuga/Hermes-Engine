use std::sync::Arc;

use crate::core::workers::{BoxManager, EphemeralBox, PersistentBox};

pub struct AppState {
    pub executor_pool: Arc<BoxManager<EphemeralBox>>,
    pub compiler_pool: Arc<BoxManager<PersistentBox>>
}

impl AppState {
    pub fn new(worker_count: usize) -> Self {

        let ten_percent = (worker_count as f64 * 0.1).ceil() as usize;
        AppState {
            executor_pool: Arc::new(BoxManager::new(0..worker_count, |id| EphemeralBox::new(id))),
            compiler_pool: Arc::new(
                    BoxManager::new(worker_count..worker_count + ten_percent, |id| PersistentBox::new(id)
                )
            ),
        }
    }
}
