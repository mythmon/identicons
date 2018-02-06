extern crate rand;

mod data;
mod shields;
mod shapes;

pub use self::shields::{ShieldIconData, ShieldIconTreatment};
pub use self::shapes::{ShapeIconData, ShapeType};

trait RngExt {
    /// Choose a random item from a collection by weight.
    fn weighted_choice<T>(&mut self, choices: Vec<(T, usize)>) -> T;
}

impl<R: rand::Rng> RngExt for R {
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

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color {
    /// Red component
    pub r: u8,
    /// Blue component
    pub g: u8,
    /// Green component
    pub b: u8,
}

impl Color {
    pub fn black() -> Self {
        Self { r: 0, g: 0, b: 0 }
    }

    pub fn white() -> Self {
        Self { r: 255, g: 255, b: 255 }
    }

    /// Format this color as a CSS color.
    ///
    ///     # use identicons::icons::Color;
    ///     let c = Color { r: 12, g: 34, b: 56 };
    ///     assert_eq!(c.css_color(), "rgb(12,34,56)".to_string());
    ///
    pub fn css_color(&self) -> String {
        format!("rgb({},{},{})", self.r, self.g, self.b)
    }

    pub fn luminance(&self) -> f32 {
        0.2126 * self.r as f32 + 0.7152 * self.g as f32 + 0.0722 * self.b as f32
    }

    pub fn contrasts_well(&self, other: &Self) -> bool {
        (self.luminance() - other.luminance()).abs() > 75.0
    }
}
