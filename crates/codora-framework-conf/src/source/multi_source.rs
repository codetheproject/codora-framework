use crate::source::Source;

// This allow you to combine multiple source
// #[derive(Debug)]
pub struct MultiSource {
    /// Sources to fetch from
    ///
    sources: Vec<Box<dyn Source>>,
}

impl MultiSource {
    /// Add `Source`
    ///
    ///
    pub fn add_source<T>(&mut self, source: T) -> &mut Self
    where
        T: Source + 'static,
    {
        self.sources.push(Box::new(source));
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn multi_source_test() {
        // let sourse = MultiSource { sources: Vec::default() };
    }
}
