#![allow(clippy::deprecated_semver)]

// 引入 vibrancy 模块
pub mod vibrancy;

pub use vibrancy::macos::{NSVisualEffectMaterial, NSVisualEffectState};

/// a tuple of RGBA colors. Each value has minimum of 0 and maximum of 255.
pub type Color = (u8, u8, u8, u8);

pub fn apply_blur(
    window: impl raw_window_handle::HasWindowHandle,
    #[allow(unused)] color: Option<Color>,
) -> Result<(), Error> {
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            vibrancy::windows::apply_blur(handle.hwnd.get() as _, color)
        }
        _ => Err(Error::UnsupportedPlatform(
            "\"apply_blur()\" is only supported on Windows.",
        )),
    }
}

pub fn clear_blur(window: impl raw_window_handle::HasWindowHandle) -> Result<(), Error> {
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            vibrancy::windows::clear_blur(handle.hwnd.get() as _)
        }
        _ => Err(Error::UnsupportedPlatform(
            "\"clear_blur()\" is only supported on Windows.",
        )),
    }
}

pub fn apply_acrylic(
    window: impl raw_window_handle::HasWindowHandle,
    #[allow(unused)] color: Option<Color>,
) -> Result<(), Error> {
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            vibrancy::windows::apply_acrylic(handle.hwnd.get() as _, color)
        }
        _ => Err(Error::UnsupportedPlatform(
            "\"apply_acrylic()\" is only supported on Windows.",
        )),
    }
}

pub fn clear_acrylic(window: impl raw_window_handle::HasWindowHandle) -> Result<(), Error> {
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            vibrancy::windows::clear_acrylic(handle.hwnd.get() as _)
        }
        _ => Err(Error::UnsupportedPlatform(
            "\"clear_acrylic()\" is only supported on Windows.",
        )),
    }
}

pub fn apply_mica(
    window: impl raw_window_handle::HasWindowHandle,
    dark: Option<bool>,
) -> Result<(), Error> {
    #[cfg(not(target_os = "windows"))]
    let _ = dark;
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            vibrancy::windows::apply_mica(handle.hwnd.get() as _, dark)
        }
        _ => Err(Error::UnsupportedPlatform(
            "\"apply_mica()\" is only supported on Windows.",
        )),
    }
}

pub fn clear_mica(window: impl raw_window_handle::HasWindowHandle) -> Result<(), Error> {
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            vibrancy::windows::clear_mica(handle.hwnd.get() as _)
        }
        _ => Err(Error::UnsupportedPlatform(
            "\"clear_mica()\" is only supported on Windows.",
        )),
    }
}

pub fn apply_tabbed(
    window: impl raw_window_handle::HasWindowHandle,
    dark: Option<bool>,
) -> Result<(), Error> {
    #[cfg(not(target_os = "windows"))]
    let _ = dark;
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            vibrancy::windows::apply_tabbed(handle.hwnd.get() as _, dark)
        }
        _ => Err(Error::UnsupportedPlatform(
            "\"apply_tabbed()\" is only supported on Windows.",
        )),
    }
}

pub fn clear_tabbed(window: impl raw_window_handle::HasWindowHandle) -> Result<(), Error> {
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "windows")]
        raw_window_handle::RawWindowHandle::Win32(handle) => {
            vibrancy::windows::clear_tabbed(handle.hwnd.get() as _)
        }
        _ => Err(Error::UnsupportedPlatform(
            "\"clear_tabbed()\" is only supported on Windows.",
        )),
    }
}

pub fn apply_vibrancy(
    window: impl raw_window_handle::HasWindowHandle,
    #[allow(unused)] effect: NSVisualEffectMaterial,
    #[allow(unused)] state: Option<NSVisualEffectState>,
    #[allow(unused)] radius: Option<f64>,
) -> Result<(), Error> {
    match window.window_handle()?.as_raw() {
        #[cfg(target_os = "macos")]
        raw_window_handle::RawWindowHandle::AppKit(handle) => unsafe {
            vibrancy::macos::apply_vibrancy(handle.ns_view, effect, state, radius)
        },
        _ => Err(Error::UnsupportedPlatform(
            "\"apply_vibrancy()\" is only supported on macOS.",
        )),
    }
}

#[derive(Debug)]
pub enum Error {
    UnsupportedPlatform(&'static str),
    UnsupportedPlatformVersion(&'static str),
    NotMainThread(&'static str),
    NoWindowHandle(raw_window_handle::HandleError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::UnsupportedPlatform(e)
            | Error::UnsupportedPlatformVersion(e)
            | Error::NotMainThread(e) => {
                write!(f, "{}", e)
            }
            Error::NoWindowHandle(e) => {
                write!(f, "{}", e)
            }
        }
    }
}

impl std::error::Error for Error {}

impl From<raw_window_handle::HandleError> for Error {
    fn from(err: raw_window_handle::HandleError) -> Self {
        Error::NoWindowHandle(err)
    }
}
