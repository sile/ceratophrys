use ceratophrys::{bmp::BmpImage, Color, Palette, Render, TextImage};
use orfail::OrFail;

fn main() -> orfail::Result<()> {
    let palette = Palette::new()
        .color(' ', Color::rgb(255, 255, 255))
        .color('o', Color::rgb(0, 0, 0));
    let cat = TextImage::new(
        palette,
        concat!(
            "                 \n", //
            "   o   o         \n", //
            "  ooo ooo        \n", //
            " ooooooooo  oo   \n", //
            " oo ooo oo   oo  \n", //
            " ooooooooo    oo \n", //
            "  ooooooo     oo \n", //
            " ooooooooo    oo \n", //
            " oooooooooo  oo  \n", //
            " oooooooooooooo  \n", //
            "  oooo ooooo     \n", //
            "                 \n", //
        ),
    )
    .or_fail()?;

    BmpImage::new(cat.to_image())
        .write_to(std::io::stdout())
        .or_fail()?;

    Ok(())
}
