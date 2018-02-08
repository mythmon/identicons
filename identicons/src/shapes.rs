extern crate rand;

use super::{data, Color};

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
    fn empty() -> Self {
        ShapeIconData {
            emoji: ' ',
            shape: ShapeType::Circle,
            fill_color: Color::white(),
            border_color: Color::black(),
            offset: 0.0,
        }
    }
}

impl rand::Rand for ShapeIconData {
    fn rand<R: rand::Rng>(rng: &mut R) -> Self {
        let mut rv = ShapeIconData::empty();

        rv.emoji = *rng.choose(&data::EMOJIS).unwrap();

        let white = Color::white();
        let contrasts_with_white: Vec<Color> = data::COLORS.iter()
            .filter(|c| white.contrasts_well(c))
            .map(|c| *c)
            .collect();

        rv.border_color = *rng.choose(&contrasts_with_white).unwrap();

        let contrasts_with_border: Vec<Color> = data::COLORS.iter()
            .filter(|c| rv.border_color.contrasts_well(c))
            .map(|c| *c)
            .collect();
        rv.fill_color = *rng.choose(&contrasts_with_border).unwrap();

        let num_sides: u8 = rng.gen_range(1, 10);
        if num_sides <= 2 {
            // A polygon with 2 or fewer sides doesn't make sense, so make it a circle instead.
            rv.shape = ShapeType::Circle;
        } else {
            rv.shape = ShapeType::Polygon(num_sides);
        }

        // bias aligned and half aligned by giving then 1/4 of the space each
        rv.offset = rng.gen_range(-1.0, 1.0);
        if rv.offset < 0.5 {
            rv.offset = 0.5;
        } else if rv.offset < 0.0 {
            rv.offset = 0.0;
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
        let expected = ShapeIconData {
            emoji: '😃',
            shape: ShapeType::Polygon(8),
            fill_color: Color { r: 0, g: 254, b: 255 },
            border_color: Color { r: 98, g: 0, b: 164 },
            offset: 0.5,
        };
        let actual = rng.gen();
        assert_eq!(expected, actual);

        // ----

        let expected = ShapeIconData {
            emoji: '🐅',
            shape: ShapeType::Polygon(9),
            fill_color: Color { r: 177, g: 177, b: 179 },
            border_color: Color { r: 0, g: 90, b: 113 },
            offset: 0.5,
        };
        let mut rng = rand::XorShiftRng::from_seed([42, 42, 42, 42]);
        let actual = rng.gen();
        assert_eq!(expected, actual);
    }
}
