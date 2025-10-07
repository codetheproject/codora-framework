pub mod context;

/// Bot contract
///
///
///
/// ### Examples
///
pub trait Bot<Request> {
    type Error;

    // Request could be Updates for telegram and Something else for Discord and Other's
    fn handle(&self, request: Request) -> impl Future<Output = Result<(), Self::Error>>;
}

// Request would be Telegram or Discord Updates
// impl<T> Bot<T> where T: BotHandler {}
