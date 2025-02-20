use crate::ScreenSizeError;

use winapi::shared::windef::HWND;
use winapi::um::wingdi::GetDeviceCaps;
use winapi::um::winuser::GetDC;
use winapi::um::winuser::ReleaseDC;
use winapi::um::winuser::HWND_DESKTOP;

const DESKTOPHORZRES: i32 = 118; // 原始水平分辨率
const DESKTOPVERTRES: i32 = 117; // 原始垂直分辨率

const HORZRES: i32 = 8;
const VERTRES: i32 = 10;

pub fn get_origin_screen_resolution() -> Result<(u32, u32), ScreenSizeError> {
    let origin_width: u32;
    let origin_height: u32;
    unsafe {
        let hdc = GetDC(HWND_DESKTOP as HWND);
        if hdc.is_null() {
            return Err(ScreenSizeError::NoScreen);
        }

        origin_width = GetDeviceCaps(hdc, DESKTOPHORZRES) as u32;
        origin_height = GetDeviceCaps(hdc, DESKTOPVERTRES) as u32;

        ReleaseDC(HWND_DESKTOP as HWND, hdc);
    }

    Ok((origin_width, origin_height))
}

pub fn get_scaled_screen_resolution() -> Result<(u32, u32), ScreenSizeError> {
    let width: u32;
    let height: u32;
    unsafe {
        let hdc = GetDC(HWND_DESKTOP as HWND);
        if hdc.is_null() {
            return Err(ScreenSizeError::NoScreen);
        }

        width = GetDeviceCaps(hdc, HORZRES) as u32;
        height = GetDeviceCaps(hdc, VERTRES) as u32;

        ReleaseDC(HWND_DESKTOP as HWND, hdc);
    }

    Ok((width, height))
}
