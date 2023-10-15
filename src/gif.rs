use crate::{Image, Render};
use std::time::Duration;

#[derive(Debug, Default)]
pub struct AnimatedGifImage {
    frames: Vec<Frame>,
    repeat: bool,
}

impl AnimatedGifImage {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn repeat(&mut self) -> &mut Self {
        self.repeat = true;
        self
    }

    pub fn frame<T: Render>(&mut self, image: &T, delay: Duration) -> &mut Self {
        self.frames.push(Frame {
            image: image.to_image(),
            delay,
        });
        self
    }
}

#[derive(Debug)]
struct Frame {
    image: Image,
    delay: Duration,
}
