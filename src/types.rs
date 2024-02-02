use windows::Win32::Foundation::HWND;
use windows::Win32::UI::WindowsAndMessaging::WINDOWPLACEMENT;

pub struct WindowInfo {
    pub handle: HWND,
    pub title: String,
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
    pub window_placement: WINDOWPLACEMENT,
}

pub struct ScreenInfo {
    /** Left bound (inclusive) */
    pub left: i32,

    /** Right bound (exclusive) */
    pub right: i32,
}
