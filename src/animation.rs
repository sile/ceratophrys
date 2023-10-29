use crate::Entity;
use std::{num::NonZeroU8, time::Duration};

#[derive(Debug, Clone)]
pub struct Animation<T = Entity> {
    pub frames: Vec<T>,
    pub fps: NonZeroU8,
}

impl<T> Animation<T> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn frame(mut self, frame: T) -> Self {
        self.frames.push(frame);
        self
    }

    pub fn frames(mut self, frames: impl IntoIterator<Item = T>) -> Self {
        self.frames.extend(frames);
        self
    }

    pub fn fps(mut self, fps: NonZeroU8) -> Self {
        self.fps = fps;
        self
    }

    pub fn fps10(self) -> Self {
        self.fps(NonZeroU8::new(10).expect("unreachable"))
    }

    pub fn fps12(self) -> Self {
        self.fps(NonZeroU8::new(12).expect("unreachable"))
    }

    pub fn fps15(self) -> Self {
        self.fps(NonZeroU8::new(15).expect("unreachable"))
    }

    pub fn fps20(self) -> Self {
        self.fps(NonZeroU8::new(20).expect("unreachable"))
    }

    pub fn fps24(self) -> Self {
        self.fps(NonZeroU8::new(24).expect("unreachable"))
    }

    pub fn fps30(self) -> Self {
        self.fps(NonZeroU8::new(30).expect("unreachable"))
    }

    pub fn fps60(self) -> Self {
        self.fps(NonZeroU8::new(60).expect("unreachable"))
    }

    pub fn get_frame_duration(&self) -> Duration {
        Duration::from_secs(1) / u32::from(self.fps.get())
    }

    pub fn get_duration(&self) -> Duration {
        Duration::from_secs(self.frames.len() as u64) / u32::from(self.fps.get())
    }

    pub fn get_nth_frame_time(&self, n: usize) -> Duration {
        Duration::from_secs(n as u64) / u32::from(self.fps.get())
    }
}

impl<T> Default for Animation<T> {
    fn default() -> Self {
        Self {
            frames: Vec::new(),
            fps: NonZeroU8::new(1).expect("unreachable"),
        }
    }
}
