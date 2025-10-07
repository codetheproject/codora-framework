use crate::codora_framework_bot::codora_framework_bot_telegram_command;
use proc_macro::TokenStream;

#[cfg(feature = "codora-framework-bot-telegram")]
mod codora_framework_bot;

#[cfg(feature = "codora-framework-bot-telegram")]
#[proc_macro_derive(Command)]
pub fn codora_framework_bot_command(token_stream: TokenStream) -> TokenStream {
    codora_framework_bot_telegram_command(token_stream)
}
