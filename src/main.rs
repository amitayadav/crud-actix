extern crate actix_web;
extern crate listenfd;
extern crate crud_actix;

use actix_web::{App, http, server};
use index::*;
use listenfd::ListenFd;

pub mod index;

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            .resource("/add", |r| r.method(http::Method::POST)
                .with(insert))
            .resource("/show/{roll_no}", |r| r.method(http::Method::GET)
                .with(show))
            .resource("/delete/{roll_no}", |r| r
                .method(http::Method::DELETE).with(delete))
            .resource("/update/{roll_no}", |r| r.method(http::Method::PUT)
                .with(update))
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:8080").unwrap()
    };

    server.run();
}
