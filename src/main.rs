
extern crate mio;
extern crate hyper;
extern crate hyper_native_tls;

mod web_api;

fn main() {
    web_api::get("rtm.start");
    println!("Hello, world!");
}
