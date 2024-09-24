use std::num::TryFromIntError;
use thiserror::Error;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "macos")]
mod macos;
#[cfg(target_os = "windows")]
mod windows;

#[cfg(target_os = "linux")]
use linux::display_size as display_size_impl;
#[cfg(target_os = "macos")]
use macos::display_size as display_size_impl;
#[cfg(target_os = "windows")]
use windows::display_size as display_size_impl;

#[derive(Debug, Error)]
pub enum ScreenSizeError {
    #[error("Screen size is invalid: {0}")]
    InvalidSize(#[source] TryFromIntError), //windows or linux only
    #[error("No screen found")]
    NoScreen, //linux only
}
/// Return the width and height in pixels of the primary screen/monitor
///
/// # Platform notes
/// ## Windows
/// This will return a scaled value if your program isn't DPI aware, so if the screen is 1000x1000 and scaled 200% this will return (500,500)
///
/// ## MacOS
/// Will panic if there's no monitor available
#[inline(always)]
pub fn get_primary_screen_size() -> Result<(u64, u64), ScreenSizeError> {
    display_size_impl()
}
