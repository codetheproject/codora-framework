use crate::http::Request;
use std::{convert::Infallible, pin::Pin};
use tower::{Layer, Service};

#[derive(Debug, Clone)]
pub enum Scope {
    SignIn,
    SignOut,
    Challenge,
    Authenticate,
    Forbid,
}

#[derive(new, Clone)]
pub struct ScopeLayer<L> {
    scope: Scope,
    scoped_layer: L,
}
#[derive(new, Clone)]
pub struct ScopeService<Scoped, S> {
    scope: Scope,
    scoped_service: Scoped,
    inner: S,
}

impl<Scoped, S> Layer<S> for ScopeLayer<Scoped>
where
    S: Service<Request> + Clone,
    Scoped: Clone + Layer<S>,
{
    type Service = ScopeService<Scoped::Service, S>;
    fn layer(&self, inner: S) -> Self::Service {
        let scoped_service = self
            .scoped_layer
            .clone()
            .layer(inner.clone());
        ScopeService::new(self.scope.clone(), scoped_service, inner)
    }
}
impl<Scoped, S> Service<Request> for ScopeService<Scoped, S> {
    type Response = Infallible;
    type Error = Infallible;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

    fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
        todo!()
    }
    fn call(&mut self, _req: Request) -> Self::Future {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_sign_in_layer() -> anyhow::Result<()> {
        let _sign_in_layer = ScopeLayer::new(Scope::SignIn, ());

        Ok(())
    }
}
