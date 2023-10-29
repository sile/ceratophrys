use crate::{Color, Pixel, Point};

#[derive(Debug, Clone, Copy)]
pub struct Line {
    pub color: Color,
    pub position: Point,
    pub vector: Point,
}

impl Line {
    pub const fn new() -> Self {
        Self {
            color: Color::BLACK,
            position: Point::ORIGIN,
            vector: Point::ORIGIN,
        }
    }

    pub const fn color(self, color: Color) -> Self {
        Self { color, ..self }
    }

    pub const fn position(self, position: Point) -> Self {
        Self { position, ..self }
    }

    pub const fn vector(self, vector: Point) -> Self {
        Self { vector, ..self }
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
        let p0 = Point::ORIGIN;
        let p1 = self.vector;
        let dx = (p1.x - p0.x).abs() + 1;
        let dy = (p1.y - p0.y).abs() + 1;
        let sign_y = if p1.y > p0.y { 1 } else { -1 };
        let sign_x = if p1.x > p0.x { 1 } else { -1 };
        let (f, r, n, v0, sign0, mut v1, sign1) = if dx > dy {
            let f = Point::xy as fn(i16, i16) -> Point;
            let r = Rational::new(dx, dy);
            (f, r, dx, p0.x, sign_x, p0.y, sign_y)
        } else {
            let f = Point::yx as fn(i16, i16) -> Point;
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
