// Aivero
// Copyright (C) <2019> Aivero
//
// This library is free software; you can redistribute it and/or
// modify it under the terms of the GNU Library General Public
// License as published by the Free Software Foundation; either
// version 2 of the License, or (at your option) any later version.
// This library is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU
// Library General Public License for more details.
// You should have received a copy of the GNU Library General Public
// License along with this library; if not, write to the
// Free Software Foundation, Inc., 51 Franklin St, Fifth Floor,
// Boston, MA 02110-1301, USA.

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
