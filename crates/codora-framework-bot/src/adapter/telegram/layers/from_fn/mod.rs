pub fn from_fn<F, Future>(func: F)
where
    F: FnOnce() -> Future,
    Future: std::future::Future<Output = ()>,
{
    todo!()
}
