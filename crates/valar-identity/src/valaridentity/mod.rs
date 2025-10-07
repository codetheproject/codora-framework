use std::sync::Arc;

pub mod email;
pub mod user;

pub struct CFrameworkIdentity {
    // extension
    inner: Arc<CFrameworkIdentityInner>,
}

impl CFrameworkIdentity {
    // maanage cframework security
    // worker for background tasks perhaps
}

pub struct CFrameworkIdentityInner {}

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn test_cf_identiy() -> anyhow::Result<()> {
        Ok(())
    }
}
