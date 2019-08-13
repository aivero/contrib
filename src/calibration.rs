use k4a_sys::*;

/// Struct representation of `Calibration` that wraps around `k4a_calibration_t`.
pub struct Calibration {
    pub(crate) handle: k4a_calibration_t,
}

// The memory the `k4a_calibration_t` is written to is allocated and owned by the caller, so there is no need to free or release.

impl Calibration {
    /// Converts raw calibration data into `Calibration`.
    ///
    /// **Parameters:**
    /// * **raw_calibration** - Raw calibration data.
    /// * **depth_mode** - Mode of the depth camera.
    /// * **color_resolution** - Resolution of the color camera.
    ///
    /// **Return value:**
    /// * **Ok(Calibration)** on success.
    /// * **Err(&str)** on failure.
    pub fn from_raw(
        raw_calibration: &mut [i8],
        depth_mode: k4a_depth_mode_t,
        color_resolution: k4a_color_resolution_t,
    ) -> Result<Calibration, &'static str> {
        let mut calibration_handle = k4a_calibration_t::default();
        match unsafe {
            k4a_calibration_get_from_raw(
                raw_calibration.as_mut_ptr(),
                raw_calibration.len(),
                depth_mode,
                color_resolution,
                &mut calibration_handle,
            )
        } {
            k4a_result_t::K4A_RESULT_SUCCEEDED => Ok(Calibration {
                handle: calibration_handle,
            }),
            k4a_result_t::K4A_RESULT_FAILED => {
                Err("Failed to convert raw calibration data into `Calibration`")
            }
        }
    }

    /// This function is NOT implemented!
    pub fn convert_2d_to_2d(&self) {
        unimplemented!()
    }

    /// This function is NOT implemented!
    pub fn convert_2d_to_3d(&self) {
        unimplemented!()
    }

    /// This function is NOT implemented!
    pub fn convert_3d_to_2d(&self) {
        unimplemented!()
    }

    /// This function is NOT implemented!
    pub fn convert_3d_to_3d(&self) {
        unimplemented!()
    }
}
