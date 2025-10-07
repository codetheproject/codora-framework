mod enviroment;
mod multi_source;
mod remote_source;

pub use {enviroment::Enviroment, multi_source::MultiSource, remote_source::RemoteSource};

// Source could be one like Enviroment which read from enviroment

// define source in here
pub trait Source {}
