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
use std::{env, process};
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
        .resource("/i/shield/v1/{seed}.{format}", |r| {
            r.get().with(shield_generator)
        })
        .resource("/i/shape/v0/{seed}.{format}", |r| {
            r.get().with(shape_generator)
        })
}

fn index(_: HttpRequest) -> impl actix_web::Responder {
    let context = Context::new();
    let content = templ::render("index.html.tera", &context).unwrap();

    HttpResponse::Ok().content_type("text/html").body(content)
}

#[derive(Debug, Deserialize)]
struct GeneratorInfo {
    seed: String,
    format: GeneratorFormat,
}

#[derive(Debug, Deserialize)]
enum GeneratorFormat {
    #[serde(rename = "svg")]
    Svg,
    #[serde(rename = "json")]
    Json,
}

fn shield_generator(info: Path<GeneratorInfo>) -> Result<HttpResponse, GeneratorError> {
    let icon_data = ShieldIconData::from_input(&info.seed[..])?;

    Ok(match info.format {
        GeneratorFormat::Svg => {
            let content = icon_data.to_svg()?;
            HttpResponse::Ok()
                .content_type("image/svg+xml")
                .body(content)
        }
        GeneratorFormat::Json => {
            let json = serde_json::to_string(&icon_data)?;
            HttpResponse::Ok()
                .content_type("application/json")
                .body(json)
        }
    })
}

fn shape_generator(info: Path<GeneratorInfo>) -> Result<impl actix_web::Responder, GeneratorError> {
    let icon_data = ShapeIconData::from_input(&info.seed[..])?;

    Ok(match info.format {
        GeneratorFormat::Svg => {
            let content = icon_data.to_svg().unwrap();
            HttpResponse::Ok()
                .content_type("image/svg+xml")
                .body(content)
        }
        GeneratorFormat::Json => {
            let json = serde_json::to_string(&icon_data).unwrap(); // TODO better error handling
            HttpResponse::Ok()
                .content_type("application/json")
                .body(json)
        }
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
        "There was an unknown error generating the image"
    }
}

impl std::fmt::Display for GeneratorError {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(fmt, "{}", (self as &std::error::Error).description())?;
        Ok(())
    }
}

macro_rules! from_for_generator_error {
    ($t:ty) => {
        impl From<$t> for GeneratorError {
            fn from(_: $t) -> Self {
                GeneratorError
            }
        }
    };
}

from_for_generator_error!(());
from_for_generator_error!(serde_json::Error);
from_for_generator_error!(tera::Error);

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{
        http::{Method, StatusCode}, test, HttpMessage,
    };
    use std::default::Default;

    #[test]
    fn test_index() {
        let req = test::TestRequest::default();
        let res = req.run(index).unwrap();
        assert_eq!(res.status(), StatusCode::OK);
    }

    #[test]
    fn test_routing() {
        let mut srv = test::TestServer::with_factory(make_app);

        let req = srv
            .client(Method::GET, "/i/shield/v1/test.svg")
            .finish()
            .unwrap();
        let res = srv.execute(req.send()).unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(res.headers().get("content-type").unwrap(), "image/svg+xml");

        let req = srv
            .client(Method::GET, "/i/shield/v1/test.json")
            .finish()
            .unwrap();
        let res = srv.execute(req.send()).unwrap();
        assert_eq!(res.status(), StatusCode::OK);
        assert_eq!(
            res.headers().get("content-type").unwrap(),
            "application/json"
        );
    }
}
