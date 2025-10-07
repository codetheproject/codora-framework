use crate::codoraframeworkbot::{bot::Bot, listener::Listener};
use std::{
    pin::Pin,
    task::{Context, Poll},
};

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub enum Error {
    #[error("[Error]: Server error happened")]
    ServeError,
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct BotContext<T> {
    bot_handler: T,
    // extension
}

impl<T> BotContext<T> {
    pub fn builder(bot_handler: T) -> BotContextBuilder<T> {
        BotContextBuilder { bot_handler }
    }
}

// impl<T> BotContext<T>
// where
//     T: Listener,
// {
//     /// This will use default listener provided by `T` to overide this please use the `run_with_listener` available on [BotContext]
//     pub async fn run(self) -> Result<()> {
//         todo!()
//     }
// }

#[derive(Debug)]
pub struct BotContextBuilder<T> {
    bot_handler: T,
}

// impl<T> BotContextBuilder<T> {
//     pub fn with_graceful_shutdown<F>(self, shutdown_handler: F) -> BotContextBuilder<T>
//     where
//         F: Future<Output = ()> + Send + 'static,
//     {
//         todo!()
//     }

//     pub fn build(self) -> BotContext<T> {
//         let BotContextBuilder { bot_handler } = self;
//         BotContext { bot_handler }
//     }
// }
