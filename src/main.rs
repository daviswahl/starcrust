extern crate futures;
extern crate websocket;
extern crate tokio;
extern crate protobuf;

mod engine;
mod connection;
use protobuf::Message;


use tokio::prelude::*;

use std::thread;
use protos::sc2api::LocalMap;
use futures::IntoFuture;
mod protos;

use protos::sc2api;


fn main() {
    let mut runtime = tokio::runtime::current_thread::Builder::new()
        .build()
        .unwrap();

    let conn = connection::connect();
    match runtime.block_on(conn) {
        Ok(c) => println!("got a connection"),
        Err(e) => println!("{:?}", e),
    }
}
