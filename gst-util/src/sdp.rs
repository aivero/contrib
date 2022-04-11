use gst_sdp::*;
use std::str::FromStr;

pub trait SDPMessageRefExtensions {
    fn typed_attribute_val<T, E>(&self, name: &str) -> Result<Option<T>, E>
    where
        T: FromStr<Err = E>;
}

impl SDPMessageRefExtensions for SDPMessageRef {
    fn typed_attribute_val<T, E>(&self, name: &str) -> Result<Option<T>, E>
    where
        T: FromStr<Err = E>,
    {
        match self.attribute_val(name) {
            Some(res) => Ok(Some(T::from_str(res)?)),
            None => Ok(None),
        }
    }
}
