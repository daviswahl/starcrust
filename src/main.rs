#![feature(fn_traits)]
#![feature(unboxed_closures)]

#[macro_use]
extern crate futures;
extern crate websocket;
extern crate tokio;
extern crate protobuf;

mod engine;
mod connection;
mod unit;

use unit::{UnitFutureExt,UnitStreamExt};
use protobuf::Message;


use tokio::prelude::*;

use std::thread;
use protos::sc2api::LocalMap;
use futures::IntoFuture;
mod protos;

use protos::sc2api;
use std::sync::Arc;
use std::sync::Mutex;
use unit::unit;
use std::thread::JoinHandle;


fn main() {

    let conn = connection::connect();
    let engine = conn.map_err(unit).and_then(move |c| {
        let mut e = engine::new(c);
        e.run().map_err(unit)
    });

    tokio::run(engine.unit_err("engine run"));
}
