#[macro_use]
extern crate diesel;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate redis_async;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate validator_derive;

use crate::core::server::server;

mod core;
mod helper;
mod middleware;
mod module;
mod config;
mod schema;

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    server().await
}
