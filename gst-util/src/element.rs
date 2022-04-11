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
use glib::translate::*;
use glib::value::*;
use glib::*;
use gst::prelude::*;
use gst::*;
use gst_sdp::*;

pub trait ElementExtension {
    /// Creates a ghost pad with the same name and direction as the pad named `pad_name` in
    /// `element`. This function only ghosts static pads.
    fn ghost_static_pad(&self, pad_name: &str) -> Result<GhostPad, BoolError>;

    /// Links all `elements` together in the same way that `Element::link_many` does. Unlike
    /// `Element::link_many`, this function takes an `IntoIterator` of elements instead of a
    /// slice, which can allow one to avoid creating a slice when not necessary. On failure,
    /// this function might have linked some, but not all, the elements.
    fn link_iter<Elems, ElemRef>(elements: Elems) -> Result<(), BoolError>
    where
        Elems: IntoIterator<Item = ElemRef>,
        ElemRef: AsRef<gst::Element>;
}

impl<T> ElementExtension for T
where
    T: IsA<gst::Element>,
{
    fn ghost_static_pad(&self, pad_name: &str) -> Result<GhostPad, BoolError> {
        let pad = self
            .static_pad(pad_name)
            .ok_or_else(|| glib::bool_error!("Element did not have pad '{}'", pad_name))?;

        let direction = pad.direction();
        gst::GhostPad::builder(Some(pad_name), direction).build_with_target(&pad)
    }

    fn link_iter<Elems, ElemRef>(elements: Elems) -> Result<(), BoolError>
    where
        Elems: IntoIterator<Item = ElemRef>,
        ElemRef: AsRef<gst::Element>,
    {
        let mut iter = elements.into_iter();
        let mut prev = orelse!(iter.next(), return Ok(()));
        for elem in iter {
            prev.as_ref().link(elem.as_ref())?;
            prev = elem;
        }
        Ok(())
    }
}

pub trait ElementSignalExtension {
    /// Connect to `update-sdp` signal of `element`, which will call `callback`
    /// when this signal is emited. This function exists, as a wrapper over some
    /// ugly code to aquire the sdp message as a mutable borrow.
    fn connect_update_sdp<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, &mut SDPMessageRef) + Send + Sync + 'static;
}

impl<T> ElementSignalExtension for T
where
    T: IsA<gst::Element>,
    Option<T>: for<'a> FromValue<'a>,
{
    fn connect_update_sdp<F>(&self, f: F) -> SignalHandlerId
    where
        F: Fn(&Self, &mut SDPMessageRef) + Send + Sync + 'static,
    {
        self.connect("update-sdp", false, move |values| {
            let element = values[0].get::<Option<T>>().unwrap().unwrap();

            // Obtain a mutable reference to the SDPMessage.
            // See https://github.com/aivero-support/centricular-consulting/issues/9#issuecomment-755298751
            let sdp_msg = unsafe {
                let ptr = gobject_ffi::g_value_get_boxed(values[1].to_glib_none().0);
                assert!(!ptr.is_null());
                &mut *(ptr as *mut SDPMessageRef)
            };

            f(&element, sdp_msg);
            None
        })
    }
}

pub trait ElementFactoryExtension {
    /// Creates an element with the properties in the `props` arguments set. This is
    /// the same as calling `make + elem.set_properties`
    fn make_with(elem: &str, props: &[(&str, &dyn ToValue)]) -> Result<Element, BoolError>;
}

impl ElementFactoryExtension for ElementFactory {
    fn make_with(elem: &str, props: &[(&str, &dyn ToValue)]) -> Result<Element, BoolError> {
        let res = ElementFactory::make(elem, None)?;
        res.set_properties(props);
        Ok(res)
    }
}
