use orfail::OrFail;
use piac::{gif::AnimatedGifImage, Color, Palette, TextImage};
use std::time::Duration;

fn main() -> orfail::Result<()> {
    let palette = Palette::new()
        .set_color(' ', Color::rgb(255, 255, 255))
        .set_color('o', Color::rgb(0, 0, 0))
        .clone();
    let cat0 = TextImage::new(
        palette.clone(),
        concat!(
            "                   \n", //
            "   o   o           \n", //
            "  ooo ooo          \n", //
            " ooooooooo  oo     \n", //
            " oo ooo oo   oo    \n", //
            " ooooooooo    oo   \n", //
            "  ooooooo     oo   \n", //
            " ooooooooo    oo   \n", //
            " oooooooooo  oo    \n", //
            " oooooooooooooo    \n", //
            "  oooo ooooo       \n", //
            "                   \n", //
        ),
    )
    .or_fail()?;
    let cat1 = TextImage::new(
        palette,
        concat!(
            "                   \n", //
            "   o   o           \n", //
            "  ooo ooo          \n", //
            " ooooooooo      oo \n", //
            " oo ooo oo     oo  \n", //
            " ooooooooo    oo   \n", //
            "  ooooooo     oo   \n", //
            " ooooooooo    oo   \n", //
            " oooooooooo  oo    \n", //
            " oooooooooooooo    \n", //
            "  oooo ooooo       \n", //
            "                   \n", //
        ),
    )
    .or_fail()?;

    let delay = Duration::from_millis(200);
    AnimatedGifImage::new()
        .repeat()
        .frame(&cat0, delay)
        .frame(&cat1, delay)
        .write_to(std::io::stdout())
        .or_fail()?;

    Ok(())
}
