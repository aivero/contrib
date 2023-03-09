use crate::orelse;
use gst::prelude::*;

pub trait ObjectExtention {
    /// Get the gstreamer pipeline this object is a part of
    fn pipeline(&self) -> Option<gst::Pipeline>;

    /// Get an iterator that iterates up the parents starting from this object.
    fn iterate_parents(&self) -> ParentIterator;
}

impl<T> ObjectExtention for T
where
    T: IsA<gst::Object>,
{
    fn pipeline(&self) -> Option<gst::Pipeline> {
        self.iterate_parents()
            .filter_map(|obj| obj.downcast().ok())
            .next()
    }

    fn iterate_parents(&self) -> ParentIterator {
        ParentIterator {
            curr: self.parent(),
        }
    }
}

pub struct ParentIterator {
    curr: Option<gst::Object>,
}

impl std::iter::Iterator for ParentIterator {
    type Item = gst::Object;

    fn next(&mut self) -> Option<Self::Item> {
        let curr = orelse!(self.curr.take(), return None);
        self.curr = curr.parent();
        Some(curr)
    }
}
