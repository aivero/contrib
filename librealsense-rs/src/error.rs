use crate::low_level_utils::cstring_to_string;
use std::{error, fmt};

/// Struct representation of an [`Error`](../error/struct.Error.html) that wraps around
/// `rs2_error` handle.
#[derive(Debug)]
pub struct Error(pub(crate) *mut rs2::rs2_error);

/// Safe releasing of the `rs2_error` handle.
impl Drop for Error {
    fn drop(&mut self) {
        unsafe { rs2::rs2_free_error(self.0) }
    }
}

/// Default constructor of [`Error`](../error/struct.Error.html) that contains no error.
impl Default for Error {
    fn default() -> Self {
        Self(std::ptr::null_mut::<rs2::rs2_error>())
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
    ///
    /// # Returns
    /// * New [`Error`](../error/struct.Error.html)
    #[cfg(any(target_arch = "x86", target_arch = "x86_64"))]
    pub fn new(message: &str, function: &str, args: &str) -> Self {
        Self(unsafe {
            rs2::rs2_create_error(
                message.as_ptr() as *const i8,
                function.as_ptr() as *const i8,
                args.as_ptr() as *const i8,
                rs2::rs2_exception_type_RS2_EXCEPTION_TYPE_UNKNOWN,
            )
        })
    }

    /// Create a new [`Error`](../error/struct.Error.html).
    ///
    /// # Arguments
    /// * `message` - Descriptive error message
    /// * `function` - The function that caused the error
    /// * `args` - The argument that caused the error
    ///
    /// # Returns
    /// * New [`Error`](../error/struct.Error.html)
    #[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
    pub fn new(message: &str, function: &str, args: &str) -> Self {
        Self(unsafe {
            rs2::rs2_create_error(
                message.as_ptr() as *const u8,
                function.as_ptr() as *const u8,
                args.as_ptr() as *const u8,
                rs2::rs2_exception_type_RS2_EXCEPTION_TYPE_UNKNOWN,
            )
        })
    }

    /// Return the message of the error.
    pub fn get_message(&self) -> String {
        unsafe { cstring_to_string(rs2::rs2_get_error_message(self.0)) }
    }

    /// Return the function in which the error occured.
    pub fn get_function(&self) -> String {
        unsafe { cstring_to_string(rs2::rs2_get_failed_function(self.0)) }
    }

    /// Return what arguments caused the error.
    pub fn get_args(&self) -> String {
        unsafe { cstring_to_string(rs2::rs2_get_failed_args(self.0)) }
    }

    /// All librealsense functions from the C API follow the same pattern. The last argument is
    /// always an error argument. If an error is stored into this argument it means the function
    /// failed.
    ///
    /// The `callX` functions wrap arround this pattern and extracts the error and result into a
    /// rust `Result`. They take a functions from librealsense and calls it with the arguments
    /// passed in. The number X corrisponds to the number of arguments the librealsense function
    /// takes (minus the error argument).
    ///
    /// These function are also able to convert the result into a rust wrapper, as long as the
    /// rust wrapper implements `From<CRes>` (where `CRes` is the type returned from C).
    ///
    /// Using these instead of implementing the error checks and conversions manually avoides
    /// writing a ton of unessesary boilerplate code, as most of these bindings are just wrapping
    /// the C API and not much else.
    pub(crate) fn call0<CRes, Res>(
        func: unsafe extern "C" fn(*mut *mut rs2::rs2_error) -> CRes,
    ) -> Result<Res, Error>
    where
        Res: From<CRes>,
    {
        let mut error = Error::default();
        let res: Res = unsafe { func(error.inner()) }.into();
        error.check()?;
        Ok(res)
    }

    /// See `call0`
    pub(crate) fn call1<A0, CRes, Res>(
        func: unsafe extern "C" fn(A0, *mut *mut rs2::rs2_error) -> CRes,
        a0: A0,
    ) -> Result<Res, Error>
    where
        Res: From<CRes>,
    {
        let mut error = Error::default();
        let res: Res = unsafe { func(a0, error.inner()) }.into();
        error.check()?;
        Ok(res)
    }

    /// See `call0`
    pub(crate) fn call2<A0, A1, CRes, Res>(
        func: unsafe extern "C" fn(A0, A1, *mut *mut rs2::rs2_error) -> CRes,
        a0: A0,
        a1: A1,
    ) -> Result<Res, Error>
    where
        Res: From<CRes>,
    {
        let mut error = Error::default();
        let res: Res = unsafe { func(a0, a1, error.inner()) }.into();
        error.check()?;
        Ok(res)
    }

    /// See `call0`
    pub(crate) fn call3<A0, A1, A2, CRes, Res>(
        func: unsafe extern "C" fn(A0, A1, A2, *mut *mut rs2::rs2_error) -> CRes,
        a0: A0,
        a1: A1,
        a2: A2,
    ) -> Result<Res, Error>
    where
        Res: From<CRes>,
    {
        let mut error = Error::default();
        let res: Res = unsafe { func(a0, a1, a2, error.inner()) }.into();
        error.check()?;
        Ok(res)
    }

    /// See `call0`
    pub(crate) fn call6<A0, A1, A2, A3, A4, A5, CRes, Res>(
        func: unsafe extern "C" fn(A0, A1, A2, A3, A4, A5, *mut *mut rs2::rs2_error) -> CRes,
        a0: A0,
        a1: A1,
        a2: A2,
        a3: A3,
        a4: A4,
        a5: A5,
    ) -> Result<Res, Error>
    where
        Res: From<CRes>,
    {
        let mut error = Error::default();
        let res: Res = unsafe { func(a0, a1, a2, a3, a4, a5, error.inner()) }.into();
        error.check()?;
        Ok(res)
    }

    /// See `call0`
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn call7<A0, A1, A2, A3, A4, A5, A6, CRes, Res>(
        func: unsafe extern "C" fn(A0, A1, A2, A3, A4, A5, A6, *mut *mut rs2::rs2_error) -> CRes,
        a0: A0,
        a1: A1,
        a2: A2,
        a3: A3,
        a4: A4,
        a5: A5,
        a6: A6,
    ) -> Result<Res, Error>
    where
        Res: From<CRes>,
    {
        let mut error = Error::default();
        let res: Res = unsafe { func(a0, a1, a2, a3, a4, a5, a6, error.inner()) }.into();
        error.check()?;
        Ok(res)
    }

    /// Return `*mut *mut rs2::rs2_error` handle required by other functions of the API.
    fn inner(&mut self) -> *mut *mut rs2::rs2_error {
        &mut self.0 as *mut *mut rs2::rs2_error
    }

    /// Check the value of [`Error`](../error/struct.Error.html).
    ///
    /// # Returns
    /// * A Result that can be bubbled up if the error is not null.
    fn check(self) -> Result<(), Error> {
        if self.0.is_null() {
            Ok(())
        } else {
            Err(self)
        }
    }
}
