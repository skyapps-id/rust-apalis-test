use crate::storage::postgres::StorageFactory;
use crate::usecase::{EmailSender, EmailService, OrderService, OrderUsecase};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppContainer {
    pub storage: Arc<StorageFactory>,
    pub email_service: Arc<dyn EmailSender>,
    pub order_service: Arc<dyn OrderUsecase>,
}

impl AppContainer {
    pub fn new(storage: Arc<StorageFactory>) -> Self {
        let order_service = Arc::new(OrderService::new(storage.clone())) as Arc<dyn OrderUsecase>;
        let email_service = Arc::new(EmailService) as Arc<dyn EmailSender>;

        Self {
            storage,
            email_service,
            order_service,
        }
    }
}
