extern crate rand;
extern crate tera;

use std::default::Default;
use serde::Serialize;

lazy_static! {
    static ref TERA_ENGINE: tera::Tera = {
        let mut engine = tera::Tera::default();
        engine.add_raw_templates(vec![
            ("index.html.tera", include_str!("templates/index.html.tera")),
        ]).unwrap();
        engine
    };
}

/// Render a named template
pub fn render<T: Serialize>(template_name: &str, data: &T) -> tera::Result<String> {
    TERA_ENGINE.render(template_name, data)
}
