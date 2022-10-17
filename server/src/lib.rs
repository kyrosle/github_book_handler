use lazy_static::lazy_static;

pub mod bookhandler;
pub mod configs;
pub mod service;
pub mod path;


lazy_static! {
    pub static ref SERVER_IP: &'static str = "127.0.0.1";
    pub static ref SERVER_PORT: &'static str = "8889";
}