use crate::valar::{credential::Credential, handler::State};

// Complete this api later add test and reference to the main source code

#[derive(new)]
pub struct Request<B> {
    inner: http::request::Request<B>,
    credential: Option<Credential>,
    state: Option<Box<dyn State>>,
}

impl<B> Request<B> {
    pub fn unpack(self) -> (http::request::Request<B>, Option<Credential>, Option<Box<dyn State>>) {
        (self.inner, self.credential, self.state)
    }

    pub fn unpack_ref(&self) -> (&http::request::Request<B>, Option<&Credential>, Option<&dyn State>) {
        (&self.inner, self.credential.as_ref(), self.state.as_deref())
    }

    pub fn into_inner(self) -> http::request::Request<B> {
        self.inner
    }

    pub fn inner(&self) -> &http::request::Request<B> {
        &self.inner
    }

    pub fn resolve_state<T: 'static>(&self) -> Option<&T> {
        self.state
            .as_deref()
            .and_then(crate::resolve_state)
    }

    pub fn builder() -> Builder {
        Builder::new(http::request::Builder::new(), None, None)
    }
}

#[derive(new)]
pub struct Builder {
    builder: http::request::Builder,
    credential: Option<Credential>,
    state: Option<Box<dyn State>>,
}

impl Builder {
    pub fn credential(mut self, credential: Credential) -> Self {
        self.credential = Some(credential);
        self
    }

    pub fn state(mut self, state: Box<dyn State>) -> Self {
        self.state = Some(state);
        self
    }

    pub fn body<T>(self, body: T) -> http::Result<Request<T>> {
        Ok(Request::new(self.builder.body(body)?, None, None))
    }
}

impl std::ops::Deref for Builder {
    type Target = http::request::Builder;

    fn deref(&self) -> &Self::Target {
        &self.builder
    }
}

impl std::ops::DerefMut for Builder {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.builder
    }
}
