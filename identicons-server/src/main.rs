//! A server that serves up identicons.

#![deny(missing_docs)]

extern crate actix_web;
extern crate ctrlc;
extern crate identicons;
extern crate identicons_server;
extern crate listenfd;
extern crate rand;
extern crate serde;
extern crate serde_json;
#[macro_use] extern crate  serde_derive;
extern crate tera;

use actix_web::{App, HttpRequest, HttpResponse, Path};
use listenfd::ListenFd;
use std::{env, process};
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use tera::Context;
use rand::{Rng, SeedableRng};

use identicons::{ShapeIconData, ShieldIconData};
use identicons_server::templ;

fn main() {
    // Rust doesn't have a ctrl-c handler itself, so when running as
    // PID 1 in Docker it doesn't respond to SIGINT. This prevents
    // ctrl-c from stopping a docker container running this
    // program. Handle SIGINT (aka ctrl-c) to fix this problem.
    ctrlc::set_handler(move || {
        process::exit(0);
    }).expect("error setting ctrl-c handler");

    let server = actix_web::server::new(|| make_app());

    // Re-use a passed file descriptor, or create a new one to listen on.
    let mut listenfd = ListenFd::from_env();
    let server = if let Some(listener) = listenfd.take_tcp_listener(0).unwrap() {
        server.listen(listener)
    } else {
        let host = env::var("HOST").unwrap_or("127.0.0.1".to_string());
        let port = env::var("PORT").unwrap_or("8080".to_string());
        let addr = format!("{}:{}", host, port);
        server.bind(&addr).expect(&format!("Couldn't listen on {}", &addr))
    };

    server.run();
}

fn make_app() -> App {
    App::new()
        .resource("/", |r| r.get().f(index))
        .resource("/i/shield/v1/{query}", |r| r.get().with(shield_generator))
        .resource("/i/shape/v0/{query}", |r| r.get().with(shape_generator))
}

fn index(_: HttpRequest) -> impl actix_web::Responder {
    let context = Context::new();
    let content = templ::render("index.html.tera", &context).unwrap();

    HttpResponse::Ok()
        .content_type("text/html")
        .body(content)
}


fn parse_query(query: &String) -> ([u8; 16], String) {
    let (seed, format) = if query.contains(".") {
        let mut parts: Vec<&str> = query.splitn(2, ".").collect();
        let format = parts.pop().unwrap().to_string();
        let seed = parts.pop().unwrap().to_string();
        (seed, format)
    } else {
        (query.to_string(), "svg".to_string())
    };

    let mut hasher = DefaultHasher::new();
    hasher.write(&seed.bytes().collect::<Vec<u8>>());
    let hash = hasher.finish();

    let mut seed_vec = Vec::with_capacity(16);
    for i in 0..8 {
        let offset = i * 8;
        let mask = 0xFF << offset;
        let byte = (hash & mask) >> offset;
        seed_vec.push(byte as u8);
    }
    seed_vec.resize(16, 0);
    let mut seed = [0u8; 16];
    seed.copy_from_slice(&seed_vec[..]);

    (seed, format)
}

#[derive(Debug, Deserialize)]
struct GeneratorInfo {
    query: String,
}

fn shield_generator(info: Path<GeneratorInfo>) -> impl actix_web::Responder {
    let (seed, format) = parse_query(&info.query);
    let mut rng = rand::XorShiftRng::from_seed(seed);
    let icon_data = rng.gen::<ShieldIconData>();

    match &format[..] {
        "svg" => {
            let content = icon_data.to_svg().unwrap();
            HttpResponse::Ok()
                .content_type("image/svg+xml")
                .body(content)
        }
        "json" => {
            let json = serde_json::to_string(&icon_data).unwrap(); // TODO better error handling
            HttpResponse::Ok()
                .content_type("application/json")
                .body(json)
        }
        _ => {
            HttpResponse::BadRequest()
                .content_type("text/plain")
                .body(format!("Unsupported format \"{}\"", format))
        }
    }
}

fn shape_generator(info: Path<GeneratorInfo>) -> impl actix_web::Responder {
    let (seed, format) = parse_query(&info.query);
    let mut rng = rand::XorShiftRng::from_seed(seed);
    let icon_data = rng.gen::<ShapeIconData>();

    match &format[..] {
        "svg" => {
            let content = icon_data.to_svg().unwrap();
            HttpResponse::Ok()
                .content_type("image/svg+xml")
                .body(content)
        }
        "json" => {
            let json = serde_json::to_string(&icon_data).unwrap(); // TODO better error handling
            HttpResponse::Ok()
                .content_type("application/json")
                .body(json)
        }
        _ => {
            HttpResponse::BadRequest()
                .content_type("text/plain")
                .body(format!("Unsupported format \"{}\"", format))
        }
    }
}

/*
/// Make the icon server.
pub fn make_icon_server() -> Iron<Chain> {
    let mut router = Router::new();
    router.get("/", index, "index");
    router.get("/i/shield/v1/:query", shield_generator, "shield_1");
    router.get("/i/shape/v0/:query", shape_generator, "shape_0");

    let mut chain = Chain::new(router);
    chain.link_after(ErrorHandler);
    Iron::new(chain)
}

struct ErrorHandler;

impl AfterMiddleware for ErrorHandler {
    fn after(&self, _: &mut Request, resp: Response) -> IronResult<Response> {
        Ok(resp)
    }

    fn catch(&self, _: &mut Request, err: IronError) -> IronResult<Response> {
        let mut resp = Response::new();
        resp.set_mut(status::InternalServerError);
        resp.set_mut(format!("{:?}", err));
        Ok(resp)
    }
}




*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::default::Default;
    use actix_web::{http, test};

    #[test]
    fn test_index() {
        let req = test::TestRequest::default();
        let res = req.run(index).unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);
    }
}