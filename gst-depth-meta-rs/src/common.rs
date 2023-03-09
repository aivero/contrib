#[cfg(test)]
pub(crate) mod tests {
    /// Return true if `a` and `b` are nearly equal.
    /// Adapted from https://floating-point-gui.de/errors/comparison
    pub(crate) fn nearly_equal_f32(a: f32, b: f32) -> bool {
        let diff = (a - b).abs();

        #[allow(clippy::float_cmp)]
        let are_equal = a == b;

        if are_equal {
            true
        } else if a == 0.0 || b == 0.0 || diff < std::f32::MIN_POSITIVE {
            diff < (std::f32::EPSILON * std::f32::MIN_POSITIVE)
        } else {
            (diff / (a.abs() + b.abs()).min(std::f32::MAX)) < std::f32::EPSILON
        }
    }
}
