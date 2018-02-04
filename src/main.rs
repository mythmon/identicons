extern crate ctrlc;
extern crate identicons;

use std::{env, process};
use identicons::server::make_icon_server;

fn main() {
    // Rust doesn't have a ctrl-c handler itself, so when running as
    // PID 1 in Docker it doesn't respond to SIGINT. This prevents
    // ctrl-c from stopping a docker container running this
    // program. Handle SIGINT (aka ctrl-c) to fix this problem.
    ctrlc::set_handler(move || {
        process::exit(0);
    }).expect("error setting ctrl-c handler");

    let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or("8080".to_string());
    let addr = format!("{}:{}", host, port);

    let server = make_icon_server();
    let _listening = match server.http(&addr) {
        Ok(v) => v,
        Err(e) => {
            println!("Could not start server: {}", e);
            process::exit(2);
        }
    };

    println!("listening on http://{}", addr);
}
