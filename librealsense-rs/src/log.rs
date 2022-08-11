// License: MIT. See LICENSE file in root directory.
// Copyright(c) 2019 Aivero. All Rights Reserved.
use crate::error::Error;

// Expose `rs2_log_severity` for external use.
pub use rs2::rs2_log_severity;

/// Determine the level of severity to be logged to console.
///
/// # Returns
/// * `Ok()` on success.
/// * `Err(Error)` on failure.
pub fn log_to_console(min_severity: rs2_log_severity) -> Result<(), Error> {
    Error::call1(rs2::rs2_log_to_console, min_severity)
}

/// Select a file for logging and determine the level of severity to be logged inside such file.
///
/// # Returns
/// * `Ok()` on success.
/// * `Err(Error)` on failure.
#[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
pub fn log_to_file(min_severity: rs2_log_severity, file_path: &str) -> Result<(), Error> {
    let s = std::ffi::CString::new(file_path).expect("Failed to create CString");
    Error::call2(rs2::rs2_log_to_file, min_severity, s.as_ptr() as *const i8)
}

/// Select a file for logging and determine the level of severity to be logged inside such file.
///
/// # Returns
/// * `Ok()` on success.
/// * `Err(Error)` on failure.
#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub fn log_to_file(min_severity: rs2_log_severity, file_path: &str) -> Result<(), Error> {
    let s = std::ffi::CString::new(file_path).expect("Failed to create CString");
    Error::call2(rs2::rs2_log_to_file, min_severity, s.as_ptr() as *const u8)
}
