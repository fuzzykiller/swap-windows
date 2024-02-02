#![allow(non_snake_case)]

mod query_elevation;
mod switch_windows;
mod types;
mod window_enumerator;

use crate::query_elevation::is_elevated;
use crate::switch_windows::switch_windows;
use crate::types::ScreenInfo;
use crate::window_enumerator::enumerate_windows;

fn main() {
    let is_elevated = is_elevated().unwrap();

    if !is_elevated {
        eprintln!("Not elevated, continuing anyway. May not be able to swap some windows.");
    }

    let windows = enumerate_windows().unwrap();

    let leftMainScreen = ScreenInfo {
        left: 0,
        right: 2560,
    };

    let rightMainScreen = ScreenInfo {
        left: 2560,
        right: 2560 * 2,
    };

    switch_windows(windows.iter(), leftMainScreen, rightMainScreen);
}
