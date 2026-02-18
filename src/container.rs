use crate::workflow::usecase::{EmailService, OrderService};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppContainer {
    pub email_service: Arc<EmailService>,
    pub order_service: Arc<OrderService>,
}

impl AppContainer {
    pub fn new(email_service: Arc<EmailService>, order_service: Arc<OrderService>) -> Self {
        Self {
            email_service,
            order_service,
        }
    }

    pub fn default() -> Self {
        Self {
            email_service: Arc::new(EmailService),
            order_service: Arc::new(OrderService),
        }
    }
}
