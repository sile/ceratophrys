use orfail::OrFail;
use piac::{bmp::BmpImage, Color, Palette, TextImage};

fn main() -> orfail::Result<()> {
    let palette = Palette::new()
        .set_color(' ', Color::rgb(255, 255, 255))
        .set_color('o', Color::rgb(0, 0, 0))
        .clone();
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

    BmpImage::new(&cat).write_to(std::io::stdout()).or_fail()?;

    Ok(())
}
