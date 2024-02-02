use crate::types::{ScreenInfo, WindowInfo};
use std::cmp::{max, min};
use windows::Win32::UI::WindowsAndMessaging::*;

/// Switch windows between screen 1 and 2; that is, if a window is mainly (>50%) on screen 1, move
/// it to screen 2 and vice versa
pub fn switch_windows<'a, I>(windows: I, screen1: ScreenInfo, screen2: ScreenInfo)
where
    I: IntoIterator<Item = &'a WindowInfo>,
{
    for window in windows {
        if is_on_screen(&window, &screen1) {
            switch_window(&window, &screen1, &screen2);
        } else if is_on_screen(&window, &screen2) {
            switch_window(&window, &screen2, &screen1);
        }
    }
}

fn switch_window(window: &WindowInfo, from: &ScreenInfo, to: &ScreenInfo) {
    let offset = from.left - to.left;
    let newLeft = window.left - offset;

    let is_maximized = window.window_placement.showCmd == SW_SHOWMAXIMIZED.0 as u32;
    // let is_minimized = window.window_placement.showCmd == SW_SHOWMINIMIZED.0 as u32;

    if is_maximized {
        let result = unsafe {
            SetWindowPos(
                window.handle,
                HWND_TOP,
                newLeft,
                window.top,
                0,
                0,
                SWP_NOZORDER | SWP_NOOWNERZORDER | SWP_NOACTIVATE | SWP_NOSIZE,
            )
        };

        result.unwrap_or_else(|err| {
            eprintln!("Could not reposition maximized window with title '{}'", err);
        });
    }

    let mut new_window_placement = window.window_placement.clone();
    new_window_placement.rcNormalPosition.left -= offset;
    new_window_placement.rcNormalPosition.right -= offset;

    let result = unsafe { SetWindowPlacement(window.handle, &new_window_placement) };

    result.unwrap_or_else(|err| {
        eprintln!("Could not place window with title '{}'", err);
    });
}

fn is_on_screen(window: &WindowInfo, screen: &ScreenInfo) -> bool {
    let window_width = window.right - window.left;
    let on_screen_width = min(window.right, screen.right) - max(window.left, screen.left);
    let mainly_on_screen = on_screen_width * 2 > window_width; // more than 50% on screen

    return mainly_on_screen;
}
