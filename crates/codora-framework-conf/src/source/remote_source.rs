use crate::source::Source;

// #[derive(Debug)]
pub struct RemoteSource<S> {
    /// Source to fetch from
    source: S,
}

impl<S> RemoteSource<S> {
    pub fn new(source: S) -> Self {
        Self { source }
    }
}

impl<S> Source for RemoteSource<S> {}

#[cfg(test)]
mod tests {}
