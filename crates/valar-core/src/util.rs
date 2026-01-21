// use crate::http::{Response, body::Body};
// use crate::valar::Error;
// use std::{
//     future::{Ready, ready},
//     task::Poll,
// };

// #[derive(Debug, Clone)]
// pub struct NoopService;

// impl<Request> tower::Service<Request> for NoopService {
//     type Response = Response;
//     type Error = Error;
//     type Future = Ready<Result<Self::Response, Self::Error>>;

//     fn poll_ready(&mut self, _cx: &mut std::task::Context<'_>) -> Poll<Result<(), Self::Error>> {
//         Poll::Ready(Ok(()))
//     }

//     fn call(&mut self, _req: Request) -> Self::Future {
//         ready(Ok(Response::new(Body::new(()))))
//     }
// }
