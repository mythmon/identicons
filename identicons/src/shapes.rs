extern crate rand;
extern crate tera;

use super::{
    data, genome::{Genome, GenomeGen, GenomeResult}, templ, Color,
};
use std::default::Default;

/// A shape.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ShapeType {
    /// A polygon of `n` sides.
    Polygon(u8),
    /// A circle.
    Circle,
}

/// A description of a shape icon.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ShapeIconData {
    /// The emoji to overlay on the icon.
    pub emoji: char,
    /// The icon's shape.
    pub shape: ShapeType,
    /// The icon's fill color.
    pub fill_color: Color,
    /// The icon's border color.
    pub border_color: Color,
    /// The offset of the icon.
    pub offset: f32,
}

impl ShapeIconData {
    /// Generate a ShapeIconData by hashing an input and choosing unpredictable
    /// values for all parameters.
    pub fn from_input<'a, T: Into<String>>(input: T) -> GenomeResult<Self> {
        let mut genome = Genome::via_sha512(input);
        genome.gen()
    }

    /// Render as an SVG.
    pub fn to_svg(&self) -> tera::Result<String> {
        let mut context = tera::Context::new();
        context.add("icon", &self);

        if let ShapeType::Polygon(sides) = self.shape {
            let step = ::std::f32::consts::PI * 2.0 / (sides as f32);
            let offset = step * self.offset;
            let radius = 0.45;
            let points: Vec<(f32, f32)> = (0..sides)
                .map(|i| {
                    let ang = step * i as f32 + offset;
                    (ang.cos() * radius + 0.5, ang.sin() * radius + 0.5)
                })
                .collect();
            context.add("points", &points);
        }

        templ::render("shape.svg.tera", &context)
    }
}

impl GenomeGen for ShapeIconData {
    fn gen(genome: &mut Genome) -> Result<Self, ()> {
        let mut rv = ShapeIconData::default();

        rv.emoji = genome.choose(&data::EMOJIS)?;

        let white = Color::white();
        let contrasts_with_white: Vec<Color> = data::COLORS
            .iter()
            .filter(|c| white.contrasts_well(c))
            .map(|c| *c)
            .collect();

        rv.border_color = genome.choose(&contrasts_with_white)?;

        let contrasts_with_border: Vec<Color> = data::COLORS
            .iter()
            .filter(|c| rv.border_color.contrasts_well(c))
            .map(|c| *c)
            .collect();
        rv.fill_color = genome.choose(&contrasts_with_border)?;

        let num_sides: u8 = genome.gen_range(1u8, 10u8)?;
        if num_sides <= 2 {
            // A polygon with 2 or fewer sides doesn't make sense, so make it a circle instead.
            rv.shape = ShapeType::Circle;
        } else {
            rv.shape = ShapeType::Polygon(num_sides);
        }

        // bias aligned and half aligned by giving then 1/4 of the space each
        let rotation: u8 = genome.gen_range(0, 100)?;
        if rotation >= 75 {
            rv.offset = 0.5;
        } else if rotation >= 50 {
            rv.offset = 0.0;
        } else {
            rv.offset = rotation as f32 / 50.
        }

        Ok(rv)
    }
}

impl Default for ShapeIconData {
    fn default() -> Self {
        ShapeIconData {
            emoji: 'A',
            shape: ShapeType::Circle,
            fill_color: Color::white(),
            border_color: Color::black(),
            offset: 0.0,
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
        let expected = ShapeIconData {
            emoji: '🎺',
            shape: ShapeType::Polygon(4),
            fill_color: Color {
                r: 18,
                g: 188,
                b: 0,
            },
            border_color: Color {
                r: 128,
                g: 0,
                b: 215,
            },
            offset: 0.0,
        };
        let actual = ShapeIconData::from_input("one").unwrap();
        assert_eq!(expected, actual);

        // ----
 
        let expected = ShapeIconData {
            emoji: '🚛',
            shape: ShapeType::Polygon(6),
            fill_color: Color {
                r: 90,
                g: 0,
                b: 2,
            },
            border_color: Color {
                r: 48,
                g: 230,
                b: 11,
            },
            offset: 0.04,
        };
        let actual = ShapeIconData::from_input("two").unwrap();
        assert_eq!(expected, actual);     }
}
