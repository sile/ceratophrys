#[derive(Debug, Clone, Copy)]
pub struct Silhouette<T> {
    pub image: T,
}

impl<T> Silhouette<T> {
    pub const fn new(image: T) -> Self {
        Self { image }
    }
}

// impl<T: Render> Render for Silhouette<T> {
//     fn render(&self, offset: Point, canvas: &mut Canvas) {
//         let image = self.image.to_image();
//         for (point, color) in image.pixels() {
//             if color.is_transparent() {
//                 continue;
//             }
//             canvas.set_pixel(point + offset, Color::rgb_hex(0x000000));
//         }
//     }
// }
