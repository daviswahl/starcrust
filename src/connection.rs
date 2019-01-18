use protobuf::Message;
use protos::{self, sc2api};
use websocket::ClientBuilder;
use websocket::OwnedMessage;

use std::sync::atomic::{Ordering, AtomicUsize};
use unit::{UnitFutureExt, UnitStreamExt};


use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::sync::mpsc::{self, Receiver, Sender};
use std::sync::Arc;
use tokio::prelude::*;

use protobuf::error::WireError;
use protobuf::ProtobufError;
use protos::sc2api::Response;
use std::collections::VecDeque;
use std::sync::Mutex;
use std::sync::RwLock;
use std::time::Duration;
use std::time::Instant;
use tokio::prelude::*;
use unit::unit;
use unit::unit_debug;


const CONNECTION: &'static str = "ws://127.0.0.1:5000/sc2api";
struct Conn {
    tx: Sender<sc2api::Request>,
    rx: Receiver<sc2api::Response>,

    sent_msgs: usize,
    recvd_msgs: usize,
}

pub trait Connection {
    fn send(&mut self, req: sc2api::Request);
    fn recv(&mut self) -> Option<sc2api::Response>;
    fn stats(&self);
}

impl Connection for Conn {
    fn send(&mut self, req: sc2api::Request) {
        if let Ok(ok) = self.tx.start_send(req) {
            self.sent_msgs += 1;
        }
    }

    fn recv(&mut self) -> Option<Response> {
        if let Ok(Async::Ready(Some(v))) = self.rx.poll() {
            self.recvd_msgs += 1;
            Some(v)
        } else {
            None
        }
    }

    fn stats(&self) {
        println!("messages sent: {}, received: {}", self.sent_msgs, self.recvd_msgs)
    }
}



fn parse_message(m: OwnedMessage) -> Option<sc2api::Response> {
    match m {
        OwnedMessage::Binary(v) => protobuf::parse_from_bytes(v.as_ref()).ok(),
        _ => None,
    }
}

pub fn connect() -> impl Future<Item = impl Connection, Error = websocket::WebSocketError> {
    let conn = ClientBuilder::new(CONNECTION)
        .unwrap()
        .add_protocol("rust-websocket")
        .async_connect_insecure();

    conn.map(|(duplex, _)| {

        let sent = Arc::new(AtomicUsize::new(1));
        let recvd = Arc::new(AtomicUsize::new(1));

        let (req_sink, resp_stream) = duplex.split();
        let (resp_tx, resp_rx) = mpsc::channel(1_024);
        let (req_tx, req_rx) = mpsc::channel(1_024);


        let recvd_cloned = recvd.clone();
        let resp_stream = resp_stream
            .unit_err("resp_stream")
            .inspect(move |_| {
                recvd_cloned.clone().fetch_add(1, Ordering::SeqCst);
            })
            .filter_map(parse_message)
            .forward(resp_tx.sink_map_err(unit));

        let sent_cloned = sent.clone();
        let req_rx = req_rx
            .map(move |req: sc2api::Request| {
                sent_cloned.clone().fetch_add(1, Ordering::SeqCst);
                let mut buf = vec![];
                req.write_to_vec(&mut buf);
                OwnedMessage::Binary(buf)
            })
            .forward(req_sink.sink_map_err(unit_debug("req_sink")));

        let req_tx_clone = req_tx.clone();
        let flush_timer = tokio::timer::Interval::new(Instant::now(), Duration::from_millis(10))
            .unit_err("flush timer")
            .for_each(move |_| req_tx_clone.clone().flush().unit().unit_err("flush error"));

        tokio::spawn(flush_timer);
        tokio::spawn(resp_stream.unit());
        tokio::spawn(req_rx.unit());

        Conn {
            tx: req_tx,
            rx: resp_rx,
            sent_msgs: 0,
            recvd_msgs: 0,
        }
    })
}
