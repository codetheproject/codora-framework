//! # Cookie Authentication
//!
//! # Overview
//! - Cookie authentication is a method of authentication that involves using cookies to store user credentials.

use valar_core::http::IntoResponse;

pub mod handler;

#[derive(Debug, Clone)]
pub struct CookieState {}

#[derive(Debug, Default)]
pub struct CookieResponse {}

impl IntoResponse for CookieResponse {
    type Body = ();

    fn into_response(self) -> http::Response<Self::Body> {
        todo!()
    }
}

#[derive(Debug)]
pub enum CookieError {
    MissingExtension,
}

impl IntoResponse for CookieError {
    type Body = ();

    fn into_response(self) -> http::Response<Self::Body> {
        todo!()
    }
}
