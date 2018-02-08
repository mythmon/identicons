extern crate rand;

use super::{data, Color, RngExt};

/// A description of a treatment for a shield.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum ShieldIconTreatment {
    /// A single, solid shield color, aka no treatment.
    SingleColor,

    /// A treatment that results in a two-color shield pattern, by applying
    /// another color at an angle.
    TwoColor {
        /// The color of the pattern.
        pattern_color: Color,
        /// The treatment's angle.
        angle: u16,
    },

    /// A treatment that results in a two-color striped shield pattern.
    Stripes {
        /// The color of the strips we are adding.
        pattern_color: Color,
        /// The strip's stride.
        stride: f32,
        /// X coordinates for the stripes.
        stripe_xs: Vec<f32>,
        /// Angle of the stripes.
        angle: u16,
    },
}

/// A description of a shield icon.
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
