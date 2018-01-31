extern crate iron;
extern crate rand;
extern crate router;
extern crate iron_tera;
extern crate tera;
#[macro_use]
extern crate lazy_static;
extern crate ctrlc;

use iron::prelude::*;
use iron::{status, AfterMiddleware};
use rand::{Rng, SeedableRng};
use router::Router;
use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use tera::Context;
use iron_tera::{Template, TemplateMode, TeraEngine};

// TODO read PORT and HOST from env
// TODO add support for versions
// TODO multiple template support (need another template)
// TODO smarter context generation (probably a struct for all the needed fields?)
// TODO can treatments be sub-templates?
// TODO can templates and their data be better tied?

fn main() {
    // Rust doesn't have a ctrl-c handler itself, so when running as
    // PID 1 in Docker it doesn't respond to SIGINT. This prevents
    // ctrl-c from stopping a docker container running this
    // program. Handle SIGINT (aka ctrl-c) to fix this problem.
    ctrlc::set_handler(move || {
        ::std::process::exit(1);
    }).expect("error setting ctrl-c handler");

    let mut router = Router::new();
    router.get("/", handler, "index");
    router.get("/:query", handler, "shield");

    let mut chain = Chain::new(router);

    let teng = TeraEngine::new("templates/**/*");
    chain.link_after(teng);
    chain.link_after(ErrorHandler);

    let host = "0.0.0.0:3000";
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

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    fn css_color(&self) -> String {
        format!("rgb({},{},{})", self.r, self.g, self.b)
    }

    fn luminance(&self) -> f64 {
        0.2126 * self.r as f64 + 0.7152 * self.g as f64 + 0.0722 * self.b as f64
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

fn handler(req: &mut Request) -> Result<Response, IronError> {
    let ref query = req.extensions.get::<Router>().unwrap()
        .find("query").unwrap_or("/");

    let mut hasher = DefaultHasher::new();
    hasher.write(&query.bytes().collect::<Vec<u8>>());
    let hash = hasher.finish();

    let high = ((hash & 0xFFFF_FFFF_0000_0000) >> 32) as u32;
    let low = (hash & 0x0000_0000_FFFF_FFFF) as u32;
    let seed = [high, low, 0, 0];
    let mut rng = rand::XorShiftRng::from_seed(seed);

    let mut context = Context::new();

    let treatment = rng.weighted_choice(vec![
        ("SingleColor", 1),
        ("TwoColor", 4),
        ("Stripes", 6),
    ]);
    context.add("treatment", &treatment);

    let angle_choices: Vec<u16> = (0..8).map(|a| a * 45).collect();

    match treatment {
        "SingleColor" => (),
        "TwoColor" => {
            let angle = rng.choose(&angle_choices).unwrap();
            context.add("transform", &format!("scale(100) rotate({} 0.5,0.5)", angle));
        }
        "Stripes" => {
            let count: u8 = rng.gen_range(1, 4);
            let padding = rng.gen_range(0.1, 0.4);
            let stride = (1.0 - 2.0 * padding) / (2.0 * count as f64 + 1.0);
            let stripe_x_list: Vec<f64> = (0..count)
                .map(|i| padding + stride * (2 * i + 1) as f64)
                .collect();
            context.add("stripe_x_list", &stripe_x_list);
            context.add("stride", &stride);
            let angle = rng.choose(&angle_choices).unwrap();
            context.add("transform", &format!("scale(100) rotate({} 0.5,0.5)", angle));
        }
        _ => panic!("Unknown treatment"),
    }

    let field_color = rng.choose(&COLORS).unwrap();
    context.add("field_color", &field_color.css_color());
    let contrasting_colors: Vec<Color> = COLORS.iter()
        .filter(|c| field_color.contrasts_well(c))
        .map(|c| *c)
        .collect();
    context.add("pattern_color", &rng.choose(&contrasting_colors).unwrap().css_color());
    context.add("emoji", &rng.choose(&EMOJIS).unwrap());

    let template = Template::new("shield.svg.tmpl", TemplateMode::from_context(context));

    let mut resp = Response::new();
    resp.set_mut(status::Ok);
    resp.set_mut(template);
    Ok(resp)
}
