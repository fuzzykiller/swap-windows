use crate::types::WindowInfo;
use windows::{
    core::*, Win32::Foundation::*, Win32::Graphics::Dwm::*, Win32::UI::WindowsAndMessaging::*,
};

type WindowInfoVector = Vec<WindowInfo>;

/// Enumerate visible windows. Excludes minimized, hidden and cloaked windows as well as the legacy
/// 'Progman' window.
pub fn enumerate_windows() -> Result<WindowInfoVector> {
    let mut windows_vector: WindowInfoVector = Vec::new();
    let lparam = LPARAM(&mut windows_vector as *mut _ as _);
    unsafe {
        EnumWindows(Some(enum_window_cb), lparam)?;
    }

    Ok(windows_vector)
}

unsafe extern "system" fn enum_window_cb(window: HWND, lparam: LPARAM) -> BOOL {
    let windows_vector = &mut *(lparam.0 as *mut WindowInfoVector);

    process_window(window, windows_vector).unwrap_or_else(|err| {
        eprintln!(
            "Error enumerating window with handle {:#x}: {}",
            window.0, err
        );
    });

    true.into()
}

unsafe fn process_window(window: HWND, windows_vector: &mut WindowInfoVector) -> Result<()> {
    let title = get_window_title(window);
    let window_info = get_window_info(window)?;

    let window_placement = get_window_placement(window)?;

    let is_cloaked = get_is_cloaked(window)?;

    let window_class_name = get_window_class_name(window);

    let is_visible = window_info.dwStyle.contains(WS_VISIBLE) && !is_cloaked;
    let is_progman = window_class_name == "Progman";

    if !title.is_empty() && is_visible && !is_progman {
        windows_vector.push(WindowInfo {
            handle: window,
            title,
            left: window_info.rcWindow.left,
            top: window_info.rcWindow.top,
            right: window_info.rcWindow.right,
            bottom: window_info.rcWindow.bottom,
            window_placement,
        });
    }

    Ok(())
}

unsafe fn get_is_cloaked(window: HWND) -> Result<bool> {
    let mut cloaked_attribute_value = 0u32;
    DwmGetWindowAttribute(
        window,
        DWMWA_CLOAKED,
        &mut cloaked_attribute_value as *mut _ as _,
        core::mem::size_of::<u32>() as u32,
    )?;

    Ok(cloaked_attribute_value != 0)
}

unsafe fn get_window_class_name(window: HWND) -> String {
    let mut window_class_name: [u16; 16] = [0; 16]; // Longer class names are irrelevant here
    let len = GetClassNameW(window, &mut window_class_name);
    let window_class_name = String::from_utf16_lossy(&window_class_name[..len as usize]);

    window_class_name
}

unsafe fn get_window_placement(window: HWND) -> Result<WINDOWPLACEMENT> {
    let mut window_placement = WINDOWPLACEMENT {
        length: core::mem::size_of::<WINDOWPLACEMENT>() as u32,
        ..Default::default()
    };
    GetWindowPlacement(window, &mut window_placement)?;

    Ok(window_placement)
}

unsafe fn get_window_info(window: HWND) -> Result<WINDOWINFO> {
    let mut window_info = WINDOWINFO {
        cbSize: core::mem::size_of::<WINDOWINFO>() as u32,
        ..Default::default()
    };
    GetWindowInfo(window, &mut window_info)?;

    Ok(window_info)
}

unsafe fn get_window_title(window: HWND) -> String {
    let mut title: [u16; 512] = [0; 512];
    let len = GetWindowTextW(window, &mut title);
    let title = String::from_utf16_lossy(&title[..len as usize]);

    title
}
