use std::sync::Arc;

use crate::storage::redis::StorageFactory;
use crate::AppContainer;

pub mod router;

pub use router::*;

#[derive(Clone)]
pub struct ServerState {
    pub container: AppContainer,
}

impl ServerState {
    pub fn new(container: AppContainer) -> Self {
        Self { container }
    }
    
    pub fn storage(&self) -> &Arc<StorageFactory> {
        &self.container.storage
    }
}
