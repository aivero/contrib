/*
 * AIVERO CONFIDENTIAL
 * __________________
 *
 *  [2017] - [2020] Aivero AS
 *  All Rights Reserved.
 *
 * NOTICE:  All information contained herein is, and remains
 * the property of Aivero AS and its suppliers,
 * if any. The intellectual and technical concepts contained
 * herein are proprietary to Aivero AS
 * and its suppliers and may be covered by EU,
 * patents in process, and are protected by trade secret or copyright law.
 * Dissemination of this information or reproduction of this material
 * is strictly forbidden unless prior written permission is obtained
 * from Aivero AS.
 */
use serde::{Deserialize, Serialize};

/// List of all intrinsics and extrinsics for a calibrated camera setup.
#[derive(Debug, PartialEq, Eq, Clone, Copy, Serialize, Deserialize)]
pub struct DDDQRoiMeta {
    pub x: u16,
    pub y: u16,
    pub w: u16,
    pub h: u16,
}

impl Default for DDDQRoiMeta {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            w: u16::MAX,
            h: u16::MAX,
        }
    }
}
/// Struct that holds a custom tag to add dddq roi
pub struct DDDQRoiTag {}
// Implement Tag trait for DDDQRoiTag
impl<'a> gst::tags::Tag<'a> for DDDQRoiTag {
    type TagType = &'a str;
    fn tag_name<'b>() -> &'b str {
        "dddq_roi_tag"
    }
}
// Implement CustomTag for DDDQRoiTag
impl gst::tags::CustomTag<'_> for DDDQRoiTag {
    // TODO: Check the appropriate Tag Flag
    const FLAG: gst::TagFlag = gst::TagFlag::Decoded;
    const NICK: &'static str = "dddq_roi_tag";
    const DESCRIPTION: &'static str = "DDDQ ROI";
}
