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

use crate::element::*;
use crate::message::*;
use crate::orelse;
use glib::*;
use gst::prelude::*;
use gst::*;

/// Options for the `add_iter` function which extends what the function does.
#[derive(Default, Copy, Clone)]
pub struct Add {
    /// All elements added will also be linked in order `E1 ! E2 ! ... EN`
    pub link: bool,
    /// Will also sync the elements state with the bin they where added to.
    pub sync: bool,
    /// Will take the first and ghost its sink pads to the bins sink pad.
    pub ghost_sink: bool,
    /// Will take the last and ghost its src pads to the bins src pad.
    pub ghost_src: bool,
}

impl Add {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn link() -> Self {
        Self::new().and_link()
    }

    pub fn sync() -> Self {
        Self::new().and_sync()
    }

    pub fn ghost() -> Self {
        Self::new().and_ghost()
    }

    pub fn ghost_src() -> Self {
        Self::new().and_ghost_src()
    }

    pub fn ghost_sink() -> Self {
        Self::new().and_ghost_sink()
    }

    pub fn and_link(mut self) -> Self {
        self.link = true;
        self
    }

    pub fn and_sync(mut self) -> Self {
        self.sync = true;
        self
    }

    pub fn and_ghost(self) -> Self {
        self.and_ghost_sink().and_ghost_src()
    }

    pub fn and_ghost_sink(mut self) -> Self {
        self.ghost_sink = true;
        self
    }

    pub fn and_ghost_src(mut self) -> Self {
        self.ghost_src = true;
        self
    }
}

/// Options for the `remove_iter` function which extends what the function does.
#[derive(Default, Copy, Clone)]
pub struct Remove {
    /// Set the state of every element removed to `gst::State::Null`
    pub null: bool,
}

impl Remove {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn null() -> Self {
        Self::new().and_null()
    }

    pub fn and_null(mut self) -> Self {
        self.null = true;
        self
    }
}

pub trait BinExtension {
    /// Adds all `elements` to `bin`. Unlike `Bin::add_many`, this function accepts an
    /// `IntoIterator` of elements instead of a slice, which can allow one to avoid creating a
    /// slice when not necessary. This function also takes an `opt` parameter, where one can
    /// turn on more things this function should do while adding elements to the bin.
    /// On failure, this function might leave `bin` with some of, but not all, the elements
    /// added.
    fn add_iter<Elems, ElemRef>(&self, opt: Add, elements: Elems) -> Result<(), BoolError>
    where
        Elems: IntoIterator<Item = ElemRef>,
        ElemRef: AsRef<Element>;

    /// Removes specified `elements` from `bin`. Unlike `Bin::remove_many`, this function accepts an
    /// `IntoIterator` of elements instead of a slice, which can allow one to avoid creating a
    /// slice when not necessary. This function also takes an `opt` parameter, where one can
    /// turn on more things this function should do while adding elements to the bin.
    /// On failure, this function might leave `bin` with some of, but not all, the elements
    /// removed.
    fn remove_iter<Elems, ElemRef>(&self, opt: Remove, elements: Elems) -> Result<(), BoolError>
    where
        Elems: IntoIterator<Item = ElemRef>,
        ElemRef: AsRef<Element>;

    /// Hook into the bins bus and dump debug dots when certain messages are posted.
    fn dump_dot_on_important_messages(&self);
}

impl<T> BinExtension for T
where
    T: IsA<Bin> + IsA<Element> + IsA<gst::Object> + std::marker::Send,
{
    fn add_iter<Elems, ElemRef>(&self, opt: Add, elements: Elems) -> Result<(), BoolError>
    where
        Elems: IntoIterator<Item = ElemRef>,
        ElemRef: AsRef<Element>,
    {
        let mut iter = elements.into_iter();
        let mut prev = orelse!(iter.next(), return Ok(()));

        self.add(prev.as_ref())?;
        if opt.ghost_sink {
            let pad = prev.as_ref().ghost_static_pad("sink")?;
            self.add_pad(&pad)?;
        }
        if opt.sync {
            prev.as_ref().sync_state_with_parent()?;
        }

        for elem in iter {
            self.add(elem.as_ref())?;

            if opt.link {
                prev.as_ref().link(elem.as_ref())?;
            }
            if opt.sync {
                elem.as_ref().sync_state_with_parent()?;
            }
            prev = elem;
        }

        if opt.ghost_src {
            let pad = prev.as_ref().ghost_static_pad("src")?;
            self.add_pad(&pad)?;
        }
        Ok(())
    }

    fn remove_iter<Elems, ElemRef>(&self, opt: Remove, elements: Elems) -> Result<(), BoolError>
    where
        Elems: IntoIterator<Item = ElemRef>,
        ElemRef: AsRef<Element>,
    {
        for elem in elements.into_iter() {
            let elem = elem.as_ref();
            self.remove(elem)?;

            if opt.null {
                let _ = elem
                    .set_state(gst::State::Null)
                    .map_err(|e| glib::bool_error!("{}", e))?;
            }
        }
        Ok(())
    }

    fn dump_dot_on_important_messages(&self) {
        let _ = self.bus().map(|bus| {
            bus.add_watch({
                let this = self.clone();
                move |_, msg| {
                    msg.dump_dot_if_important(this.upcast_ref());
                    glib::Continue(true)
                }
            })
        });
    }
}
