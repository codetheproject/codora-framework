use codora_framework_bot::{Bot, Telegram, TelegramOption};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let _bot = Bot::new(Telegram::new(TelegramOption {}));

    // able to configure bot
    // serve bot and handle request
    Ok(())
}
