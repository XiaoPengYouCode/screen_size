use crate::ScreenSizeError;
use winapi::um::winuser::{GetSystemMetrics, SM_CXSCREEN, SM_CYSCREEN};

pub fn display_size() -> Result<(u64, u64), ScreenSizeError> {
    let w = unsafe {
        GetSystemMetrics(SM_CXSCREEN)
            .try_into()
            .map_err(ScreenSizeError::InvalidSize)?
    };
    let h = unsafe {
        GetSystemMetrics(SM_CYSCREEN)
            .try_into()
            .map_err(ScreenSizeError::InvalidSize)?
    };
    Ok((w, h))
}
