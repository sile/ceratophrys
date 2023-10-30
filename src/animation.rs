use crate::{Entity, Image, Size};
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

    pub fn still_frame(frame: T) -> Self {
        Self {
            frames: vec![frame],
            fps: NonZeroU8::MAX,
        }
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

    pub fn fps4(self) -> Self {
        self.fps(NonZeroU8::new(4).expect("unreachable"))
    }

    pub fn fps5(self) -> Self {
        self.fps(NonZeroU8::new(5).expect("unreachable"))
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

    pub fn map_frame<F, U>(self, f: F) -> Animation<U>
    where
        F: FnMut(T) -> U,
    {
        Animation {
            frames: self.frames.into_iter().map(f).collect(),
            fps: self.fps,
        }
    }
}

impl Animation<Image> {
    pub fn get_size(&self) -> Size {
        let mut size = Size::EMPTY;
        for frame in &self.frames {
            size = size.max(frame.size());
        }
        size
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
