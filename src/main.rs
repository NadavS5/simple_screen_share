#![allow(dead_code)]

use std::env;
pub mod client;
pub mod server;
use client::client;
// use server::server;


const WIDTH: u16 = 1000;

const HEIGHT: u16 = 500;

fn main() {

    let args: Vec<String> = env::args().collect();
    match args.get(1).map(String::as_str) {
        Some("client") => {
            client(args.get(2).unwrap()).map_err( |err| eprintln!("{}",err)).unwrap();
        }
        // Some("server") => {
        //     server().map_err( |err| eprintln!("{}",err)).unwrap();
        // }
         _ => {
            eprintln!("Usage: cargo run -- [server|client]");
        }
    }

}
