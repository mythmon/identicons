extern crate rand;

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
        let expected = ShieldIconData {
            emoji: '🐛',
            field_color: Color { r: 133, g: 153, b: 0 },
            treatment: ShieldIconTreatment::TwoColor {
                pattern_color: Color { r: 238, g: 232, b: 213 },
                angle: 45,
            }
        };
        let mut rng = rand::XorShiftRng::from_seed([1, 2, 3, 4]);
        let actual = rng.gen();
        assert_eq!(expected, actual);

        // ----

        let expected = ShieldIconData {
            emoji: '🐅',
            field_color: Color { r: 220, g: 50, b: 47 },
            treatment: ShieldIconTreatment::Stripes {
                pattern_color: Color { r: 255, g: 207, b: 0 },
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
