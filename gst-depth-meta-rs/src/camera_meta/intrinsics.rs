use crate::camera_meta_capnp::intrinsics::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

/// Intrinsic parameters of a specific camera.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Intrinsics {
    /// Focal length of the image plane, as a multiple of pixel width.
    pub fx: f32,
    /// Focal length of the image plane, as a multiple of pixel height.
    pub fy: f32,
    /// Horizontal coordinate of the principal point of the image, as a pixel offset from the left edge.
    pub cx: f32,
    /// Vertical coordinate of the principal point of the image, as a pixel offset from the top edge.
    pub cy: f32,
    /// The distortion model of the camera optics.
    pub distortion: Distortion,
}

impl Intrinsics {
    /// Create new Intrinsics from parameters.
    ///
    /// # Arguments
    /// * `fx` - Focal length of the image plane, as a multiple of pixel width.
    /// * `fy` - Focal length of the image plane, as a multiple of pixel height.
    /// * `cx` - Horizontal coordinate of the principal point of the image, as a pixel offset from the left edge.
    /// * `cy` - Vertical coordinate of the principal point of the image, as a pixel offset from the top edge.
    /// * `distortion` - The distortion model of the camera optics.
    ///
    /// # Returns
    /// * Newly created Transformaion.
    pub fn new(fx: f32, fy: f32, cx: f32, cy: f32, distortion: Distortion) -> Self {
        Self {
            fx,
            fy,
            cx,
            cy,
            distortion,
        }
    }
}

impl Display for Intrinsics {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "fx={}, fy={}, cy={}, cx={}, dist={}",
            self.fx, self.fy, self.cx, self.cy, self.distortion
        )
    }
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum Distortion {
    /// Unknown or unsupported distortion model.
    Unknown,
    /// Image is already rectilinear. No distortion compensation is required.
    None,
    /// RealSense Brown-Conrady calibration model.
    RsBrownConrady(RsCoefficients),
    /// RealSense equivalent to Brown-Conrady distortion, except that tangential distortion is applied to radially distorted points.
    RsModifiedBrownConrady(RsCoefficients),
    /// RealSense equivalent to Brown-Conrady distortion, except that it undistorts image instead of distorting it.
    RsInverseBrownConrady(RsCoefficients),
    /// RealSense four parameter Kannala Brandt distortion model.
    RsKannalaBrandt4(RsCoefficients),
    /// RealSense F-Theta fish-eye distortion model.
    RsFTheta(RsCoefficients),
    /// K4A Brown-Conrady calibration model.
    K4aBrownConrady(K4aCoefficients),
}

impl Display for Distortion {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Distortion::*;
        let cs = match self {
            Unknown | None => String::default(),
            RsBrownConrady(rs)
            | RsModifiedBrownConrady(rs)
            | RsInverseBrownConrady(rs)
            | RsKannalaBrandt4(rs)
            | RsFTheta(rs) => format!(
                "a1={}, a2={}, a3={}, a4={}, a5={}",
                rs.a1, rs.a2, rs.a3, rs.a4, rs.a5
            ),
            K4aBrownConrady(k4a) => format!(
                "k1={}, k2={}, k3={}, k4={}, k5={}, k6={}, p1={}, p2={}",
                k4a.k1, k4a.k2, k4a.k3, k4a.k4, k4a.k5, k4a.k6, k4a.p1, k4a.p2
            ),
        };

        match self {
            Unknown => write!(f, "Unknown"),
            None => write!(f, "Already rectilinear"),
            RsBrownConrady(_) | K4aBrownConrady(_) => write!(f, "Brown-Conrady[{}]", cs),
            RsModifiedBrownConrady(_) => write!(f, "Modified Brown-Conrady[{}]", cs),
            RsInverseBrownConrady(_) => write!(f, "Inverse Brown-Conrady[{}]", cs),
            RsKannalaBrandt4(_) => write!(f, "Kannala-Brandt[{}]", cs),
            RsFTheta(_) => write!(f, "F-Theta[{}]", cs),
        }
    }
}

/// RealSense distortion coefficients. The use of these coefficients depend on the utilised distrortion model.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct RsCoefficients {
    /// 1st distortion coefficient.
    pub a1: f32,
    /// 2nd distortion coefficient.
    pub a2: f32,
    /// 3rd distortion coefficient.
    pub a3: f32,
    /// 4th distortion coefficient.
    pub a4: f32,
    /// 5th distortion coefficient.
    pub a5: f32,
}

impl RsCoefficients {
    /// Create new RsCoefficients.
    ///
    /// # Arguments
    /// * `a1` - 1st distortion coefficient.
    /// * `a2` - 2nd distortion coefficient.
    /// * `a3` - 3rd distortion coefficient.
    /// * `a4` - 4th distortion coefficient.
    /// * `a5` - 5th distortion coefficient.
    ///
    /// # Returns
    /// * Newly created Transformaion.
    pub fn new(a1: f32, a2: f32, a3: f32, a4: f32, a5: f32) -> Self {
        Self { a1, a2, a3, a4, a5 }
    }
}

impl From<rs_coefficients::Reader<'_>> for RsCoefficients {
    /// Implements conversion from Cap'n Proto schema representation of RsCoefficients.
    fn from(coefficients: rs_coefficients::Reader) -> Self {
        Self {
            a1: coefficients.get_a1(),
            a2: coefficients.get_a2(),
            a3: coefficients.get_a3(),
            a4: coefficients.get_a4(),
            a5: coefficients.get_a5(),
        }
    }
}

impl From<[f32; 5]> for RsCoefficients {
    /// Implements conversion from slice into RsCoefficients.
    fn from(slice: [f32; 5]) -> Self {
        Self {
            a1: slice[0],
            a2: slice[1],
            a3: slice[2],
            a4: slice[3],
            a5: slice[4],
        }
    }
}

/// K4A distortion coefficients.
#[derive(Debug, Default, PartialEq, Clone, Serialize, Deserialize)]
pub struct K4aCoefficients {
    /// 1st radial distortion coefficient.
    pub k1: f32,
    /// 2nd radial distortion coefficient.
    pub k2: f32,
    /// 3rd radial distortion coefficient.
    pub k3: f32,
    /// 4th radial distortion coefficient.
    pub k4: f32,
    /// 5th radial distortion coefficient.
    pub k5: f32,
    /// 6th radial distortion coefficient.
    pub k6: f32,
    /// 1st tangential distortion coefficient.
    pub p1: f32,
    /// 2nd tangential distortion coefficient.
    pub p2: f32,
}

impl K4aCoefficients {
    /// Create new K4aCoefficients.
    ///
    /// # Arguments
    /// * `k1` - 1st radial distortion coefficient.
    /// * `k2` - 2nd radial distortion coefficient.
    /// * `k3` - 3rd radial distortion coefficient.
    /// * `k4` - 4th radial distortion coefficient.
    /// * `k5` - 5th radial distortion coefficient.
    /// * `k6` - 6th radial distortion coefficient.
    /// * `p1` - 1st tangential distortion coefficient.
    /// * `p2` - 2nd tangential distortion coefficient.
    ///
    /// # Returns
    /// * Newly created Transformaion.
    #[allow(clippy::too_many_arguments)]
    pub fn new(k1: f32, k2: f32, k3: f32, k4: f32, k5: f32, k6: f32, p1: f32, p2: f32) -> Self {
        Self {
            k1,
            k2,
            k3,
            k4,
            k5,
            k6,
            p1,
            p2,
        }
    }
}

impl From<k4a_coefficients::Reader<'_>> for K4aCoefficients {
    /// Implements conversion from Cap'n Proto schema representation of K4aCoefficients.
    fn from(coefficients: k4a_coefficients::Reader) -> Self {
        Self {
            k1: coefficients.get_k1(),
            k2: coefficients.get_k2(),
            k3: coefficients.get_k3(),
            k4: coefficients.get_k4(),
            k5: coefficients.get_k5(),
            k6: coefficients.get_k6(),
            p1: coefficients.get_p1(),
            p2: coefficients.get_p2(),
        }
    }
}

impl From<[f32; 8]> for K4aCoefficients {
    /// Implements conversion from slice into K4aCoefficients.
    fn from(slice: [f32; 8]) -> Self {
        Self {
            k1: slice[0],
            k2: slice[1],
            k3: slice[2],
            k4: slice[3],
            k5: slice[4],
            k6: slice[5],
            p1: slice[6],
            p2: slice[7],
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::common::tests::nearly_equal_f32;

    #[test]
    fn rs_coefficients_from_slice() {
        let rs_coefficients_slice = [1.1, 2.2, 3.3, 4.4, 5.5];

        let rs_coefficients = RsCoefficients::from(rs_coefficients_slice);

        assert!(nearly_equal_f32(rs_coefficients.a1, 1.1));
        assert!(nearly_equal_f32(rs_coefficients.a2, 2.2));
        assert!(nearly_equal_f32(rs_coefficients.a3, 3.3));
        assert!(nearly_equal_f32(rs_coefficients.a4, 4.4));
        assert!(nearly_equal_f32(rs_coefficients.a5, 5.5));
    }

    #[test]
    fn k4a_coefficients_from_slice() {
        let k4a_coefficients_slice = [1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8];

        let k4a_coefficients = K4aCoefficients::from(k4a_coefficients_slice);

        assert!(nearly_equal_f32(k4a_coefficients.k1, 1.1));
        assert!(nearly_equal_f32(k4a_coefficients.k2, 2.2));
        assert!(nearly_equal_f32(k4a_coefficients.k3, 3.3));
        assert!(nearly_equal_f32(k4a_coefficients.k4, 4.4));
        assert!(nearly_equal_f32(k4a_coefficients.k5, 5.5));
        assert!(nearly_equal_f32(k4a_coefficients.k6, 6.6));
        assert!(nearly_equal_f32(k4a_coefficients.p1, 7.7));
        assert!(nearly_equal_f32(k4a_coefficients.p2, 8.8));
    }
}
