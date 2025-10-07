use std::sync::Arc;
pub(super) mod builder;
pub(super) mod extension;
use extension::ContextHandlerExtension;

use crate::SignInHandler;

#[derive(new, Clone)]
pub struct Context<Request> {
    request: Request,
    inner: Arc<ContextHandlerExtension>,
}

impl<Request> Context<Request> {
    pub async fn sign_in<H>(&self, state: H::State, payload: H::Payload) -> Result<H::Response, H::Error>
    where
        H: SignInHandler<Request>,
    {
        // log and metrics later
        self.inner
            .get_handler_ref::<H>()
            .sign_in(self, state, payload)
            .await
    }
}
