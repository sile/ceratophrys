use crate::{Canvas, Color, Line, Point, Render};

#[derive(Debug, Clone)]
pub struct Polygon {
    vertices: Vec<Point>,
    color: Color,
}

impl Polygon {
    pub fn new(color: Color) -> Self {
        Self {
            vertices: Vec::new(),
            color,
        }
    }

    pub fn vertex(mut self, point: Point) -> Self {
        self.vertices.push(point);
        self
    }

    pub fn vertices(mut self, points: impl Iterator<Item = Point>) -> Self {
        self.vertices.extend(points);
        self
    }

    pub fn points(&self) -> impl Iterator<Item = Point> {
        let mut points = Vec::new();
        let start_points = std::iter::once(Point::ORIGIN).chain(self.vertices.iter().copied());
        let end_points = self
            .vertices
            .iter()
            .copied()
            .chain(std::iter::once(Point::ORIGIN));
        for (p0, p1) in start_points.zip(end_points) {
            points.extend(Line::new(self.color, p1 - p0).points().map(|p| p + p0));
        }
        points.sort();
        points.dedup();
        points.into_iter()
    }
}

impl Render for Polygon {
    fn render(&self, point: Point, canvas: &mut Canvas) {
        for p in self.points() {
            canvas.draw_pixel(point + p, self.color);
        }
    }
}
