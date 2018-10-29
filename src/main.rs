extern crate actix_web;
#[macro_use]
extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
extern crate listenfd;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use actix_web::{App, http, server};
use cdrs::frame::IntoBytes;
use cdrs::frame::TryFromRow;
use cdrs::query::*;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;
use cdrs::types::prelude::Row;
use index::*;
use listenfd::ListenFd;

pub mod connection;
pub mod db;
pub mod index;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, IntoCDRSValue, TryFromRow)]
pub struct Student {
    pub roll_no: i32,
    pub name: String,
    pub marks: i32,
}

impl Student {
    pub fn into_query_values(self) -> QueryValues {
        query_values!( self.roll_no, self.marks, self.name)
    }
}

fn main() {
    let mut listenfd = ListenFd::from_env();
    let mut server = server::new(|| {
        App::new()
            .resource("/add", |r| r.method(http::Method::POST).with(index1))
            .resource("/show/{roll_no}", |r| r.method(http::Method::GET).with(index2))
            .resource("/delete/{roll_no}", |r| r.method(http::Method::DELETE).with(index3))
            .resource("/update/{roll_no}", |r| r.method(http::Method::PUT).with(index4))
    });
    server = if let Some(l) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(l)
    } else {
        server.bind("127.0.0.1:8080").unwrap()
    };

    server.run();
}
