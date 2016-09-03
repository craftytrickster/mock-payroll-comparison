#![feature(custom_derive, plugin)]
#![plugin(serde_macros)]

extern crate iron;
extern crate router;

extern crate serde;
extern crate serde_json;

mod controller;
mod submission;

fn main() {
    controller::start();
}