use std::mem::size_of;
use windows::{core::*, Win32::Foundation::*, Win32::Security::*, Win32::System::Threading::*};

/// Check whether current process' token is an elevated token
pub fn is_elevated() -> Result<bool> {
    let mut token_elevation = TOKEN_ELEVATION::default();
    let mut process_token = HANDLE::default();
    let mut result_size = 0;

    unsafe {
        OpenProcessToken(GetCurrentProcess(), TOKEN_QUERY, &mut process_token)?;

        GetTokenInformation(
            process_token,
            TokenElevation,
            Some(&mut token_elevation as *mut _ as *mut _),
            size_of::<TOKEN_ELEVATION>() as u32,
            &mut result_size,
        )?;
    }

    Ok(token_elevation.TokenIsElevated != 0)
}
