extern crate cdrs;
#[macro_use]
extern crate cdrs_helpers_derive;
extern crate actix_web;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

pub mod network;

pub mod set_up {
   pub mod keyspace;
   pub mod table;
}

pub mod crud_operation {
   pub mod insert;
   pub mod remove;
   pub mod update;
   pub mod display;
}

pub mod models;