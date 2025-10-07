mod request;
mod response;
pub use request::IntoCfSecurityRequest;
pub use response::{IntoCfSecurityResponse, Response};

#[cfg(test)]
mod tests {

    #[tokio::test]
    async fn check_http() -> anyhow::Result<()> {
        Ok(())
    }
}
