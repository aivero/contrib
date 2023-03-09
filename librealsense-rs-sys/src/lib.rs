#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

impl rs2_error {
    #![allow(clippy::all)]
    pub fn new() -> rs2_error {
        rs2_error { _unused: [0; 0] }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn log_test() {
        unsafe {
            let error = std::ptr::null_mut::<*mut crate::rs2_error>();
            crate::rs2_log_to_console(crate::rs2_log_severity::RS2_LOG_SEVERITY_ERROR, error);
        }
    }
}
