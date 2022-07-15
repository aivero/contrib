// License: MIT. See LICENSE file in root directory.
// Copyright(c) 2019 Aivero. All Rights Reserved.
use crate::low_level_utils::cstring_to_string;
use std::{error, fmt};

/// Struct representation of an [`Error`](../error/struct.Error.html) that wraps around
/// `rs2_error` handle.
#[derive(Debug)]
pub struct Error {
    pub(crate) handle: *mut rs2::rs2_error,
}

/// Safe releasing of the `rs2_error` handle.
impl Drop for Error {
    fn drop(&mut self) {
        unsafe {
            rs2::rs2_free_error(self.handle);
        }
    }
}

/// Default constructor of [`Error`](../error/struct.Error.html) that contains no error.
impl Default for Error {
    fn default() -> Self {
        Self {
            handle: std::ptr::null_mut::<rs2::rs2_error>(),
        }
    }
}

/// Once returned from librealsense, `Error` is an immutable api. It should therefor be completely
/// safe to share this struct between threads.
unsafe impl Sync for Error {}
unsafe impl Send for Error {}

/// Define the source of [`Error`](../error/struct.Error.html).
impl error::Error for Error {}

/// Formatting of [`Error`](../error/struct.Error.html).
impl fmt::Display for Error {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str(self.get_message().as_str())
    }
}

impl Error {
    /// Create a new [`Error`](../error/struct.Error.html).
    ///
    /// # Arguments
    /// * `message` - Descriptive error message
    /// * `function` - The function that caused the error
    /// * `args` - The argument that caused the error
    /// * `exception_type` - The categorty of error
    ///
    /// # Returns
    /// * New [`Error`](../error/struct.Error.html)
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub(crate) fn new(
        message: &str,
        function: &str,
        args: &str,
        exception_type: rs2::rs2_exception_type,
    ) -> Self {
        Self {
            handle: unsafe {
                rs2::rs2_create_error(
                    message.as_ptr() as *const i8,
                    function.as_ptr() as *const i8,
                    args.as_ptr() as *const i8,
                    exception_type,
                )
            },
        }
    }

    /// Create a new [`Error`](../error/struct.Error.html).
    ///
    /// # Arguments
    /// * `message` - Descriptive error message
    /// * `function` - The function that caused the error
    /// * `args` - The argument that caused the error
    /// * `exception_type` - The categorty of error
    ///
    /// # Returns
    /// * New [`Error`](../error/struct.Error.html)
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    pub(crate) fn new(
        message: &str,
        function: &str,
        args: &str,
        exception_type: rs2::rs2_exception_type,
    ) -> Self {
        Self {
            handle: unsafe {
                rs2::rs2_create_error(
                    message.as_ptr() as *const u8,
                    function.as_ptr() as *const u8,
                    args.as_ptr() as *const u8,
                    exception_type,
                )
            },
        }
    }

    /// Return `*mut *mut rs2::rs2_error` handle required by other functions of the API.
    pub(crate) fn inner(&mut self) -> *mut *mut rs2::rs2_error {
        &mut self.handle as *mut *mut rs2::rs2_error
    }

    /// Check the value of [`Error`](../error/struct.Error.html).
    ///
    /// # Returns
    /// * A Result that can be bubbled up if the error is not null.
    pub fn check(self) -> Result<(), Error> {
        if self.handle.is_null() {
            Ok(())
        } else {
            Err(self)
        }
    }

    /// Return the message of the error.
    pub fn get_message(&self) -> String {
        unsafe { cstring_to_string(rs2::rs2_get_error_message(self.handle)) }
    }

    /// Return the function in which the error occured.
    pub fn get_function(&self) -> String {
        unsafe { cstring_to_string(rs2::rs2_get_failed_function(self.handle)) }
    }

    /// Return what arguments caused the error.
    pub fn get_args(&self) -> String {
        unsafe { cstring_to_string(rs2::rs2_get_failed_args(self.handle)) }
    }
}
