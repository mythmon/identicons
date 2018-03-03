extern crate rand;
extern crate tera;

use std::collections::HashMap;
use std::default::Default;
use serde::Serialize;
use super::Color;

lazy_static! {
    pub static ref TERA_ENGINE: tera::Tera = {
        let mut engine = tera::Tera::default();
        engine.add_raw_templates(vec![
            ("shield.svg.tmpl", include_str!("templates/shield.svg.tmpl")),
            ("shape.svg.tmpl", include_str!("templates/shape.svg.tmpl")),
        ]).unwrap();
        engine.register_filter("css", tera_to_css);
        engine
    };
}

/// Render a tera::Value as a CSS value, if possible
pub fn tera_to_css(value: tera::Value, _args: HashMap<String, tera::Value>) -> tera::Result<tera::Value> {
    let debug_copy = value.clone();
    if let Ok(color) = tera::from_value::<Color>(value) {
        Ok(tera::Value::String(color.css_color()))
    } else {
        Err(tera::Error::from_kind(tera::ErrorKind::Msg(format!("css is not implemented for {:?}", debug_copy))))
    }
}

pub fn render<T: Serialize>(template_name: &str, data: &T) -> tera::Result<String> {
    TERA_ENGINE.render(template_name, data)
}
