extern crate ctrlc;
extern crate identicons;

use identicons::server::make_icon_server;

fn main() {
    // Rust doesn't have a ctrl-c handler itself, so when running as
    // PID 1 in Docker it doesn't respond to SIGINT. This prevents
    // ctrl-c from stopping a docker container running this
    // program. Handle SIGINT (aka ctrl-c) to fix this problem.
    ctrlc::set_handler(move || {
        ::std::process::exit(1);
    }).expect("error setting ctrl-c handler");

    let host = "0.0.0.0:8080";
    let server = make_icon_server();
    let _listening = server.http(host).expect("could not start server");
    println!("listening on http://{}", host);
}
