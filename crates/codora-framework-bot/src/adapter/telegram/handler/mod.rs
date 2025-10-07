pub trait Handler {
    type Error;

    fn handle(&self) -> impl Future<Output = Result<(), Self::Error>>;
}

#[derive(Debug)]
pub struct HandlerFromFunc<F> {
    handler: F,
    // interceptor and other's goes in here
}

pub fn handler_from_func<F, Fut>(func: F) -> HandlerFromFunc<F>
where
    F: FnOnce() -> Fut,
    // This is just a template fix later
    Fut: Future<Output = Result<(), &'static str>>,
{
    HandlerFromFunc { handler: func }
}

impl<T> Handler for HandlerFromFunc<T> {
    // Change this to valid error
    type Error = &'static str;

    async fn handle(&self) -> Result<(), Self::Error> {
        todo!()
    }
}
