use std::collections::BTreeMap;

pub mod bmp;

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Image {
    size: Size,
    pixels: Vec<Color>,
}

impl Image {
    pub fn new(size: Size, pixels: Vec<Color>) -> Option<Self> {
        (size.area() == pixels.len() as u32).then(|| Self { size, pixels })
    }

    pub fn size(&self) -> Size {
        self.size
    }

    pub fn pixels(&self) -> &[Color] {
        &self.pixels
    }
}

impl Render for Image {
    fn render(&self, point: Point, canvas: &mut Canvas) {
        for (p, c) in self.size().points().zip(self.pixels.iter().copied()) {
            canvas.set_pixel(point + p, c);
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
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

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Palette {
    colors: BTreeMap<char, Color>,
}

impl Palette {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn set_color(&mut self, ch: char, color: Color) -> &mut Self {
        self.colors.insert(ch, color);
        self
    }

    pub fn get_color(&self, ch: char) -> Option<Color> {
        self.colors.get(&ch).copied()
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq, Hash)]
pub struct Canvas {
    pixels: BTreeMap<Point, Color>,
    start_point: Point,
    end_point: Point,
    background_color: Color,
}

impl Canvas {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn pixels(&self) -> impl '_ + Iterator<Item = (Point, Color)> {
        self.pixels.iter().map(|(&point, &color)| (point, color))
    }

    pub fn set_pixel(&mut self, point: Point, color: Color) {
        self.start_point.x = self.start_point.x.min(point.x);
        self.start_point.y = self.start_point.y.min(point.y);

        self.end_point.x = self.end_point.x.max(point.x + 1);
        self.end_point.y = self.end_point.y.max(point.y + 1);

        self.pixels.insert(point, color);
    }

    pub fn start_point(&self) -> Point {
        self.start_point
    }

    pub fn end_point(&self) -> Point {
        self.end_point
    }

    pub fn size(&self) -> Size {
        Size::new(
            self.end_point.x.saturating_sub(self.start_point.x) as u16,
            self.end_point.y.saturating_sub(self.start_point.y) as u16,
        )
    }

    pub fn to_image(&self) -> Image {
        let start_point = self.start_point();
        let size = self.size();
        let mut pixels = vec![self.background_color; size.area() as usize];
        for (point, color) in self.pixels() {
            let point = point - start_point;
            let index = point.y as usize * size.width as usize + point.x as usize;
            pixels[index] = color.alpha_blend(pixels[index]);
        }
        Image::new(size, pixels).expect("unreachable")
    }
}

pub trait Render {
    fn render(&self, point: Point, canvas: &mut Canvas);

    fn to_image(&self) -> Image {
        let mut canvas = Canvas::new();
        self.render(Point::ORIGIN, &mut canvas);
        canvas.to_image()
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

impl Size {
    pub const fn new(width: u16, height: u16) -> Self {
        Self { width, height }
    }

    pub fn area(self) -> u32 {
        u32::from(self.width) * u32::from(self.height)
    }

    pub fn points(self) -> impl Iterator<Item = Point> {
        (0..self.height)
            .flat_map(move |y| (0..self.width).map(move |x| Point::new(x as i16, y as i16)))
    }

    pub const fn is_empty(self) -> bool {
        self.width == 0 || self.height == 0
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: i16,
    pub y: i16,
}

impl Point {
    pub const ORIGIN: Self = Self::new(0, 0);

    pub const fn new(x: i16, y: i16) -> Self {
        Self { x, y }
    }

    pub const fn move_x(self, x: i16) -> Self {
        Self::new(self.x.saturating_add(x), self.y)
    }

    pub const fn move_y(self, y: i16) -> Self {
        Self::new(self.x, self.y.saturating_add(y))
    }

    pub const fn move_xy(self, x: i16, y: i16) -> Self {
        Self::new(self.x.saturating_add(x), self.y.saturating_add(y))
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.y.cmp(&other.y).then(self.x.cmp(&other.x))
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, Self { x, y }: Self) -> Self::Output {
        Self::new(self.x.saturating_add(x), self.y.saturating_add(y))
    }
}

impl std::ops::Sub for Point {
    type Output = Self;

    fn sub(self, Self { x, y }: Self) -> Self::Output {
        Self::new(self.x.saturating_sub(x), self.y.saturating_sub(y))
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
