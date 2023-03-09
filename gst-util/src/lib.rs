extern crate gst;
extern crate gst_sdp;

pub mod bin;
pub mod element;
pub mod error;
pub mod message;
pub mod object;
pub mod sdp;
pub mod taglist;

#[macro_export]
macro_rules! orelse {
    ($expr:expr, $other:expr) => {
        match ($expr) {
            Some(val) => val,
            None => $other,
        }
    };
    ($expr:expr, $err:ident, $other:expr) => {
        match ($expr) {
            Ok(val) => val,
            Err($err) => $other,
        }
    };
}
