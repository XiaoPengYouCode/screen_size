use crate::ScreenSizeError;
use core_graphics::display::CGDisplay;

pub fn display_size() -> Result<(u64, u64), ScreenSizeError> {
    let main = CGDisplay::main();
    Ok((main.pixels_wide(), main.pixels_high()))
}
