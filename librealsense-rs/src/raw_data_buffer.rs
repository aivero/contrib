use crate::error::Error;

#[derive(Debug)]
pub struct RawDataBuffer(pub(crate) *mut rs2::rs2_raw_data_buffer);

unsafe impl Sync for RawDataBuffer {}
unsafe impl Send for RawDataBuffer {}

impl Drop for RawDataBuffer {
    fn drop(&mut self) {
        unsafe { rs2::rs2_delete_raw_data(self.0) }
    }
}

impl RawDataBuffer {
    pub fn raw_data(&self) -> Result<*const u8, Error> {
        Error::call1(rs2::rs2_get_raw_data, self.0)
    }

    pub fn raw_data_size(&self) -> Result<std::ffi::c_int, Error> {
        Error::call1(rs2::rs2_get_raw_data_size, self.0)
    }
}

impl std::convert::TryFrom<&RawDataBuffer> for &str {
    type Error = Error;

    fn try_from(value: &RawDataBuffer) -> Result<Self, Self::Error> {
        let ptr = value.raw_data()?;
        let size = value.raw_data_size()?;
        let slice = unsafe { std::slice::from_raw_parts(ptr, size as usize) };
        std::str::from_utf8(slice).map_err(|_| Error::new("Invalid utf8 string", "as_str", ""))
    }
}
