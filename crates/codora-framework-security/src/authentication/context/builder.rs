use crate::{ContextHandlerExtension, Handler};
use std::sync::Arc;

#[derive(Clone)]
pub struct ContextBuilder<T = Arc<ContextHandlerExtension>> {
    pub(crate) handler_store: T,
}

impl ContextBuilder<ContextHandlerExtension> {
    pub fn builder() -> Self {
        ContextBuilder {
            handler_store: ContextHandlerExtension {},
        }
    }

    pub fn register_handler<H>(mut self, handler: H) -> Self
    where
        H: Send + Sync,
    {
        self.handler_store
            .register_handler(handler);
        self
    }

    /// Turn `[ContextBuilder<ContextHandlerExtenion>]` into `[CoktextBuilder<Arc<ContextHandlerExtension>>]`
    /// This allow us to easily clone `[ContextBuilder]`
    pub fn build(self) -> ContextBuilder<Arc<ContextHandlerExtension>> {
        ContextBuilder {
            handler_store: Arc::new(self.handler_store),
        }
    }
}
