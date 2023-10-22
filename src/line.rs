use crate::{Canvas, Color, Point, Render};

#[derive(Debug, Clone, Copy)]
pub struct Line {
    point: Point,
    color: Color,
}

impl Line {
    pub const fn new(color: Color, point: Point) -> Self {
        Self { point, color }
    }

    pub fn points(self) -> impl Iterator<Item = Point> {
        let p0 = Point::ORIGIN;
        let p1 = self.point;
        let dx = (p1.x - p0.x).abs() + 1;
        let dy = (p1.y - p0.y).abs() + 1;
        let sign_y = if p1.y > p0.y { 1 } else { -1 };
        let sign_x = if p1.x > p0.x { 1 } else { -1 };
        let (f, r, n, v0, sign0, mut v1, sign1) = if dx > dy {
            let f = xy as fn(i16, i16) -> Point;
            let r = Rational::new(dx, dy);
            (f, r, dx, p0.x, sign_x, p0.y, sign_y)
        } else {
            let f = yx as fn(i16, i16) -> Point;
            let r = Rational::new(dy, dx);
            (f, r, dy, p0.y, sign_y, p0.x, sign_x)
        };
        (0..n).map(move |i| {
            if i != 0 && (i - 1) / r != i / r {
                v1 += sign1;
            }
            f(v0 + i * sign0, v1)
        })
    }
}

impl Render for Line {
    fn render(&self, point: Point, canvas: &mut Canvas) {
        for p in self.points() {
            canvas.set_pixel(point + p, self.color);
        }
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

fn xy(x: i16, y: i16) -> Point {
    Point::new(x, y)
}

fn yx(y: i16, x: i16) -> Point {
    Point::new(x, y)
}
