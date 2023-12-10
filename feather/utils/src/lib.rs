//! Assorted utilities not directly related to Minecraft/Feather.

/// Swap-removes an item from a vector by equality.
pub fn vec_remove_item<T: PartialEq>(vec: &mut Vec<T>, item: &T) {
    let index = vec.iter().position(|x| x == item);
    if let Some(index) = index {
        vec.swap_remove(index);
    }
}

/// Enables ANSI code support on Windows 10
/// Returns an [`io:Error`](std::io::Error) with the Windows error code in it if unsuccessful
/// On non-Windows platforms, this is a no-op that always returns `Ok(())`.
/// Call `enable_ansi_support` *once*, early on in `main()`
#[cfg(windows)]
pub fn enable_ansi_support() -> Result<(), std::io::Error> {
    use std::{ffi::OsStr, iter::once, os::windows::ffi::OsStrExt};

    use windows_sys::Win32::{
        Foundation::INVALID_HANDLE_VALUE,
        Storage::FileSystem::{
            CreateFileW, FILE_GENERIC_READ, FILE_GENERIC_WRITE, FILE_SHARE_WRITE, OPEN_EXISTING,
        },
        System::Console::{GetConsoleMode, SetConsoleMode, ENABLE_VIRTUAL_TERMINAL_PROCESSING},
    };

    unsafe {
        // ref: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilew
        // Using `CreateFileW("CONOUT$", ...)` to retrieve the console handle works correctly even if STDOUT and/or STDERR are redirected
        let console_out_name: Vec<u16> =
            OsStr::new("CONOUT$").encode_wide().chain(once(0)).collect();
        let console_handle = CreateFileW(
            console_out_name.as_ptr(),
            FILE_GENERIC_READ | FILE_GENERIC_WRITE,
            FILE_SHARE_WRITE,
            std::ptr::null(),
            OPEN_EXISTING,
            0,
            0,
        );
        if console_handle == INVALID_HANDLE_VALUE {
            return Err(std::io::Error::last_os_error());
        }

        // ref: https://docs.microsoft.com/en-us/windows/console/getconsolemode
        let mut console_mode = 0;
        if 0 == GetConsoleMode(console_handle, &mut console_mode) {
            return Err(std::io::Error::last_os_error());
        }

        // VT processing not already enabled?
        if console_mode & ENABLE_VIRTUAL_TERMINAL_PROCESSING == 0 {
            // https://docs.microsoft.com/en-us/windows/console/setconsolemode
            if 0 == SetConsoleMode(
                console_handle,
                console_mode | ENABLE_VIRTUAL_TERMINAL_PROCESSING,
            ) {
                return Err(std::io::Error::last_os_error());
            }
        }
    }

    Ok(())
}

/// Enables ANSI code support on Windows 10.
///
/// Returns an [`io::Error`](std::io::Error) with the Windows error code in it if unsuccessful.
///
/// On non-Windows platforms, this is a no-op that always returns `Ok(())`.
///
/// # Examples
///
/// See the [crate documentation](crate).
#[cfg(not(windows))]
#[inline]
pub fn enable_ansi_support() -> Result<(), std::io::Error> {
    Ok(())
}
