extern crate rand;

mod data;

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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShieldIconData {
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
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        let mut rv = ShieldIconData::empty();

        let angle_choices: Vec<u16> = (0..8).map(|a| a * 45).collect();

        rv.field_color = *rng.choose(&data::COLORS).unwrap();
        let contrasting_colors: Vec<Color> = data::COLORS.iter()
            .filter(|c| rv.field_color.contrasts_well(c))
            .map(|c| *c)
            .collect();
        rv.emoji = *rng.choose(&data::EMOJIS).unwrap();

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

#[cfg(test)]
mod tests {
    use super::*;
    use rand::{Rng, SeedableRng};

    /// Test that certain seeds always generate the same icon
    /// data. This is to make sure that icons don't change overtime,
    /// since they are supposed to always be the same for a particular
    /// hash.
    #[test]
    fn test_consistent_icons() {
        let mut rng = rand::XorShiftRng::from_seed([1, 2, 3, 4]);
        let expected = ShieldIconData {
            emoji: '🐛',
            field_color: Color { r: 164, g: 0, b: 15 },
            treatment: ShieldIconTreatment::TwoColor {
                pattern_color: Color { r: 0, g: 254, b: 255 },
                angle: 45,
            },
        };
        let actual = rng.gen();
        assert_eq!(expected, actual);

        // ----

        let expected = ShieldIconData {
            emoji: '🐅',
            field_color: Color { r: 68, g: 0, b: 113 },
            treatment: ShieldIconTreatment::Stripes {
                pattern_color: Color { r: 215, g: 110, b: 0 },
                stride: 0.10725436,
                stripe_xs: vec![0.2318641, 0.44637284, 0.6608815],
                angle: 45,
            },
        };
        let mut rng = rand::XorShiftRng::from_seed([42, 42, 42, 42]);
        let actual = rng.gen();
        assert_eq!(expected, actual);
    }
}
