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

use super::icons::{Color, ShieldIconData};

pub fn make_icon_server() -> Iron<Chain> {
    let mut router = Router::new();
    router.get("/", index, "index");
    router.get("/i/shield/v1/:query", icon_generator, "shield");

    let mut chain = Chain::new(router);

    let mut teng = TeraEngine::new("templates/**/*");
    teng.tera.register_filter("css", tera_to_css);
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

fn tera_to_css(value: tera::Value, _args: HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let debug_copy = value.clone();
    if let Ok(color) = tera::from_value::<Color>(value) {
        Ok(tera::Value::String(color.css_color()))
    } else {
        Err(tera::Error::from_kind(tera::ErrorKind::Msg(format!("css is not implemented for {:?}", debug_copy))))
    }
}

fn index(_: &mut Request) -> Result<Response, IronError> {
    let context = Context::new();
    let template = Template::new("index.html.tmpl", TemplateMode::from_context(context));
    let mut resp = Response::new();
    resp.set_mut((status::Ok, template));
    Ok(resp)
}

fn icon_generator(req: &mut Request) -> Result<Response, IronError> {
    let router = req.extensions.get::<Router>().unwrap(); // TODO better error handling
    let ref query = router.find("query").unwrap(); // TODO better error handling

    let (seed, ext) = if query.contains(".") {
        let mut parts: Vec<&str> = query.splitn(2, ".").collect();
        let ext = parts.pop().unwrap().to_string();
        let seed = parts.pop().unwrap().to_string();
        (seed, ext)
    } else {
        (query.to_string(), "svg".to_string())
    };

    let mut hasher = DefaultHasher::new();
    hasher.write(&seed.bytes().collect::<Vec<u8>>());
    let hash = hasher.finish();

    let high = ((hash & 0xFFFF_FFFF_0000_0000) >> 32) as u32;
    let low = (hash & 0x0000_0000_FFFF_FFFF) as u32;
    let seed = [high, low, 0, 0];
    let mut rng = rand::XorShiftRng::from_seed(seed);

    let icon_data = rng.gen::<ShieldIconData>();

    match &ext[..] {
        "svg" => {
            let mut context = Context::new();
            context.add("icon", &icon_data);

            let template = Template::new("shield.svg.tmpl", TemplateMode::from_context(context));

            let mut resp = Response::new();
            let svg_type: mime::Mime = "image/svg+xml;charset=utf-8".parse().unwrap();
            resp.headers.set(headers::ContentType(svg_type));
            resp.set_mut((status::Ok, template));
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
            resp.set_mut((status::BadRequest, format!("Unsupported format \"{}\"", ext)));
            Ok(resp)
        }
    }
}
