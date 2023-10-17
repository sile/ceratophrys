use crate::{Canvas, Palette, Point, Render};

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct TextImage {
    palette: Palette,
    text: String,
}

impl TextImage {
    pub fn new(palette: Palette, text: impl Into<String>) -> Option<Self> {
        let text = text.into();
        text.chars()
            .all(|ch| ch == '\n' || palette.get_color(ch).is_some())
            .then(|| Self { palette, text })
    }
}

impl Render for TextImage {
    fn render(&self, point: Point, canvas: &mut Canvas) {
        for (y, line) in self.text.lines().enumerate() {
            for (x, ch) in line.chars().enumerate() {
                let point = point.move_xy(x as i16, -(y as i16));
                let color = self.palette.get_color(ch).expect("unreachable");
                canvas.set_pixel(point, color);
            }
        }
    }
}
