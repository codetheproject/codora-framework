use proc_macro::TokenStream;
use quote::quote;

pub fn codora_framework_bot_telegram_command(token_stream: TokenStream) -> TokenStream {
    drop(token_stream);

    let _quote = quote! {
        //
    };
    TokenStream::from(_quote)
}
