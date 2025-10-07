#![allow(dead_code, unused_variables, unused_imports)]

use crate::valarbot::valar_bot_telegram_command;
use proc_macro::TokenStream;

mod valar;

#[proc_macro_derive(valar, attributes(valar))]
pub fn derive_valar(token_stream: TokenStream) -> TokenStream {
    valar::derive(token_stream)
}

#[cfg(feature = "valar-bot-telegram")]
mod valarbot;

#[cfg(feature = "valar-bot-telegram")]
#[proc_macro_derive(Command)]
pub fn valar_bot_command(token_stream: TokenStream) -> TokenStream {
    valar_bot_telegram_command(token_stream)
}
