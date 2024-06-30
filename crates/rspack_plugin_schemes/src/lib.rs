#![feature(let_chains)]

mod data_uri;
mod file_uri;

mod http_uri;

pub use data_uri::DataUriPlugin;
pub use file_uri::FileUriPlugin;
