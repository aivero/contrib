use glib::*;
use gst::*;

pub trait TagListExtension {
    /// Creates a new `TagList` containing a single tag.
    fn new_single<'a, T: Tag<'a>>(value: &T::TagType) -> gst::TagList
    where
        T::TagType: ToSendValue;
}

impl TagListExtension for gst::TagList {
    fn new_single<'a, T: Tag<'a>>(value: &T::TagType) -> gst::TagList
    where
        T::TagType: ToSendValue,
    {
        let mut res = gst::TagList::new();
        res.make_mut().add::<T>(value, gst::TagMergeMode::Append);
        res
    }
}
