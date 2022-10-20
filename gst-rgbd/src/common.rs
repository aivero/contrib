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

/// Get the property from the given `value` as type `T`, and debug the value of it using the gst
/// logging system.
/// # Arguments
/// * `cat` - Debug category of the element.
/// * `value` - The value to read.
/// * `property_name` - The name of the property we're in the process of setting.
/// * `old_value` - The old value of the property.
/// # Returns
/// `value` converted to type `T`.
/// # Panics
/// * If `value` contains type that is different from `T`.
pub fn get_property_and_debug<'a, T>(
    cat: gst::DebugCategory,
    value: &'a gst::glib::Value,
    property_name: &str,
    old_value: T,
) -> T
where
    T: std::fmt::Display + gst::glib::value::FromValue<'a>,
{
    let t = value.get::<T>().unwrap();
    gst_info!(
        cat,
        "Changing property `{}` from {} to {}",
        property_name,
        old_value,
        t
    );
    t
}
