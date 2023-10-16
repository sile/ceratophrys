use crate::{Color, Image, Render, Size};
use std::{collections::BTreeSet, io::Write, time::Duration};

#[derive(Debug, Default)]
pub struct AnimatedGifImage {
    frames: Vec<Frame>,
    size: Size,
    global_palette: BTreeSet<Color>,
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
        let image = image.to_image();

        self.size.width = self.size.width.max(image.size().width);
        self.size.height = self.size.height.max(image.size().height);

        self.global_palette.extend(image.pixels());

        self.frames.push(Frame { image, delay });
        self
    }

    pub fn write_to<W: Write>(&self, writer: W) -> Result<(), gif::EncodingError> {
        let mut encoder = gif::Encoder::new(
            writer,
            self.size.width,
            self.size.height,
            &self
                .global_palette
                .iter()
                .flat_map(|c| [c.r, c.g, c.b].into_iter())
                .collect::<Vec<_>>(),
        )?;

        if self.repeat {
            encoder.set_repeat(gif::Repeat::Infinite)?;
        }

        for frame in &self.frames {
            let delay = frame.delay.as_millis() as u16 / 10;
            let mut frame = gif::Frame::from_rgb(
                self.size.width,
                self.size.height,
                &frame
                    .image
                    .rows()
                    .rev()
                    .flat_map(|row| row.iter().flat_map(|c| [c.r, c.g, c.b].into_iter()))
                    .collect::<Vec<_>>(),
            );
            frame.delay = delay;
            encoder.write_frame(&frame)?;
        }

        Ok(())
    }
}

#[derive(Debug)]
struct Frame {
    image: Image,
    delay: Duration,
}
