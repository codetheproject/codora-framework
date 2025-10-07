pub(crate) mod error;
pub(crate) mod handler;
pub(crate) mod option;
pub(crate) mod payload;
pub(crate) mod response;
pub(crate) mod state;

use valar_core::http::response::IntoResponse;

#[derive(Debug, Default)]
pub struct BearerResponse {}

impl IntoResponse for BearerResponse {
    type Body = ();

    fn into_response(&self) -> http::Response<Self::Body> {
        todo!()
    }
}

#[derive(Debug, Clone)]
pub struct BearerState {}

#[derive(Debug, Clone, new)]
pub struct BearerPayload {}

#[derive(Debug, Clone, new)]
pub struct BearerOption {}

impl Default for BearerOption {
    fn default() -> Self {
        Self {}
    }
}

use valar_core::http::response::IntoResponse;

#[derive(Debug)]
pub enum BearerError {
    MissingExtension,
}

impl IntoResponse for BearerError {
    type Body = ();

    fn into_response(&self) -> http::Response<Self::Body> {
        todo!()
    }
}
