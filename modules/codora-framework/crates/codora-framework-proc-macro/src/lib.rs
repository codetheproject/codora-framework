//! This crates contain all the proc macro found in codora

use proc_macro::TokenStream;

#[cfg(feature = "conf")]
mod conf;

/// Derive macro for configuration parsing
///
/// TODO: extensive docs that cover all the features
#[cfg(feature = "conf")]
#[proc_macro_derive(Conf, attributes(conf))]
pub fn conf(tk: TokenStream) -> TokenStream {
    conf::conf_parse_token_stream(tk)
}

/// This is only availble when the macros feature is enabled
///
/// ```no_run
/// #[codora::main]
/// fn main(type: Type) {
///
/// }
/// ```
///
/// Type is expected to implement a specific trait to work as user could do that given the context
///
/// assuming this is user type
/// struct Run;
///
/// impl FromLoader for Run {
///     fn load(&self) -> Self {
///         Intantiate self
///     }
/// }
///
/// // This is expected to work that way user can implement there own too
/// #[codora::main]
/// fn main(run: Run) {
///    // user can drop run here to free memory easy
///     drop(run)
/// }
#[proc_macro_attribute]
pub fn main(_tk: TokenStream, __tk: TokenStream) -> TokenStream {
    quote::quote! {
        // main to return whatever run return Result or unit
        fn main () -> () {
            // return whatever run return here
            // Context::build().run(|env: Env, conf: Conf| {
            //     println!("{} - {}", env, conf);
            // })

            // that's the  idea btw
        }
    }
    .into()
}
