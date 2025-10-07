//! Telegram Module

mod data;
mod handler;
mod layers;
mod listener;

pub use crate::{
    adapter::telegram::handler::{Handler, handler_from_func},
    codoraframeworkbot::{bot::Bot, listener::Listener},
};
pub use layers::from_fn::from_fn;
pub use listener::TGWebhook;

#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub enum Error {}

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug)]
pub struct TGBotOption {}

#[derive(Debug)]
pub struct TGBot {
    option: TGBotOption,
}

impl TGBot {
    pub fn with(option: TGBotOption) -> Self {
        TGBot { option }
    }

    /// `message`, A new incoming message `(text, photo, document, etc.)`
    ///
    /// Would be called when a message or edited message is recieved
    pub fn on_message<H>(self, handler: H) -> Self
    where
        H: Handler,
    {
        self
    }

    pub fn layer<L>(self, layer: L) -> Self {
        self
    }
}

// Request would mostly be a http Request with Update as the Body
impl Bot<()> for TGBot {
    type Error = Error;

    async fn handle(&self, request: ()) -> Result<(), Self::Error> {
        todo!()
    }
}

// This would be the default listener for TGBot
impl Listener for TGBot {}

#[cfg(test)]
mod tests {
    use crate::adapter::telegram::handler_from_func;
    use crate::*;
    use crate::{
        adapter::telegram::{TGBot, TGBotOption, TGWebhook, from_fn},
        codoraframeworkbot::bot::context::BotContext,
    };

    #[tokio::test]
    async fn codora_framework_bot_test() -> anyhow::Result<()> {
        #[derive(Command)]
        pub enum CodoraCommand {
            Start,
        }

        // This is just a walkthrough we will change it where Result are custom perhaps through a trait
        async fn command_handler() -> Result<(), &'static str> {
            Ok(())
        }

        let listener = TGWebhook {};
        let bot = TGBot::with(TGBotOption {})
            //edit this later
            // we could take in handler
            // handler(|| async { Ok(()) }).intercept(|message| async { can_inspect_message_to_check(message) })
            .on_message(handler_from_func(command_handler))
            // This would be used to register Middleware before any handler is called
            .layer(from_fn(|| async {}));

        // Identical api with axum yes
        // let res = BotContext::builder(bot)
        //     // allow user to add dependencies  and setup how the
        //     .with_graceful_shutdown(async {})
        //     .build()
        //     .run_with_listener(listener)
        //     .await?;
        Ok(())
    }
}
