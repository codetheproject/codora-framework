use crate::codoraframeworkbot::listener::Listener;

mod polling;
mod webhook;

// pub use polling::TGPolling;
pub use webhook::TGWebhook;
