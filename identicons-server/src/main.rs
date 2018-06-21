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
#[macro_use]
extern crate serde_derive;
extern crate tera;

use actix_web::{App, HttpRequest, HttpResponse, Path};
use listenfd::ListenFd;
use std::{collections::hash_map::DefaultHasher, env, hash::Hasher, process};
use tera::Context;

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
        server
            .bind(&addr)
            .expect(&format!("Couldn't listen on {}", &addr))
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

    HttpResponse::Ok().content_type("text/html").body(content)
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

fn shield_generator(
    info: Path<GeneratorInfo>,
) -> Result<impl actix_web::Responder, GeneratorError> {
    let (seed, format) = parse_query(&info.query);
    let seed = String::from_utf8_lossy(&seed).into_owned();
    let icon_data = ShieldIconData::from_input(&seed[..])?;

    Ok(match &format[..] {
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
        _ => HttpResponse::BadRequest()
            .content_type("text/plain")
            .body(format!("Unsupported format \"{}\"", format)),
    })
}

fn shape_generator(info: Path<GeneratorInfo>) -> Result<impl actix_web::Responder, GeneratorError> {
    let (seed, format) = parse_query(&info.query);
    let seed = String::from_utf8_lossy(&seed).into_owned();
    let icon_data = ShapeIconData::from_input(&seed[..])?;

    Ok(match &format[..] {
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
        _ => HttpResponse::BadRequest()
            .content_type("text/plain")
            .body(format!("Unsupported format \"{}\"", format)),
    })
}

#[derive(Debug)]
struct GeneratorError;

impl actix_web::error::ResponseError for GeneratorError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::InternalServerError()
            .content_type("text/plain")
            .body(format!("{}", self))
    }
}

impl std::error::Error for GeneratorError {
    fn description(&self) -> &str {
        "There was an error generating the image"
    }
}

impl std::fmt::Display for GeneratorError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", (self as &std::error::Error).description())?;
        Ok(())
    }
}

impl From<()> for GeneratorError {
    fn from(_: ()) -> Self {
        GeneratorError
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{http, test};
    use std::default::Default;

    #[test]
    fn test_index() {
        let req = test::TestRequest::default();
        let res = req.run(index).unwrap();
        assert_eq!(res.status(), http::StatusCode::OK);
    }
}
