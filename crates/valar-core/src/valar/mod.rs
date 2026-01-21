use crate::http::body::Body;
use crate::http::{IntoRequest, Response};
use crate::http::{IntoResponse, Request};
use crate::metrics::Metrics;
use crate::valar::builder::Builder as ValarBuilder;
use crate::valar::context::{AuthenticationContext, Context};
use crate::valar::handler::BoxCloneSyncHandler;
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
pub mod service;

/// Valar error, these are error possible to happened in valar crates
#[derive(Debug, Clone, thiserror::Error)]
#[error("{0}")]
pub enum Error {
    #[error("{0:?}")]
    Response(Response),
}

impl IntoResponse for Error {
    type Body = Body;

    fn into_response(self) -> Response {
        match self {
            Error::Response(resp) => resp,
        }
    }
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

    // get handler from valar instance
    pub fn get_handler_ref<H>(&self) -> Option<&BoxCloneSyncHandler<Request<Body>, Response, Error>>
    where
        H: 'static,
    {
        self.inner
            .service_map
            .get(&TypeId::of::<H>())
    }

    pub fn get_handler<H>(&self) -> Option<BoxCloneSyncHandler<Request<Body>, Response, Error>>
    where
        H: 'static,
    {
        self.get_handler_ref::<H>().cloned()
    }

    // Write into metrics
    pub fn record_metrics(&self, metrics: ()) {
        todo!()
    }
}

impl<R> AuthenticationContext<R> for Valar
where
    R: IntoRequest,
{
    // This would allow users to authenticate as other's extension method would be provided
    fn auth(&self, request: R) -> Context<'_, R> {
        Context::new(self, request)
    }
}

impl Default for Valar {
    fn default() -> Self {
        todo!()
    }
}

pub struct Inner {
    service_map: HashMap<TypeId, BoxCloneSyncHandler<Request<Body>, Response, Error>, BuildHasherDefault<IdHasher>>,
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
