use glib::*;
use gst::*;

pub trait ToErrorMessage {
    /// Converts `self` to a gstreamer `ErrorMessage` in `domain`.
    fn to_err_msg<Err: MessageErrorDomain>(&self, domain: Err) -> ErrorMessage;
}

impl ToErrorMessage for BoolError {
    fn to_err_msg<Err: MessageErrorDomain>(&self, domain: Err) -> ErrorMessage {
        ErrorMessage::new(
            &domain,
            Some(&self.message),
            None,
            self.filename,
            self.function,
            self.line,
        )
    }
}

pub trait MapErrorMessage<Out> {
    /// Maps result error to gstreamer error message
    fn map_err_msg<Err: MessageErrorDomain>(self, domain: Err) -> Result<Out, ErrorMessage>;
}

impl<T, E> MapErrorMessage<T> for Result<T, E>
where
    E: ToErrorMessage,
{
    fn map_err_msg<Err: MessageErrorDomain>(self, domain: Err) -> Result<T, ErrorMessage> {
        self.map_err(|e| e.to_err_msg(domain))
    }
}

pub trait LogError<T, E> {
    fn log_err(self, cat: gst::DebugCategory, level: gst::DebugLevel) -> Result<T, E>;
}

impl<T, E: std::fmt::Display> LogError<T, E> for Result<T, E> {
    fn log_err(self, cat: gst::DebugCategory, level: gst::DebugLevel) -> Result<T, E> {
        self.map_err(|err| {
            gst::gst_log_with_level!(cat, level: level, "{}", err);
            err
        })
    }
}
