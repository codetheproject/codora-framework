#![forbid(unsafe_code)]
// Silence the noise in development!
#![cfg_attr(debug_assertions, allow(dead_code, unused_variables, warnings))]
// Docs and linting rules
#![cfg_attr(docsrs, feature(doc_auto_cfg, doc_cfg))]
#![cfg_attr(test, allow(clippy::float_cmp))]
#![cfg_attr(not(test), deny(clippy::print_stdout, clippy::dbg_macro))]
// - Lint for missing docs
#![cfg_attr(not(debug_assertions), deny(missing_docs))]

use crate::telegram_types::GetMe;
use std::{
    pin::Pin,
    task::{Context, Poll},
};
use tower::Service;

#[macro_use]
extern crate derive_new;
#[macro_use]
extern crate tracing;

/*
    async fn get_me(ctx: Content) -> GetMe {
        ctx.get_me().await
    }
*/
#[derive(Debug, thiserror::Error)]
#[error("{0}")]
pub enum Error {}

pub type Result<T, E = Error> = core::result::Result<T, E>;

trait BotApi {
    type Option: Default;
    type Connected;

    fn connected(&self) -> Result<Self::Connected>;

    // Provide other function
}

/// Bot is generic over the intended api just like sqlx
///         --- discord
///  ----   --- telegram   ---> unified api to talk to outside world
///         ---     x
///
/// Bot is service and expected to run forever
///
/// ```no_run
///     let bot_instance = Bot::<Telegram>::new(|option| {
///           option.url = "https://web.api.telegram";
///           // other configs
///         option
///     });
///
///     bot_instance
///         .setup_listener(3000)
///         .with_graceful_shutdown()
///         .await?;
/// ```
/// or be plugges as service example  assuming we using axum
///
/// ```no_run
///     use axum::Router;
///
///     let bot_instance = Bot::<Telegram>::new(|option| {
///           option.url = "https://web.api.telegram";
///           // other configs
///         option
///     });
///
///     let app = Router::new().route("/webhook", post(bot_instance));
///
///     axum::serve(listener, app).await?;
/// ```
#[derive(Debug, new)]
pub struct Bot<A> {
    bot_api: A,
}

#[derive(Debug, Clone, new)]
pub struct Telegram {
    inner: TelegramOption,
}

#[derive(Debug, Clone, Default)]
pub struct TelegramOption {}

mod telegram_types {
    #[derive(Debug, Clone)]
    pub struct Updates {}

    #[derive(Debug, Clone)]
    pub struct GetMe {}
}

impl BotApi for Telegram {
    type Option = TelegramOption;

    // assuming this is what telegram gives back when connected let hope
    type Connected = telegram_types::GetMe;

    fn connected(&self) -> Result<Self::Connected> {
        Ok(GetMe {})
    }
}

// This pattern don't work well if Self needs more than option look into it
impl From<TelegramOption> for Telegram {
    fn from(value: TelegramOption) -> Self {
        Self::new(value)
    }
}

impl Bot<Telegram> {
    fn on<H>(self, arg: &str, handler: H) -> Result<Self> {
        // the idea we wanna register arg here with the handler to look it upm later

        Ok(self)
    }
}

/// This is expected to be generic all  bot api to work with this let proceed
pub trait Handler {}

pub fn handler<H>(handler: H)
where
    H: Handler,
{
    //  we gonna drop it for now but you get the idea we wanna work with handler here
    drop(handler)
}

impl<F> Handler for F where F: FnOnce() {}

impl<A> Service<telegram_types::Updates> for Bot<A> {
    type Response = ();
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        Poll::Ready(Ok(()))
    }

    fn call(&mut self, req: telegram_types::Updates) -> Self::Future {
        Box::pin(async move {
            // when update is recieved we wanna call self
            trace!("Recieved update: {:?}", req);
            Ok(())
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{Bot, Telegram, TelegramOption, handler, telegram_types::Updates};
    use anyhow::Result;
    use tower::ServiceExt as _;
    use tracing_subscriber::EnvFilter;

    #[tokio::test]
    async fn test_bot_with_long_polling_or_webhook() -> anyhow::Result<()> {
        tracing_subscriber::fmt()
            .with_env_filter(EnvFilter::new("trace"))
            .init();

        trace!("Subscriber installed");

        // assuming this is a async function with extractor in it you get the idea
        fn start_handler() {
            // let me = ctx.request::<GetMe>().await?;
            // let message = text!(chat_id, format!("Hello, I am {}", me.username));

            // the reply function would definently be provided via extension
            // let result = ctx.reply(message).await?:
            // inside send function
            // send {
            // convert it into
            // let request = message.into();

            // we didn't block here
            // let res = tokio::task::spawn(request).await;
            // return the res
            // res
        }
        let bot: Bot<Telegram> = Bot::new(Telegram::new(TelegramOption {})).on("/start", handler(start_handler))?;

        // This is how we wanna test our bot
        let res = bot.oneshot(Updates {}).await?;
        assert_eq!(res, ());

        // What if we wanna serve it we are thinking Bot could be injected as service like in axum as a service
        //  or

        // as teloxide dispatching method or long poll we will explore both
        Ok(())
    }
}
