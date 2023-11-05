use crate::{Color, Image, Pixel, Position};

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub color: Color,
    pub position: Position,
    pub vector: Position,
}

impl Line {
    pub const fn new() -> Self {
        Self {
            color: Color::BLACK,
            position: Position::ORIGIN,
            vector: Position::ORIGIN,
        }
    }

    pub const fn color(self, color: Color) -> Self {
        Self { color, ..self }
    }

    pub const fn position(self, position: Position) -> Self {
        Self { position, ..self }
    }

    pub const fn vector(self, vector: Position) -> Self {
        Self { vector, ..self }
    }

    pub fn to_image(self) -> Image {
        self.into_iter().collect()
    }
}

impl Default for Line {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for Line {
    type Item = Pixel;
    type IntoIter = Box<dyn Iterator<Item = Pixel>>;

    fn into_iter(self) -> Self::IntoIter {
        let p0 = Position::ORIGIN;
        let p1 = self.vector;
        let dx = (p1.x - p0.x).abs() + 1;
        let dy = (p1.y - p0.y).abs() + 1;
        let sign_y = if p1.y > p0.y { 1 } else { -1 };
        let sign_x = if p1.x > p0.x { 1 } else { -1 };
        let (f, r, n, v0, sign0, mut v1, sign1) = if dx > dy {
            let f = Position::xy as fn(i16, i16) -> Position;
            let r = Rational::new(dx, dy);
            (f, r, dx, p0.x, sign_x, p0.y, sign_y)
        } else {
            let f = Position::yx as fn(i16, i16) -> Position;
            let r = Rational::new(dy, dx);
            (f, r, dy, p0.y, sign_y, p0.x, sign_x)
        };
        Box::new((0..n).map(move |i| {
            if i != 0 && (i - 1) / r != i / r {
                v1 += sign1;
            }
            Pixel::new(f(v0 + i * sign0, v1) + self.position, self.color)
        }))
    }
}

#[derive(Debug, Clone, Copy)]
struct Rational {
    num: i16,
    den: i16,
}

impl Rational {
    const fn new(num: i16, den: i16) -> Self {
        Self { num, den }
    }
}

impl std::ops::Div<Rational> for i16 {
    type Output = i16;

    fn div(self, rhs: Rational) -> Self::Output {
        self * rhs.den / rhs.num
    }
}
