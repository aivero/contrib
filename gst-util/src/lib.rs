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
