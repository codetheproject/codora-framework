// This hold all the state we wanna share between our Context and it shold be easy to clone
pub struct Extension {}

impl Extension {
    pub fn get<T: Clone + Sync>(&self) -> Option<&T> {
        todo!()
    }
}

impl Clone for Extension {
    fn clone(&self) -> Self {
        Self {}
    }
}

impl Default for Extension {
    fn default() -> Self {
        Self {}
    }
}

impl std::fmt::Debug for Extension {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Extension").finish()
    }
}
