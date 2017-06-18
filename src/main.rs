extern crate mio;
extern crate hyper;
extern crate hyper_native_tls;

#[macro_use]
extern crate serde_json;

extern crate websocket;
extern crate futures;
extern crate tokio_core;

use tokio_core::reactor::Core;
use websocket::{ClientBuilder, OwnedMessage};
use futures::future::{Future, IntoFuture};
use futures::stream::Stream;
use futures::sink::Sink;
use serde_json::{Value};
use futures::sync::mpsc;

mod web_api;

fn main() {
    let uri = web_api::get("rtm.start");

    let mut core = Core::new().unwrap();

    println!("Connect to {:?}", uri);

    let (msg_sender, msg_receiver) = mpsc::channel::<OwnedMessage>(0);

    let runner = ClientBuilder::new(&uri)
        .unwrap()
        .async_connect_secure(None, &core.handle())
        .and_then(|(duplex, _)| {
            let (sink, stream) = duplex.split();
            stream.filter_map(move |message| {
                println!("Received Message: {:?}", message);
                match message {
                    OwnedMessage::Close(e) => Some(OwnedMessage::Close(e)),
                    OwnedMessage::Ping(d) => Some(OwnedMessage::Pong(d)),
                    OwnedMessage::Text(t) => dispatch_message(&t, msg_sender.clone()),
                    _ => None,
                }
            })
                .forward(sink)
        });

    core.run(runner).unwrap();
}

fn dispatch_message(msg: &String, mut sender: mpsc::Sender<OwnedMessage>) -> Option<OwnedMessage> {
    let val: Value = serde_json::from_str(&msg).unwrap();
    match val["type"].to_string().as_ref() {
        r#""message""# => {
            let json = OwnedMessage::Text(json!({
                "type": "message",
                "text": val["text"],
                "channel": val["channel"]}).to_string());
            sender.send(json);
            None
        },
        _ => None
    }
}
