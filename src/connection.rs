use protos::{self, sc2api};
use protobuf::Message;
use websocket::ClientBuilder;
use websocket::OwnedMessage;

const CONNECTION: &'static str = "ws://127.0.0.1:5000/sc2api";

use futures::future::Future;
use futures::sink::Sink;
use futures::stream::Stream;
use futures::sync::mpsc::{self, Sender, Receiver};

pub struct Connection {
    tx: Box<Sink<SinkItem=sc2api::Request, SinkError=()>>,
    rx: Box<Stream<Item=sc2api::Response, Error=()>>
}

use tokio::prelude::*;

pub fn connect () -> impl Future<Item=Connection, Error=websocket::WebSocketError> {
    ClientBuilder::new(CONNECTION)
        .unwrap()
        .add_protocol("rust-websocket")
        .async_connect_insecure()
        .map(|(duplex, _)| {

            let (sink, stream) = duplex.split();
            let stream = stream.map_err(|_| ()).map(|message| {
                match message {
                    OwnedMessage::Binary(v) => {
                        protobuf::parse_from_bytes::<sc2api::Response>(v.as_ref()).unwrap()
                    },
                    _ => sc2api::Response::new(),
                }
            });

            let sink = sink.sink_map_err(|_|()).with(|m: sc2api::Request| {
                let mut buf = Vec::new();
                m.write_to_vec(&mut buf);
                future::finished(OwnedMessage::Binary(buf))
            });

            Connection { tx: Box::new(sink), rx: Box::new(stream) }
        })
}
