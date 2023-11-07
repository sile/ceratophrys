#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl Color {
    pub const fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }

    pub const fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self::rgba(r, g, b, 255)
    }

    pub const fn rgba_hex(hex: u32) -> Self {
        Self::rgba(
            ((hex >> 24) & 0xFF) as u8,
            ((hex >> 16) & 0xFF) as u8,
            ((hex >> 8) & 0xFF) as u8,
            (hex & 0xFF) as u8,
        )
    }

    pub const fn rgb_hex(hex: u32) -> Self {
        Self::rgb(
            ((hex >> 16) & 0xFF) as u8,
            ((hex >> 8) & 0xFF) as u8,
            (hex & 0xFF) as u8,
        )
    }

    pub const fn alpha(self, a: u8) -> Self {
        Self::rgba(self.r, self.g, self.b, a)
    }

    pub const fn to_rgba(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    pub const fn is_transparent(self) -> bool {
        self.a == 0
    }

    pub fn alpha_blend(self, dst: Self) -> Self {
        if dst.a == 0 {
            return self;
        } else if self.a == 0 {
            return dst;
        } else if self.a == 0xFF {
            return self;
        }

        fn blend(s: u32, d: u32, a: u32) -> u32 {
            s + d - (d * a / (0xFF * 0xFF))
        }

        let a = blend(
            u32::from(self.a) * 0xFF,
            u32::from(dst.a) * 0xFF,
            u32::from(self.a) * 0xFF,
        );
        let r = blend(
            u32::from(self.r) * u32::from(self.a),
            u32::from(dst.r) * u32::from(dst.a),
            u32::from(self.a) * 0xFF,
        );
        let g = blend(
            u32::from(self.g) * u32::from(self.a),
            u32::from(dst.g) * u32::from(dst.a),
            u32::from(self.a) * 0xFF,
        );
        let b = blend(
            u32::from(self.b) * u32::from(self.a),
            u32::from(dst.b) * u32::from(dst.a),
            u32::from(self.a) * 0xFF,
        );
        Self {
            r: (r * 0xFF * 0xFF / a / 0xFF) as u8,
            g: (g * 0xFF * 0xFF / a / 0xFF) as u8,
            b: (b * 0xFF * 0xFF / a / 0xFF) as u8,
            a: (a / 0xFF) as u8,
        }
    }
}

// https://developer.mozilla.org/ja/docs/Web/CSS/named-color
impl Color {
    pub const ALICE_BLUE: Self = Self::rgb_hex(0xf0f8ff);
    pub const ANTIQUE_WHITE: Self = Self::rgb_hex(0xfaebd7);
    pub const AQUA: Self = Self::rgb_hex(0x00ffff);
    pub const AQUAMARINE: Self = Self::rgb_hex(0x7fffd4);
    pub const AZURE: Self = Self::rgb_hex(0xf0ffff);
    pub const BEIGE: Self = Self::rgb_hex(0xf5f5dc);
    pub const BISQUE: Self = Self::rgb_hex(0xffe4c4);
    pub const BLACK: Self = Self::rgb_hex(0x000000);
    pub const BLANCHED_ALMOND: Self = Self::rgb_hex(0xffebcd);
    pub const BLUE: Self = Self::rgb_hex(0x0000ff);
    pub const BLUE_VIOLET: Self = Self::rgb_hex(0x8a2be2);
    pub const BROWN: Self = Self::rgb_hex(0xa52a2a);
    pub const BURLY_WOOD: Self = Self::rgb_hex(0xdeb887);
    pub const CADET_BLUE: Self = Self::rgb_hex(0x5f9ea0);
    pub const CHARTREUSE: Self = Self::rgb_hex(0x7fff00);
    pub const CHOCOLATE: Self = Self::rgb_hex(0xd2691e);
    pub const CORAL: Self = Self::rgb_hex(0xff7f50);
    pub const CORNFLOWER_BLUE: Self = Self::rgb_hex(0x6495ed);
    pub const CORNSILK: Self = Self::rgb_hex(0xfff8dc);
    pub const CRIMSON: Self = Self::rgb_hex(0xdc143c);
    pub const CYAN: Self = Self::AQUA;
    pub const DARK_BLUE: Self = Self::rgb_hex(0x00008b);
    pub const DARK_CYAN: Self = Self::rgb_hex(0x008b8b);
    pub const DARK_GOLDENROD: Self = Self::rgb_hex(0xb8860b);
    pub const DARK_GRAY: Self = Self::rgb_hex(0xa9a9a9);
    pub const DARK_GREEN: Self = Self::rgb_hex(0x006400);
    pub const DARK_GREY: Self = Self::DARK_GRAY;
    pub const DARK_KHAKI: Self = Self::rgb_hex(0xbdb76b);
    pub const DARK_MAGENTA: Self = Self::rgb_hex(0x8b008b);
    pub const DARK_OLIVEGREEN: Self = Self::rgb_hex(0x556b2f);
    pub const DARK_ORANGE: Self = Self::rgb_hex(0xff8c00);
    pub const DARK_ORCHID: Self = Self::rgb_hex(0x9932cc);
    pub const DARK_RED: Self = Self::rgb_hex(0x8b0000);
    pub const DARK_SALMON: Self = Self::rgb_hex(0xe9967a);
    pub const DARK_SEA_GREEN: Self = Self::rgb_hex(0x8fbc8f);
    pub const DARK_SLATE_BLUE: Self = Self::rgb_hex(0x483d8b);
    pub const DARK_SLATE_GRAY: Self = Self::rgb_hex(0x2f4f4f);
    pub const DARK_SLATE_GREY: Self = Self::DARK_SLATE_GRAY;
    pub const DARK_TURQUOISE: Self = Self::rgb_hex(0x00ced1);
    pub const DARK_VIOLET: Self = Self::rgb_hex(0x9400d3);
    pub const DEEP_PINK: Self = Self::rgb_hex(0xff1493);
    pub const DEEP_SKY_BLUE: Self = Self::rgb_hex(0x00bfff);
    pub const DIM_GRAY: Self = Self::rgb_hex(0x696969);
    pub const DIM_GREY: Self = Self::DIM_GRAY;
    pub const DODGER_BLUE: Self = Self::rgb_hex(0x1e90ff);
    pub const FIRE_BRICK: Self = Self::rgb_hex(0xb22222);
    pub const FLORAL_WHITE: Self = Self::rgb_hex(0xfffaf0);
    pub const FOREST_GREEN: Self = Self::rgb_hex(0x228b22);
    pub const FUCHSIA: Self = Self::rgb_hex(0xff00ff);
    pub const GAINSBORO: Self = Self::rgb_hex(0xdcdcdc);
    pub const GHOST_WHITE: Self = Self::rgb_hex(0xf8f8ff);
    pub const GOLD: Self = Self::rgb_hex(0xffd700);
    pub const GOLDEN_ROD: Self = Self::rgb_hex(0xdaa520);
    pub const GRAY: Self = Self::rgb_hex(0x808080);
    pub const GREEN: Self = Self::rgb_hex(0x008000);
    pub const GREEN_YELLOW: Self = Self::rgb_hex(0xadff2f);
    pub const GREY: Self = Self::GRAY;
    pub const HONEY_DEW: Self = Self::rgb_hex(0xf0fff0);
    pub const HOT_PINK: Self = Self::rgb_hex(0xff69b4);
    pub const INDIAN_RED: Self = Self::rgb_hex(0xcd5c5c);
    pub const INDIGO: Self = Self::rgb_hex(0x4b0082);
    pub const IVORY: Self = Self::rgb_hex(0xfffff0);
    pub const KHAKI: Self = Self::rgb_hex(0xf0e68c);
    pub const LAVENDER: Self = Self::rgb_hex(0xe6e6fa);
    pub const LAVENDER_BLUSH: Self = Self::rgb_hex(0xfff0f5);
    pub const LAWN_GREEN: Self = Self::rgb_hex(0x7cfc00);
    pub const LEMON_CHIFFON: Self = Self::rgb_hex(0xfffacd);
    pub const LIGHT_BLUE: Self = Self::rgb_hex(0xadd8e6);
    pub const LIGHT_CORAL: Self = Self::rgb_hex(0xf08080);
    pub const LIGHT_CYAN: Self = Self::rgb_hex(0xe0ffff);
    pub const LIGHT_GOLDEN_ROD_YELLOW: Self = Self::rgb_hex(0xfafad2);
    pub const LIGHT_GRAY: Self = Self::rgb_hex(0xd3d3d3);
    pub const LIGHT_GREEN: Self = Self::rgb_hex(0x90ee90);
    pub const LIGHT_GREY: Self = Self::LIGHT_GRAY;
    pub const LIGHT_PINK: Self = Self::rgb_hex(0xffb6c1);
    pub const LIGHT_SALMON: Self = Self::rgb_hex(0xffa07a);
    pub const LIGHT_SEA_GREEN: Self = Self::rgb_hex(0x20b2aa);
    pub const LIGHT_SKY_BLUE: Self = Self::rgb_hex(0x87cefa);
    pub const LIGHT_SLATE_GRAY: Self = Self::rgb_hex(0x778899);
    pub const LIGHT_SLATE_GREY: Self = Self::LIGHT_SLATE_GRAY;
    pub const LIGHT_STEEL_BLUE: Self = Self::rgb_hex(0xb0c4de);
    pub const LIGHT_YELLOW: Self = Self::rgb_hex(0xffffe0);
    pub const LIME: Self = Self::rgb_hex(0x00ff00);
    pub const LIME_GREEN: Self = Self::rgb_hex(0x32cd32);
    pub const LINEN: Self = Self::rgb_hex(0xfaf0e6);
    pub const MAGENTA: Self = Self::FUCHSIA;
    pub const MAROON: Self = Self::rgb_hex(0x800000);
    pub const MEDIUM_AQUA_MARINE: Self = Self::rgb_hex(0x66cdaa);
    pub const MEDIUM_BLUE: Self = Self::rgb_hex(0x0000cd);
    pub const MEDIUM_ORCHID: Self = Self::rgb_hex(0xba55d3);
    pub const MEDIUM_PURPLE: Self = Self::rgb_hex(0x9370db);
    pub const MEDIUM_SEA_GREEN: Self = Self::rgb_hex(0x3cb371);
    pub const MEDIUM_SLATE_BLUE: Self = Self::rgb_hex(0x7b68ee);
    pub const MEDIUM_SPRING_GREEN: Self = Self::rgb_hex(0x00fa9a);
    pub const MEDIUM_TURQUOISE: Self = Self::rgb_hex(0x48d1cc);
    pub const MEDIUM_VIOLET_RED: Self = Self::rgb_hex(0xc71585);
    pub const MIDNIGHT_BLUE: Self = Self::rgb_hex(0x191970);
    pub const MINT_CREAM: Self = Self::rgb_hex(0xf5fffa);
    pub const MISTY_ROSE: Self = Self::rgb_hex(0xffe4e1);
    pub const MOCCASIN: Self = Self::rgb_hex(0xffe4b5);
    pub const NAVAJO_WHITE: Self = Self::rgb_hex(0xffdead);
    pub const NAVY: Self = Self::rgb_hex(0x000080);
    pub const OLD_LACE: Self = Self::rgb_hex(0xfdf5e6);
    pub const OLIVE: Self = Self::rgb_hex(0x808000);
    pub const OLIVE_DRAB: Self = Self::rgb_hex(0x6b8e23);
    pub const ORANGE: Self = Self::rgb_hex(0xffa500);
    pub const ORANGE_RED: Self = Self::rgb_hex(0xff4500);
    pub const ORCHID: Self = Self::rgb_hex(0xda70d6);
    pub const PALE_GOLDEN_ROD: Self = Self::rgb_hex(0xeee8aa);
    pub const PALE_GREEN: Self = Self::rgb_hex(0x98fb98);
    pub const PALE_TURQUOISE: Self = Self::rgb_hex(0xafeeee);
    pub const PALE_VIOLET_RED: Self = Self::rgb_hex(0xdb7093);
    pub const PAPAYA_WHIP: Self = Self::rgb_hex(0xffefd5);
    pub const PEACH_PUFF: Self = Self::rgb_hex(0xffdab9);
    pub const PERU: Self = Self::rgb_hex(0xcd853f);
    pub const PINK: Self = Self::rgb_hex(0xffc0cb);
    pub const PLUM: Self = Self::rgb_hex(0xdda0dd);
    pub const POWDER_BLUE: Self = Self::rgb_hex(0xb0e0e6);
    pub const PURPLE: Self = Self::rgb_hex(0x800080);
    /// rebeccapurple: <span style="background-color: rebeccapurple; color: white">&nbsp;#663399&nbsp;</span>
    pub const REBECCA_PURPLE: Self = Self::rgb_hex(0x663399);
    pub const RED: Self = Self::rgb_hex(0xff0000);
    pub const ROSY_BROWN: Self = Self::rgb_hex(0xbc8f8f);
    pub const ROYAL_BLUE: Self = Self::rgb_hex(0x4169e1);
    pub const SADDLE_BROWN: Self = Self::rgb_hex(0x8b4513);
    pub const SALMON: Self = Self::rgb_hex(0xfa8072);
    pub const SANDY_BROWN: Self = Self::rgb_hex(0xf4a460);
    pub const SEA_GREEN: Self = Self::rgb_hex(0x2e8b57);
    pub const SEA_SHELL: Self = Self::rgb_hex(0xfff5ee);
    pub const SIENNA: Self = Self::rgb_hex(0xa0522d);
    pub const SILVER: Self = Self::rgb_hex(0xc0c0c0);
    pub const SKY_BLUE: Self = Self::rgb_hex(0x87ceeb);
    pub const SLATE_BLUE: Self = Self::rgb_hex(0x6a5acd);
    pub const SLATE_GRAY: Self = Self::rgb_hex(0x708090);
    pub const SLATE_GREY: Self = Self::SLATE_GRAY;
    pub const SNOW: Self = Self::rgb_hex(0xfffafa);
    pub const SPRING_GREEN: Self = Self::rgb_hex(0x00ff7f);
    pub const STEEL_BLUE: Self = Self::rgb_hex(0x4682b4);
    pub const TAN: Self = Self::rgb_hex(0xd2b48c);
    pub const TEAL: Self = Self::rgb_hex(0x008080);
    pub const THISTLE: Self = Self::rgb_hex(0xd8bfd8);
    pub const TOMATO: Self = Self::rgb_hex(0xff6347);
    pub const TRANSPARENT: Self = Self::rgba(0, 0, 0, 0);
    pub const TURQUOISE: Self = Self::rgb_hex(0x40e0d0);
    pub const VIOLET: Self = Self::rgb_hex(0xee82ee);
    pub const WHEAT: Self = Self::rgb_hex(0xf5deb3);
    pub const WHITE: Self = Self::rgb_hex(0xffffff);
    pub const WHITE_SMOKE: Self = Self::rgb_hex(0xf5f5f5);
    pub const YELLOW: Self = Self::rgb_hex(0xffff00);
    pub const YELLOW_GREEN: Self = Self::rgb_hex(0x9acd32);
}

impl Color {
    pub fn map_hsv<F>(self, f: F) -> Self
    where
        F: FnOnce(f32, f32, f32) -> (f32, f32, f32),
    {
        let mut hsv = HsvColor::from_color(self);
        (hsv.h, hsv.s, hsv.v) = f(hsv.h, hsv.s, hsv.v);
        hsv.to_color()
    }
}

#[derive(Debug, Clone, Copy)]
struct HsvColor {
    h: f32,
    s: f32,
    v: f32,
    a: u8,
}

impl HsvColor {
    fn from_color(c: Color) -> Self {
        let r = c.r as f32 / 255.0;
        let g = c.g as f32 / 255.0;
        let b = c.b as f32 / 255.0;
        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let n = max - min;

        let s = if max == 0.0 { 0.0 } else { n / max };
        let v = max;
        let h = if n == 0.0 {
            0.0
        } else if max == r {
            if g < b {
                6.0 + g / n - b / n
            } else {
                (g - b) / n
            }
        } else if max == g {
            2.0 + b / n - r / n
        } else {
            4.0 + r / n - g / n
        } / 6.0;

        Self {
            h: h * 360.0,
            s: s * 100.0,
            v: v * 100.0,
            a: c.a,
        }
    }

    fn to_color(mut self) -> Color {
        self.h = (self.h / 360.0) % 1.0;
        if self.h < 0.0 {
            self.h += 1.0;
        }
        self.s = (self.s / 100.0).clamp(0.0, 1.0);
        self.v = (self.v / 100.0).clamp(0.0, 1.0);

        if self.s == 0.0 {
            let v = (self.v * 255.0).round() as u8;
            return Color::rgba(v, v, v, self.a);
        }

        let mut r = self.v;
        let mut g = self.v;
        let mut b = self.v;
        let s = self.s;
        let h6 = self.h * 6.0;

        let f = h6.fract();
        match h6.floor() as usize {
            1 => {
                r *= 1.0 - s * f;
                b *= 1.0 - s;
            }
            2 => {
                r *= 1.0 - s;
                b *= 1.0 - s * (1.0 - f);
            }
            3 => {
                r *= 1.0 - s;
                g *= 1.0 - s * f;
            }
            4 => {
                r *= 1.0 - s * (1.0 - f);
                g *= 1.0 - s;
            }
            5 => {
                g *= 1.0 - s;
                b *= 1.0 - s * f;
            }
            n => {
                debug_assert!(n == 0 || n == 6, "n: {n}");
                g *= 1.0 - s * (1.0 - f);
                b *= 1.0 - s;
            }
        }

        Color::rgba(
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
            self.a,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_blend_works() {
        let black = Color::rgba(0, 0, 0, 255);
        assert_eq!(black, black.alpha_blend(black));

        let transparent = Color::rgba(0, 0, 0, 0);
        assert_eq!(transparent, transparent.alpha_blend(transparent));

        assert_eq!(black, transparent.alpha_blend(black));
        assert_eq!(black, black.alpha_blend(transparent));
    }
}
