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

use crate::camera_meta_capnp::intrinsics::*;

/// Intrinsic parameters of a specific camera.
#[derive(Debug, PartialEq, Clone)]
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

#[derive(Debug, PartialEq, Clone)]
pub enum Distortion {
    /// Unknown or unsupported distortion model. Distortion coefficients might are invalid.
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

/// RealSense distortion coefficients. The use of these coefficients depend on the utilised distrortion model.
#[derive(Debug, Default, PartialEq, Clone)]
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

/// K4A distortion coefficients.
#[derive(Debug, Default, PartialEq, Clone)]
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
