use crate::ScreenSizeError;
use std::convert::TryInto;
use std::ptr::null;
use x11::xlib;

pub fn display_size() -> Result<(u64, u64), ScreenSizeError> {
    unsafe {
        let display = xlib::XOpenDisplay(null());
        if display.is_null() {
            return Err(ScreenSizeError::NoScreen);
        }
        let screen_ptr = xlib::XDefaultScreenOfDisplay(display);
        if screen_ptr.is_null() {
            return Err(ScreenSizeError::NoScreen);
        }
        let screen = *screen_ptr;
        Ok((
            screen
                .width
                .try_into()
                .map_err(ScreenSizeError::InvalidSize)?,
            screen
                .height
                .try_into()
                .map_err(ScreenSizeError::InvalidSize)?,
        ))
    }
}
