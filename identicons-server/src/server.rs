//! A server that serves up identicons.

extern crate iron;
extern crate rand;
extern crate router;
extern crate iron_tera;
extern crate tera;
extern crate serde_json;

use iron::prelude::*;
use iron::{status, AfterMiddleware, headers, mime};
use iron_tera::{Template, TemplateMode, TeraEngine};
use router::Router;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use tera::Context;
use rand::{Rng, SeedableRng};

use identicons::{Color, ShieldIconData, ShapeIconData, ShapeType};

/// Make the icon server.
pub fn make_icon_server() -> Iron<Chain> {
    let mut router = Router::new();
    router.get("/", index, "index");
    router.get("/i/shield/v1/:query", shield_generator, "shield_1");
    router.get("/i/shape/v0/:query", shape_generator, "shape_0");

    let mut chain = Chain::new(router);

    let mut teng = TeraEngine::new("templates/**/*");
    chain.link_after(teng);
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

fn index(_: &mut Request) -> Result<Response, IronError> {
    let context = Context::new();
    let template = Template::new("index.html.tmpl", TemplateMode::from_context(context));
    let mut resp = Response::new();
    resp.set_mut((status::Ok, template));
    Ok(resp)
}

fn parse_query(query: &str) -> ([u32; 4], String) {
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

    let high = ((hash & 0xFFFF_FFFF_0000_0000) >> 32) as u32;
    let low = (hash & 0x0000_0000_FFFF_FFFF) as u32;
    let seed = [high, low, 0, 0];

    (seed, format)
}

fn shield_generator(req: &mut Request) -> Result<Response, IronError> {
    let router = req.extensions.get::<Router>().unwrap(); // TODO better error handling
    let ref query = router.find("query").unwrap(); // TODO better error handling
    let (seed, format) = parse_query(query);
    let mut rng = rand::XorShiftRng::from_seed(seed);
    let icon_data = rng.gen::<ShieldIconData>();

    match &format[..] {
        "svg" => {
            let mut resp = Response::new();
            let svg_type: mime::Mime = "image/svg+xml;charset=utf-8".parse().unwrap();
            resp.headers.set(headers::ContentType(svg_type));
            resp.set_mut((status::Ok, icon_data.to_svg()));
            Ok(resp)
        }
        "json" => {
            let mut resp = Response::new();
            let json_type: mime::Mime = "application/json;charset=utf-8".parse().unwrap();
            resp.headers.set(headers::ContentType(json_type));
            let json = serde_json::to_string(&icon_data).unwrap(); // TODO better error handling
            resp.set_mut((status::Ok, json));
            Ok(resp)
        }
        _ => {
            let mut resp = Response::new();
            resp.set_mut((status::BadRequest, format!("Unsupported format \"{}\"", format)));
            Ok(resp)
        }
    }
}

fn shape_generator(req: &mut Request) -> Result<Response, IronError> {
    let router = req.extensions.get::<Router>().unwrap(); // TODO better error handling
    let ref query = router.find("query").unwrap(); // TODO better error handling
    let (seed, format) = parse_query(query);
    let mut rng = rand::XorShiftRng::from_seed(seed);
    let icon_data = rng.gen::<ShapeIconData>();

    match &format[..] {
        "svg" => {
            let mut resp = Response::new();
            let svg_type: mime::Mime = "image/svg+xml;charset=utf-8".parse().unwrap();
            resp.headers.set(headers::ContentType(svg_type));
            resp.set_mut((status::Ok, icon_data.to_svg()));
            Ok(resp)
        }
        "json" => {
            let mut resp = Response::new();
            let json_type: mime::Mime = "application/json;charset=utf-8".parse().unwrap();
            resp.headers.set(headers::ContentType(json_type));
            let json = serde_json::to_string(&icon_data).unwrap(); // TODO better error handling
            resp.set_mut((status::Ok, json));
            Ok(resp)
        }
        _ => {
            let mut resp = Response::new();
            resp.set_mut((status::BadRequest, format!("Unsupported format \"{}\"", format)));
            Ok(resp)
        }
    }
}
