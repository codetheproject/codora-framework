#[derive(Debug, Clone)]
pub struct Metrics(());

impl Metrics {
    pub fn new(_0: ()) -> Self {
        Self(_0)
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self(())
    }
}
