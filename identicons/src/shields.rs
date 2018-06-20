extern crate rand;
extern crate tera;

use super::templ;
use super::{
    data, genome::{Genome, GenomeGen, GenomeResult}, Color,
};
use std::default::Default;

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
    /// Generate a ShapeIconData by hashing an input and choosing unpredictable
    /// values for all parameters.
    pub fn from_input<'a, T: Into<String>>(input: T) -> GenomeResult<Self> {
        let mut genome = Genome::via_sha512(input);
        genome.gen()
    }

    /// Render as an SVG.
    pub fn to_svg(&self) -> Result<String, tera::Error> {
        let mut context = tera::Context::new();
        context.add("icon", &self);
        templ::render("shield.svg.tera", &context)
    }
}

impl GenomeGen for ShieldIconData {
    fn gen(genome: &mut Genome) -> GenomeResult<Self> {
        let mut rv = ShieldIconData::default();

        let angle_choices: Vec<u16> = (0..8).map(|a| a * 45).collect();

        rv.field_color = genome.choose(&data::COLORS).unwrap();
        let contrasting_colors: Vec<Color> = data::COLORS
            .iter()
            .filter(|c| rv.field_color.contrasts_well(c))
            .map(|c| *c)
            .collect();
        rv.emoji = genome.choose(&data::EMOJIS).unwrap();

        let pattern_color = genome.choose(&contrasting_colors).unwrap();

        let treatment_name =
            genome.choose_weighted(&vec![("SingleColor", 1), ("TwoColor", 4), ("Stripes", 6)])?;

        match treatment_name {
            "SingleColor" => (),
            "TwoColor" => {
                let angle = genome.choose(&angle_choices).unwrap();
                rv.treatment = ShieldIconTreatment::TwoColor {
                    angle,
                    pattern_color,
                };
            }
            "Stripes" => {
                let count: u8 = genome.gen_range(1, 4)?;
                let padding = genome.gen_range(10u8, 40u8)? as f32 / 10.;
                let stride = (1.0 - 2.0 * padding) / (2.0 * count as f32 + 1.0);
                let stripe_xs: Vec<f32> = (0..count)
                    .map(|i| padding + stride * (2 * i + 1) as f32)
                    .collect();
                let angle = genome.choose(&angle_choices).unwrap();
                rv.treatment = ShieldIconTreatment::Stripes {
                    stride,
                    stripe_xs,
                    pattern_color,
                    angle,
                };
            }
            _ => panic!("Unexpected treatment name"),
        }

        Ok(rv)
    }
}

impl Default for ShieldIconData {
    fn default() -> Self {
        ShieldIconData {
            treatment: ShieldIconTreatment::SingleColor,
            field_color: Color::black(),
            emoji: 'A',
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test that certain seeds always generate the same icon
    /// data. This is to make sure that icons don't change overtime,
    /// since they are supposed to always be the same for a particular
    /// hash.
    #[test]
    fn test_consistent_icons() {
        let expected = ShieldIconData {
            emoji: '🚣',
            field_color: Color {
                r: 125,
                g: 0,
                b: 79,
            },
            treatment: ShieldIconTreatment::TwoColor {
                pattern_color: Color { r: 177, g: 177, b: 179 },
                angle: 0,
            }
        };
        let actual = ShieldIconData::from_input("one").unwrap();
        assert_eq!(expected, actual);

        // ----

        let expected = ShieldIconData {
            emoji: '🎬',
            field_color: Color { r: 148, g: 0, b: 255 },
            treatment: ShieldIconTreatment::TwoColor {
                pattern_color: Color {
                    r: 0,
                    g: 254,
                    b: 255,
                },
                angle: 180,
            },
        };
        let actual = ShieldIconData::from_input("two").unwrap();
        assert_eq!(expected, actual);
    }
}
