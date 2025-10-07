use std::borrow::Cow;

pub(crate) mod extension;
pub(crate) mod handler;
pub(crate) mod http;

// Re - export
pub use crate::codoraframeworksecurity::http::{IntoCfSecurityRequest, IntoCfSecurityResponse};
pub use extension::Extension;
pub use handler::{Handler, SignInHandler, SignOutHandler};

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub enum Error {
    MissingHandler(Cow<'static, str>),
    HandlerError(http::Response),
}

#[derive(new)]
pub struct CFrameworkSecurity {
    extension: Extension,
}

impl CFrameworkSecurity {
    #[inline]
    pub fn extension(&self) -> &Extension {
        &self.extension
    }

    #[inline]
    pub fn extension_mut(&mut self) -> &mut Extension {
        &mut self.extension
    }

    pub fn register_handler<H>(self, handler: H) -> Self
    where
        H: Clone + Sync + 'static,
    {
        drop(handler);
        todo!()
    }

    pub fn with<Request>(&'_ mut self, request: Request) -> CFrameworkSecurityWithRequest<'_, Request>
    where
        Request: IntoCfSecurityRequest,
    {
        CFrameworkSecurityWithRequest::new(request, &mut self.extension)
    }
}

#[derive(Debug, new)]
pub struct CFrameworkSecurityWithRequest<'a, Request>
where
    Request: IntoCfSecurityRequest,
{
    request: Request,
    extension: &'a mut Extension,
}

impl Clone for CFrameworkSecurity {
    fn clone(&self) -> Self {
        Self {
            extension: self.extension.clone(),
        }
    }
}

impl std::fmt::Debug for CFrameworkSecurity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CFrameworkSecurity")
            .field("extension", &self.extension)
            .finish()
    }
}

impl Default for CFrameworkSecurity {
    fn default() -> Self {
        CFrameworkSecurity::new(Extension::default())
    }
}

const _: () = {
    use crate::handlers::cookie::{CookieHandler, CookiePayload, CookieResponse, CookieState};
    use handler::ext::{HandlerExt, SignInHandlerExt, SignOutHandlerExt};

    impl<'a, State, Request> HandlerExt<State> for CFrameworkSecurityWithRequest<'a, Request>
    where
        Request: IntoCfSecurityRequest + Send + Sync,
        State: Into<CookieState> + Send + Sync,
    {
        type Response = CookieResponse;

        async fn authenticate(self, state: State) -> Result<Self::Response, Error> {
            self.extension
                .get::<CookieHandler>()
                .ok_or(Error::MissingHandler("Cookie".into()))?
                .authenticate(self.request, state)
                .await
                .map_err(|error| Error::HandlerError(error.into_cf_security_response()))
        }

        async fn challenge(self, state: State) -> Result<Self::Response, Error> {
            self.extension
                .get::<CookieHandler>()
                .ok_or(Error::MissingHandler("Cookie".into()))?
                .challenge(self.request, state)
                .await
                .map_err(|error| Error::HandlerError(error.into_cf_security_response()))
        }

        async fn forbid(self, state: State) -> Result<Self::Response, Error> {
            self.extension
                .get::<CookieHandler>()
                .ok_or(Error::MissingHandler("Cookie".into()))?
                .forbid(self.request, state)
                .await
                .map_err(|error| Error::HandlerError(error.into_cf_security_response()))
        }
    }

    impl<'a, Request, State> SignOutHandlerExt<State> for CFrameworkSecurityWithRequest<'a, Request>
    where
        Request: IntoCfSecurityRequest + Send + Sync,
        State: Into<CookieState> + Send + Sync,
    {
        async fn sign_out(self, state: State) -> Result<Self::Response, Error> {
            self.extension
                .get::<CookieHandler>()
                // Change this to type qualified syntax later
                .ok_or(Error::MissingHandler("Cookie".into()))?
                .sign_out(self.request, state)
                .await
                .map_err(|error| Error::HandlerError(error.into_cf_security_response()))
        }
    }

    impl<'a, Request, State, Payload> SignInHandlerExt<State, Payload> for CFrameworkSecurityWithRequest<'a, Request>
    where
        Request: IntoCfSecurityRequest + Send + Sync,
        State: Into<CookieState> + Send + Sync,
        Payload: Into<CookiePayload> + Send + Sync,
    {
        async fn sign_in(self, state: State, payload: Payload) -> Result<Self::Response, Error> {
            self.extension
                .get::<CookieHandler>()
                // Change this to type qualified syntax later
                .ok_or(Error::MissingHandler("Cookie".into()))?
                .sign_in(self.request, state, payload)
                .await
                .map_err(|error| Error::HandlerError(error.into_cf_security_response()))
        }
    }
};

// impl<'a, Request> CFrameworkSecurityWithRequest<'a, Request>
// where
//     Request: IntoCfSecurityRequest,
// {
//     pub async fn authenticate<H, State>(self, state: State) -> Result<H::Response, Error>
//     where
//         H: Handler<Request, State> + Clone + Sync + 'static,
//         State: Send + Sync,
//     {
//         self.extension
//             .get::<H>()
//             .ok_or(Error::MissingHandler(H::NAME.into()))?
//             .authenticate(self.request, state)
//             .await
//             .map_err(|error| Error::HandlerError(error.into_cf_security_response()))
//     }

//     pub async fn challenge<H, State>(self, state: State) -> Result<H::Response, Error>
//     where
//         H: Handler<Request, State> + Clone + Sync + 'static,
//         State: Send + Sync,
//     {
//         self.extension
//             .get::<H>()
//             .ok_or(Error::MissingHandler(H::NAME.into()))?
//             .challenge(self.request, state)
//             .await
//             .map_err(|error| Error::HandlerError(error.into_cf_security_response()))
//     }

//     pub async fn forbid<H, State>(self, state: State) -> Result<H::Response, Error>
//     where
//         H: Handler<Request, State> + Clone + Sync + 'static,
//         State: Send + Sync,
//     {
//         self.extension
//             .get::<H>()
//             .ok_or(Error::MissingHandler(H::NAME.into()))?
//             .forbid(self.request, state)
//             .await
//             .map_err(|error| Error::HandlerError(error.into_cf_security_response()))
//     }

//     pub async fn sign_out<H, State>(self, state: State) -> Result<H::Response, Error>
//     where
//         H: SignOutHandler<Request, State> + Clone + Sync + 'static,
//         State: Send + Sync,
//     {
//         self.extension
//             .get::<H>()
//             .ok_or(Error::MissingHandler(H::NAME.into()))?
//             .sign_out(self.request, state)
//             .await
//             .map_err(|error| Error::HandlerError(error.into_cf_security_response()))
//     }

//     pub async fn sign_in<H, State, Payload>(self, state: State, payload: Payload) -> Result<H::Response, Error>
//     where
//         H: SignInHandler<Request, State, Payload> + Clone + Sync + 'static,
//         State: Send + Sync,
//         Payload: Send + Sync,
//     {
//         self.extension
//             .get::<H>()
//             .ok_or(Error::MissingHandler(H::NAME.into()))?
//             .sign_in(self.request, state, payload)
//             .await
//             .map_err(|error| Error::HandlerError(error.into_cf_security_response()))
//     }
// }
