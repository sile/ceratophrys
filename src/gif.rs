use crate::{Animation, Color, Image};
use std::{collections::BTreeSet, io::Write};

#[derive(Debug, Default)]
pub struct AnimatedGifImage {
    anime: Animation<Image>,
    global_palette: BTreeSet<Color>,
    repeat: bool,
}

impl AnimatedGifImage {
    pub fn new(anime: Animation<Image>) -> Self {
        Self {
            anime,
            global_palette: BTreeSet::new(),
            repeat: false,
        }
    }

    pub fn repeat(&mut self) -> &mut Self {
        self.repeat = true;
        self
    }

    pub fn write_to<W: Write>(&self, writer: W) -> Result<(), gif::EncodingError> {
        let size = self.anime.get_size();
        let mut encoder = gif::Encoder::new(
            writer,
            size.width,
            size.height,
            &self
                .global_palette
                .iter()
                .flat_map(|c| [c.r, c.g, c.b].into_iter())
                .collect::<Vec<_>>(),
        )?;

        if self.repeat {
            encoder.set_repeat(gif::Repeat::Infinite)?;
        }

        let delay = self.anime.get_frame_duration().as_millis() as u16 / 10;
        for frame in &self.anime.frames {
            let mut frame = gif::Frame::from_rgb(
                frame.size().width,
                frame.size().height,
                &frame
                    .colors()
                    .iter()
                    .flat_map(|c| [c.r, c.g, c.b].into_iter())
                    .collect::<Vec<_>>(),
            );
            frame.delay = delay;
            frame.palette = None;
            encoder.write_frame(&frame)?;
        }

        Ok(())
    }
}
