extern crate iron;
extern crate rand;
extern crate router;
extern crate iron_tera;
extern crate tera;
#[macro_use]
extern crate lazy_static;
extern crate ctrlc;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;

use iron::prelude::*;
use iron::{status, AfterMiddleware, headers, mime};
use iron_tera::{Template, TemplateMode, TeraEngine};
use rand::{Rng, SeedableRng};
use router::Router;
use std::collections::HashMap;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use tera::Context;

fn main() {
    // Rust doesn't have a ctrl-c handler itself, so when running as
    // PID 1 in Docker it doesn't respond to SIGINT. This prevents
    // ctrl-c from stopping a docker container running this
    // program. Handle SIGINT (aka ctrl-c) to fix this problem.
    ctrlc::set_handler(move || {
        ::std::process::exit(1);
    }).expect("error setting ctrl-c handler");

    let mut router = Router::new();
    router.get("/", index, "index");
    router.get("/i/shield/v1/:query", icon_generator, "shield");

    let mut chain = Chain::new(router);

    let mut teng = TeraEngine::new("templates/**/*");
    teng.tera.register_filter("css", tera_to_css);
    chain.link_after(teng);
    chain.link_after(ErrorHandler);

    let host = "0.0.0.0:8080";
    let server = Iron::new(chain);
    let _listening = server.http(host).expect("could not start server");
    println!("listening on http://{}", host);
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

trait RngExt {
    /// Choose a random item from a collection by weight.
    fn weighted_choice<T>(&mut self, choices: Vec<(T, usize)>) -> T;
}

impl<R: Rng> RngExt for R {
    fn weighted_choice<T>(&mut self, choices: Vec<(T, usize)>) -> T {
        let sum_weights = choices.iter().map(|c| c.1).sum();
        let mut choice = self.gen_range(0, sum_weights);
        for (item, weight) in choices.into_iter() {
            if choice < weight {
                return item;
            }
            choice -= weight;
        }
        unreachable!("No items chosen");
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    fn black() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    fn css_color(&self) -> String {
        format!("rgb({},{},{})", self.r, self.g, self.b)
    }

    fn luminance(&self) -> f32 {
        0.2126 * self.r as f32 + 0.7152 * self.g as f32 + 0.0722 * self.b as f32
    }

    fn contrasts_well(&self, other: &Self) -> bool {
        (self.luminance() - other.luminance()).abs() > 75.0
    }
}

lazy_static!(
    // Colors taken from the Solarized color scheme (http://ethanschoonover.com/solarized)
    static ref COLORS: Vec<Color> = vec![
        Color { r: 0x00, g: 0x2b, b: 0x36 },
        Color { r: 0x07, g: 0x36, b: 0x42 },
        Color { r: 0x58, g: 0x6e, b: 0x75 },
        Color { r: 0x65, g: 0x7b, b: 0x83 },
        Color { r: 0x83, g: 0x94, b: 0x96 },
        Color { r: 0x93, g: 0xa1, b: 0xa1 },
        Color { r: 0xee, g: 0xe8, b: 0xd5 },
        Color { r: 0xfd, g: 0xf6, b: 0xe3 },
        Color { r: 0xff, g: 0xcf, b: 0x00 },  // alternate yellow color, not the one from Solarized
        Color { r: 0xcb, g: 0x4b, b: 0x16 },
        Color { r: 0xdc, g: 0x32, b: 0x2f },
        Color { r: 0xd3, g: 0x36, b: 0x82 },
        Color { r: 0x6c, g: 0x71, b: 0xc4 },
        Color { r: 0x26, g: 0x8b, b: 0xd2 },
        Color { r: 0x2a, g: 0xa1, b: 0x98 },
        Color { r: 0x85, g: 0x99, b: 0x00 },
    ];

    static ref EMOJIS: Vec<char> = vec![
        '😄', '😃', '😀', '😊', '😉', '😍', '😘', '😚', '😗', '😙', '😜', '😝', '😛',
        '😳', '😁', '😔', '😌', '😒', '😞', '😣', '😢', '😂', '😭', '😪', '😥', '😰',
        '😅', '😓', '😨', '😱', '😠', '😡', '😤', '😖', '😆', '😋', '😷', '😎', '😴',
        '😵', '😲', '😟', '😦', '😧', '😈', '👿', '😮', '😬', '😐', '😯', '😶', '😇',
        '😏', '😑', '👼', '😺', '😻', '😽', '😼', '🙀', '😿', '😹', '😾', '👹', '👺',
        '🙈', '🙉', '🙊', '💀', '👽', '💩', '🔥', '✨', '🌟', '💫', '💥', '💦', '💧',
        '💤', '👂', '👀', '👃', '👅', '👄', '👍', '👎', '👌', '👊', '✊', '👋', '✋',
        '👐', '👆', '🙌', '🙏', '👏', '💪', '💃', '🎩', '👑', '👒', '👟', '👞', '👡',
        '👠', '👢', '💼', '👜', '👝', '👛', '👓', '🎀', '🌂', '💄', '💛', '💙', '💜',
        '💚', '💔', '💗', '💓', '💕', '💖', '💞', '💘', '💌', '💋', '💍', '💎', '👣',
        '🐶', '🐺', '🐱', '🐭', '🐹', '🐰', '🐸', '🐯', '🐨', '🐻', '🐷', '🐽', '🐮',
        '🐗', '🐵', '🐒', '🐴', '🐑', '🐘', '🐼', '🐧', '🐦', '🐤', '🐥', '🐣', '🐔',
        '🐍', '🐢', '🐛', '🐝', '🐜', '🐞', '🐌', '🐙', '🐚', '🐠', '🐟', '🐬', '🐳',
        '🐋', '🐄', '🐏', '🐀', '🐃', '🐅', '🐇', '🐉', '🐎', '🐐', '🐓', '🐕', '🐖',
        '🐁', '🐂', '🐲', '🐡', '🐊', '🐫', '🐪', '🐆', '🐈', '🐩', '🐾', '💐', '🌸',
        '🌷', '🍀', '🌹', '🌻', '🌺', '🍁', '🍃', '🍂', '🌿', '🌾', '🍄', '🌵', '🌴',
        '🌲', '🌳', '🌰', '🌱', '🌼', '🌐', '🌞', '🌝', '🌚', '🌜', '🌛', '🌙', '🌍',
        '🌎', '🌏', '⭐', '⛅', '⛄', '🌀', '💝', '🎒', '🎓', '🎏', '🎃', '👻', '🎄',
        '🎁', '🎋', '🎉', '🎈', '🔮', '🎥', '📷', '📹', '📼', '💿', '📀', '💽', '💾',
        '💻', '📱', '📞', '📟', '📠', '📡', '📺', '📻', '🔊', '🔔', '📢', '⏳', '⏰',
        '🔓', '🔒', '🔏', '🔐', '🔑', '🔎', '💡', '🔦', '🔆', '🔅', '🔌', '🔋', '🔍',
        '🛁', '🚿', '🚽', '🔧', '🔨', '🚪', '💣', '🔫', '🔪', '💊', '💉', '💰', '💸',
        '📨', '📬', '📌', '📎', '📕', '📓', '📚', '📖', '🔬', '🔭', '🎨', '🎬', '🎤',
        '🎵', '🎹', '🎻', '🎺', '🎷', '🎸', '👾', '🎮', '🃏', '🎲', '🎯', '🏈', '🏀',
        '⚽', '🎾', '🎱', '🏉', '🎳', '⛳', '🚴', '🏁', '🏇', '🏆', '🎿', '🏂', '🏄',
        '🎣', '🍵', '🍶', '🍼', '🍺', '🍻', '🍸', '🍹', '🍷', '🍴', '🍕', '🍔', '🍟',
        '🍗', '🍤', '🍞', '🍩', '🍮', '🍦', '🍨', '🍧', '🎂', '🍰', '🍪', '🍫', '🍬',
        '🍭', '🍯', '🍎', '🍏', '🍊', '🍋', '🍒', '🍇', '🍉', '🍓', '🍑', '🍌', '🍐',
        '🍍', '🍆', '🍅', '🌽', '🏠', '🏡', '⛵', '🚤', '🚣', '🚀', '🚁', '🚂', '🚎',
        '🚌', '🚍', '🚙', '🚘', '🚗', '🚕', '🚖', '🚛', '🚚', '🚨', '🚓', '🚔', '🚒',
        '🚑', '🚐', '🚲', '🚜', '💈', '🚦', '🚧', '🏮', '🎰', '🗿', '🎪', '🎭', '📍',
        '🚩', '💯',
    ];
);

#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum ShieldIconTreatment {
    SingleColor,
    TwoColor {
        pattern_color: Color,
        angle: u16,
    },
    Stripes {
        pattern_color: Color,
        stride: f32,
        stripe_xs: Vec<f32>,
        angle: u16,
    },
}

#[derive(Serialize, Deserialize)]
struct ShieldIconData {
    treatment: ShieldIconTreatment,
    field_color: Color,
    emoji: char,
}

impl ShieldIconData {
    fn empty() -> Self {
        ShieldIconData {
            treatment: ShieldIconTreatment::SingleColor,
            field_color: Color::black(),
            emoji: ' ',
        }
    }
}

impl rand::Rand for ShieldIconData {
    fn rand<R: Rng>(rng: &mut R) -> Self {
        let mut rv = ShieldIconData::empty();

        let angle_choices: Vec<u16> = (0..8).map(|a| a * 45).collect();

        rv.field_color = *rng.choose(&COLORS).unwrap();
        let contrasting_colors: Vec<Color> = COLORS.iter()
            .filter(|c| rv.field_color.contrasts_well(c))
            .map(|c| *c)
            .collect();
        rv.emoji = *rng.choose(&EMOJIS).unwrap();

        let pattern_color = *rng.choose(&contrasting_colors).unwrap();

        let treatment_name = rng.weighted_choice(vec![
            ("SingleColor", 1),
            ("TwoColor", 4),
            ("Stripes", 6),
        ]);

        match treatment_name {
            "SingleColor" => (),
            "TwoColor" => {
                let angle = *rng.choose(&angle_choices).unwrap();
                rv.treatment = ShieldIconTreatment::TwoColor { angle, pattern_color };
            },
            "Stripes" => {
                let count: u8 = rng.gen_range(1, 4);
                let padding = rng.gen_range(0.1, 0.4);
                let stride = (1.0 - 2.0 * padding) / (2.0 * count as f32 + 1.0);
                let stripe_xs: Vec<f32> = (0..count)
                    .map(|i| padding + stride * (2 * i + 1) as f32)
                    .collect();
                let angle = *rng.choose(&angle_choices).unwrap();
                rv.treatment = ShieldIconTreatment::Stripes { stride, stripe_xs, pattern_color, angle };
            },
            _ => panic!("Unexpected treatment name"),
        }

        rv
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
