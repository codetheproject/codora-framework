#![allow(dead_code, unused_variables)]
use crate::bearer::{error::BearerError, option::BearerOption, payload::BearerPayload, response::BearerResponse, state::BearerState};
use std::sync::Arc;
use valar_core::{
    http::request::IntoRequest,
    security::handler::{Handler, SignInHandler, SignOutHandler},
};

// Cookie must be easy to clone we could use the inner scope
#[derive(Clone)]
pub struct BearerHandler {
    inner: Arc<InnerBearerHandler>,
}

impl BearerHandler {
    pub fn new(bearer_option: BearerOption) -> Self {
        let inner = Arc::new(InnerBearerHandler::new(bearer_option));
        Self { inner }
    }
}

#[derive(new)]
struct InnerBearerHandler {
    cookie_option: BearerOption,
}

impl<Request> Handler<Request> for BearerHandler
where
    Request: IntoRequest + Sync + Send,
{
    type Response = BearerResponse;
    type Error = BearerError;
    type State = BearerState;

    async fn authenticate(&self, request: Request, state: Self::State) -> Result<Self::Response, Self::Error> {
        // let request = request.into_cf_security_request();

        // implement authentication logic here

        Ok(BearerResponse::default())
    }

    async fn forbid(&self, request: Request, state: Self::State) -> Result<Self::Response, Self::Error> {
        // let request = request.into_cf_security_request();

        // implement forbid logic here

        Ok(BearerResponse::default())
    }

    async fn challenge(&self, request: Request, state: Self::State) -> Result<Self::Response, Self::Error> {
        // let request = request.into_cf_security_request();

        // implement challenge logic here

        Ok(BearerResponse::default())
    }
}

impl<Request> SignOutHandler<Request> for BearerHandler
where
    Request: IntoRequest + Send + Sync,
{
    async fn sign_out(&self, request: Request, _state: Self::State) -> Result<Self::Response, Self::Error> {
        // let request = request.into_cf_security_request();

        // implement sign out  logic here

        Ok(BearerResponse::default())
    }
}

impl<Request> SignInHandler<Request> for BearerHandler
where
    Request: IntoRequest + Send + Sync,
{
    type Payload = BearerPayload;

    async fn sign_in(&self, request: Request, state: Self::State, payload: Self::Payload) -> Result<Self::Response, Self::Error> {
        // let request = request.into_cf_security_request();

        // implement sign in  logic here

        Ok(BearerResponse::default())
    }
}
