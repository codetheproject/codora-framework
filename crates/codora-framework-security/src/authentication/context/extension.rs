use crate::Handler;

// This is basically same as http::Extension we will definently add reference

#[derive(Clone)]
pub struct ContextHandlerExtension {}

impl ContextHandlerExtension {
    pub fn register_handler<H>(&mut self, handler: H) -> &mut Self
    where
        H: Send + Sync,
    {
        self
    }

    pub fn get_handler_ref<T>(&self) -> &T {
        todo!()
    }

    pub fn get_handler_mut<T>(&mut self) -> &mut T {
        todo!()
    }
}
