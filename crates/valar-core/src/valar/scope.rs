use crate::{
    http::Request,
    valar::handler::{Handler, State},
};
use tower_layer::Layer;

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
    S: Handler<Request> + Clone,
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
impl<Scoped, S> Handler<Request> for ScopeService<Scoped, S>
where
    S: Handler<Request>,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = S::Future;

    fn authenticate(&self, request: Request, state: &dyn State) -> Self::Future {
        todo!()
    }

    fn forbid(&self, request: Request, state: &dyn State) -> Self::Future {
        todo!()
    }

    fn challenge(&self, request: Request, state: &dyn State) -> Self::Future {
        todo!()
    }

    fn sign_out(&self, request: Request, state: &dyn State) -> Self::Future {
        todo!()
    }

    fn sign_in(&self, request: Request, state: &dyn State, credential: super::credential::Credential) -> Self::Future {
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
