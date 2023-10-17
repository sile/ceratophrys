use crate::{Canvas, Point, Render};

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

    pub const fn to_rgba(self) -> (u8, u8, u8, u8) {
        (self.r, self.g, self.b, self.a)
    }

    pub fn alpha_blend(self, dst: Self) -> Self {
        if dst.a == 0 {
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

impl Render for Color {
    fn render(&self, point: Point, canvas: &mut Canvas) {
        canvas.set_pixel(point, *self);
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
