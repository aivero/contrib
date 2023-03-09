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
