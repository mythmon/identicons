use std::collections::HashMap;
use super::Color;

lazy_static! {
   /// Derived from <https://github.com/FirefoxUX/design-tokens/blob/29dc1033677c2b4721817b1d61b53034d0deea74/photon-colors/photon-colors.json>
    pub static ref COLOR_MAP: HashMap<&'static str, Color> = {
        let mut map = HashMap::new();

        map.insert("magenta50", Color { r: 0xff, g: 0x1a, b: 0xd9 });
        map.insert("magenta60", Color { r: 0xed, g: 0x00, b: 0xb5 });
        map.insert("magenta70", Color { r: 0xb5, g: 0x00, b: 0x7f });
        map.insert("magenta80", Color { r: 0x7d, g: 0x00, b: 0x4f });
        map.insert("magenta90", Color { r: 0x44, g: 0x00, b: 0x27 });
        map.insert("purple50", Color { r: 0x94, g: 0x00, b: 0xff });
        map.insert("purple60", Color { r: 0x80, g: 0x00, b: 0xd7 });
        map.insert("purple70", Color { r: 0x62, g: 0x00, b: 0xa4 });
        map.insert("purple80", Color { r: 0x44, g: 0x00, b: 0x71 });
        map.insert("purple90", Color { r: 0x25, g: 0x00, b: 0x3e });
        map.insert("blue40", Color { r: 0x45, g: 0xa1, b: 0xff });
        map.insert("blue50", Color { r: 0x0a, g: 0x84, b: 0xff });
        map.insert("blue60", Color { r: 0x00, g: 0x60, b: 0xdf });
        map.insert("blue70", Color { r: 0x00, g: 0x3e, b: 0xaa });
        map.insert("blue80", Color { r: 0x00, g: 0x22, b: 0x75 });
        map.insert("blue90", Color { r: 0x00, g: 0x0f, b: 0x40 });
        map.insert("teal50", Color { r: 0x00, g: 0xfe, b: 0xff });
        map.insert("teal60", Color { r: 0x00, g: 0xc8, b: 0xd7 });
        map.insert("teal70", Color { r: 0x00, g: 0x8e, b: 0xa4 });
        map.insert("teal80", Color { r: 0x00, g: 0x5a, b: 0x71 });
        map.insert("teal90", Color { r: 0x00, g: 0x2d, b: 0x3e });
        map.insert("green50", Color { r: 0x30, g: 0xe6, b: 0x0b });
        map.insert("green60", Color { r: 0x12, g: 0xbc, b: 0x00 });
        map.insert("green70", Color { r: 0x05, g: 0x8b, b: 0x00 });
        map.insert("green80", Color { r: 0x00, g: 0x65, b: 0x04 });
        map.insert("green90", Color { r: 0x00, g: 0x37, b: 0x06 });
        map.insert("yellow50", Color { r: 0xff, g: 0xe9, b: 0x00 });
        map.insert("yellow60", Color { r: 0xd7, g: 0xb6, b: 0x00 });
        map.insert("yellow70", Color { r: 0xa4, g: 0x7f, b: 0x00 });
        map.insert("yellow80", Color { r: 0x71, g: 0x51, b: 0x00 });
        map.insert("yellow90", Color { r: 0x3e, g: 0x28, b: 0x00 });
        map.insert("red50", Color { r: 0xff, g: 0x00, b: 0x39 });
        map.insert("red60", Color { r: 0xd7, g: 0x00, b: 0x22 });
        map.insert("red70", Color { r: 0xa4, g: 0x00, b: 0x0f });
        map.insert("red80", Color { r: 0x5a, g: 0x00, b: 0x02 });
        map.insert("red90", Color { r: 0x3e, g: 0x02, b: 0x00 });
        map.insert("orange50", Color { r: 0xff, g: 0x94, b: 0x00 });
        map.insert("orange60", Color { r: 0xd7, g: 0x6e, b: 0x00 });
        map.insert("orange70", Color { r: 0xa4, g: 0x49, b: 0x00 });
        map.insert("orange80", Color { r: 0x71, g: 0x2b, b: 0x00 });
        map.insert("orange90", Color { r: 0x3e, g: 0x13, b: 0x00 });
        map.insert("grey10", Color { r: 0xf9, g: 0xf9, b: 0xfa });
        map.insert("grey20", Color { r: 0xed, g: 0xed, b: 0xf0 });
        map.insert("grey30", Color { r: 0xd7, g: 0xd7, b: 0xdb });
        map.insert("grey40", Color { r: 0xb1, g: 0xb1, b: 0xb3 });
        map.insert("grey50", Color { r: 0x73, g: 0x73, b: 0x73 });
        map.insert("grey60", Color { r: 0x4a, g: 0x4a, b: 0x4f });
        map.insert("grey70", Color { r: 0x38, g: 0x38, b: 0x3d });
        map.insert("grey80", Color { r: 0x2a, g: 0x2a, b: 0x2e });
        map.insert("grey90", Color { r: 0x0c, g: 0x0c, b: 0x0d });
        map.insert("ink70", Color { r: 0x36, g: 0x39, b: 0x59 });
        map.insert("ink80", Color { r: 0x20, g: 0x23, b: 0x40 });
        map.insert("ink90", Color { r: 0x0f, g: 0x11, b: 0x26 });

        map
    };

    pub static ref COLORS: Vec<Color> = {
        let mut keys: Vec<&&'static str> = COLOR_MAP.keys().collect();
        keys.sort();
        keys.into_iter().map(|m| COLOR_MAP.get(m).unwrap().clone()).collect()
    };

    pub static ref EMOJIS: Vec<char> = vec![
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
}
