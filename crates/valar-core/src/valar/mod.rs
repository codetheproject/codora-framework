use tower::util::BoxCloneSyncService;

use crate::http::body::Body;
use crate::http::{IntoRequest, Response};
use crate::metrics::Metrics;
use crate::valar::builder::Builder as ValarBuilder;
use crate::valar::context::Context;
use crate::valar::handler::SignOutHandler;
use crate::valar::map::Map;
use crate::valar::request::Request;
use std::any::TypeId;
use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::{BuildHasherDefault, Hasher};
use std::sync::Arc;

pub mod builder;
pub mod context;
pub mod credential;
pub mod handler;
pub mod map;
pub mod request;
pub mod scope;

/// Valar error, these are error possible to happened in valar crates
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub enum Error {
    #[error("{0:?}")]
    Response(Response),
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Clone)]
pub struct Valar {
    // This hold service ranging from Handler<Cookie>, Authorization,
    inner: Arc<Inner>,
}

impl Valar {
    // This is the recommened way to create a new valar instance
    // This would be the builder pattern
    // Put bound on H so handler would be restricted to handler Service alone
    pub fn with<H>(handler: H) -> ValarBuilder<(H,)>
    where
        H: Clone,
    {
        ValarBuilder::new((handler,))
    }

    pub fn ctx<'a, Request>(&'a self, request: Request) -> Context<'a, Request>
    where
        Request: IntoRequest + Debug,
    {
        Context::new(self, request)
    }

    // get handler from valar instance
    pub fn get_handler<H>(&self) -> Option<BoxCloneSyncService<Request<Body>, Response, Error>>
    where
        H: Clone + Send + Sync + 'static,
    {
        self.inner
            .service_map
            .get(&TypeId::of::<H>())
            .cloned()
    }

    // Write into metrics
    pub fn record_metrics(&self, metrics: ()) {
        todo!()
    }
}

impl Default for Valar {
    fn default() -> Self {
        todo!()
    }
}

pub struct Inner {
    service_map: HashMap<TypeId, BoxCloneSyncService<Request<Body>, Response, Error>, BuildHasherDefault<IdHasher>>,
    metrics: Metrics,
}

impl std::fmt::Debug for Inner {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Valar")
            .field("service_map", &self.service_map)
            .field("metrics", &self.metrics)
            .finish()
    }
}

impl std::fmt::Debug for Valar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

#[derive(Default)]
struct IdHasher(u64);

impl Hasher for IdHasher {
    fn write(&mut self, _: &[u8]) {
        unreachable!("TypeId calls write_u64");
    }

    #[inline]
    fn write_u64(&mut self, id: u64) {
        self.0 = id;
    }

    #[inline]
    fn finish(&self) -> u64 {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{util::NoopService, valar::scope::Scope};
    use request::Request;
    use std::convert::Infallible;

    #[derive(Debug, Clone)]
    struct FooLayer;

    #[derive(new, Clone)]
    struct FooService<S> {
        inner: S,
    }

    impl<S> tower::Service<Request<Body>> for FooService<S> {
        type Response = Infallible;
        type Error = Infallible;
        type Future = std::pin::Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>> + Send + 'static>>;

        fn poll_ready(&mut self, cx: &mut std::task::Context<'_>) -> std::task::Poll<Result<(), Self::Error>> {
            todo!()
        }

        fn call(&mut self, req: Request<Body>) -> Self::Future {
            todo!()
        }
    }

    impl<S> tower::Layer<S> for FooLayer
    where
        S: Clone,
    {
        type Service = FooService<S>;

        fn layer(&self, inner: S) -> Self::Service {
            FooService::new(inner.clone())
        }
    }

    #[test]
    fn test_valar() -> anyhow::Result<()> {
        let valar = Valar::with(NoopService)
            .handler(NoopService)
            .layer(FooLayer)
            .handler(NoopService)
            .scope_layer(Scope::SignIn, FooLayer)
            .init();

        let request = Request::builder()
            .uri("/")
            .method("GET")
            .body(());

        // let response = valar
        //     .ctx(request)
        //     .authenticate::<()>(state)
        //     .await?;

        Ok(())
    }
}
