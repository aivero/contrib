use crate::camera_meta_capnp::{intrinsics::*, *};
pub use crate::hash_map_serialize;
pub use crate::intrinsics::*;
pub use crate::transformation::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter};

/// List of all intrinsics and extrinsics for a calibrated camera setup.
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct CameraMeta {
    /// List of intrinsics for each camera.
    #[serde(with = "hash_map_serialize")]
    pub intrinsics: HashMap<String, Intrinsics>,
    /// List of extrinsics between cameras for a calibrated camera setup. Key indicated coordinate frames in form of `(source, target)`.
    #[serde(with = "hash_map_serialize")]
    pub extrinsics: HashMap<(String, String), Transformation>,
    /// Scaling factor of the depth map, in metres.
    pub depth_scale: f32,
}

impl Display for CameraMeta {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let intrinsics: String = self
            .intrinsics
            .iter()
            .map(|(s, i)| format!("'{}': {}", s, i))
            .collect::<Vec<String>>()
            .join("\n");

        let extrinsics: String = self
            .extrinsics
            .iter()
            .map(|(s, e)| format!("'{}' -> '{}':\n{}", s.0, s.1, e))
            .collect::<Vec<String>>()
            .join("\n");

        write!(
            f,
            "Intrinsics:\n{}\nExtrinsics:\n{}\nDepth scale: {}",
            intrinsics, extrinsics, self.depth_scale
        )
    }
}

/// Struct that holds a custom tag
pub struct CameraMetaTag {}
// Implement Tag trait for CameraMetaTag
impl<'a> gst::tags::Tag<'a> for CameraMetaTag {
    type TagType = &'a str;
    fn tag_name<'b>() -> &'b str {
        "camera_meta_tag"
    }
}
// Implement CustomTag for 3DQConfigTag
impl gst::tags::CustomTag<'_> for CameraMetaTag {
    const FLAG: gst::TagFlag = gst::TagFlag::Decoded;
    const NICK: &'static str = "camera_meta_tag";
    const DESCRIPTION: &'static str = "Camera Metadata";
}

impl CameraMeta {
    /// Create a new CameraMeta.
    ///
    /// # Arguments
    /// * `intrinsics` - List of intrinsics for each camera.
    /// * `extrinsics` - List of extrinsics between cameras for a calibrated camera setup. Key indicated coordinate frames in form of `(source, target)`.
    /// * `depth_scale` - Scaling factor of the depth map, in metres.
    ///
    /// # Returns
    /// * Newly created CameraMeta.
    pub fn new(
        intrinsics: HashMap<String, Intrinsics>,
        extrinsics: HashMap<(String, String), Transformation>,
        depth_scale: f32,
    ) -> Self {
        Self {
            intrinsics,
            extrinsics,
            depth_scale,
        }
    }

    /// Get extrinsics from `source` to `target` camera. Transformation tree is not followed and
    /// therefore an explicit transformation from `source` to `target` or from `target` to `source`
    /// must be contained within CameraMeta.
    ///
    /// # Arguments
    /// * `source` - Source coordinate frame.
    /// * `target` - Target coordinate frame.
    ///
    /// # Returns
    /// * Transformation from `source` to `target`, if known.
    pub fn get_extrinsics(&self, source: String, target: String) -> Option<Transformation> {
        if let Some(transformation) = self.extrinsics.get(&(source.clone(), target.clone())) {
            Some(transformation.clone())
        } else {
            self.extrinsics
                .get(&(target, source))
                .map(|transformation| transformation.inverse())
        }
    }

    /// Serialise CameraMeta by the use of Cap'n Proto.
    ///
    /// # Returns
    /// * Serialised RotationMatrix.
    pub fn serialise(self) -> Result<Vec<u8>, capnp::Error> {
        // Create new cap'n proto builder
        let mut builder = capnp::message::Builder::new_default();
        {
            // Create root for the cap'n proto buffer
            let mut camera_meta_serialised = builder.init_root::<camera_meta::Builder>();

            // Intrinsics
            // Initialise cap'n proto intrinsics with the appropriate length
            let mut intrinsics_serialised = camera_meta_serialised
                .reborrow()
                .init_intrinsics(self.intrinsics.len() as u32);

            // Iterate over all intrinsics and serialise them
            for (index, (camera_name, camera_intrinsics)) in self.intrinsics.iter().enumerate() {
                // Get access to the next member
                let mut intrinsics_instance = intrinsics_serialised.reborrow().get(index as u32);

                // Set the corresponding primitive entries for intrinsics
                intrinsics_instance.set_camera(camera_name);
                intrinsics_instance.set_fx(camera_intrinsics.fx);
                intrinsics_instance.set_fy(camera_intrinsics.fy);
                intrinsics_instance.set_cx(camera_intrinsics.cx);
                intrinsics_instance.set_cy(camera_intrinsics.cy);

                // Set the corresponding distortion as union
                let mut union_builder = capnp::message::Builder::new_default();
                let mut distortion = intrinsics_instance.get_distortion();
                match &camera_intrinsics.distortion {
                    Distortion::Unknown => {
                        distortion.set_unknown(());
                    }
                    Distortion::None => {
                        distortion.set_none(());
                    }
                    Distortion::RsBrownConrady(rs_coefficients) => {
                        distortion.set_rs_brown_conrady(Self::serialise_rs_coefficients(
                            &mut union_builder,
                            rs_coefficients,
                        ))?;
                    }
                    Distortion::RsModifiedBrownConrady(rs_coefficients) => {
                        distortion.set_rs_modified_brown_conrady(
                            Self::serialise_rs_coefficients(&mut union_builder, rs_coefficients),
                        )?;
                    }
                    Distortion::RsInverseBrownConrady(rs_coefficients) => {
                        distortion.set_rs_inverse_brown_conrady(
                            Self::serialise_rs_coefficients(&mut union_builder, rs_coefficients),
                        )?;
                    }
                    Distortion::RsKannalaBrandt4(rs_coefficients) => {
                        distortion.set_rs_kannala_brandt4(Self::serialise_rs_coefficients(
                            &mut union_builder,
                            rs_coefficients,
                        ))?;
                    }
                    Distortion::RsFTheta(rs_coefficients) => {
                        distortion.set_rs_f_theta(Self::serialise_rs_coefficients(
                            &mut union_builder,
                            rs_coefficients,
                        ))?;
                    }
                    Distortion::K4aBrownConrady(k4a_coefficients) => {
                        distortion.set_k4a_brown_conrady(Self::serialise_k4a_coefficients(
                            &mut union_builder,
                            k4a_coefficients,
                        ))?;
                    }
                }
            }

            // Extrinsics
            // Initialise cap'n proto extrinsics with the appropriate length
            let mut extrinsics_serialised = camera_meta_serialised
                .reborrow()
                .init_extrinsics(self.extrinsics.len() as u32);

            // Iterate over all extrinsics and serialise them
            for (index, ((source, target), camera_extrinsics)) in self.extrinsics.iter().enumerate()
            {
                // Get access to the next member
                let mut extrinsics_instance = extrinsics_serialised.reborrow().get(index as u32);

                // Set the source and target coordinate frames
                extrinsics_instance.set_source(source);
                extrinsics_instance.set_target(target);

                // Set translation
                let mut translation = extrinsics_instance.reborrow().get_translation()?;
                translation.set_x(camera_extrinsics.translation.x);
                translation.set_y(camera_extrinsics.translation.y);
                translation.set_z(camera_extrinsics.translation.z);

                // Set rotation
                let mut rotation = extrinsics_instance.reborrow().get_rotation()?;
                rotation.set_r11(camera_extrinsics.rotation.r11);
                rotation.set_r12(camera_extrinsics.rotation.r12);
                rotation.set_r13(camera_extrinsics.rotation.r13);
                rotation.set_r21(camera_extrinsics.rotation.r21);
                rotation.set_r22(camera_extrinsics.rotation.r22);
                rotation.set_r23(camera_extrinsics.rotation.r23);
                rotation.set_r31(camera_extrinsics.rotation.r31);
                rotation.set_r32(camera_extrinsics.rotation.r32);
                rotation.set_r33(camera_extrinsics.rotation.r33);
            }

            // Set the depth scale
            camera_meta_serialised.set_depth_scale(self.depth_scale);
        }

        let mut serialised_config: Vec<u8> = Vec::new();
        capnp::serialize::write_message(&mut serialised_config, &builder)?;

        Ok(serialised_config)
    }

    /// Serialises RsCoefficients into Cap'n Proto compatible struct.
    ///
    /// # Arguments
    /// * `builder` - Builder used for creation of the new struct.
    /// * `coefficients` - RsCoefficients to serialise.
    fn serialise_rs_coefficients<'a>(
        builder: &'a mut capnp::message::Builder<capnp::message::HeapAllocator>,
        coefficients: &RsCoefficients,
    ) -> rs_coefficients::Reader<'a> {
        let mut rs_coefficients = builder.init_root::<rs_coefficients::Builder>();

        rs_coefficients.set_a1(coefficients.a1);
        rs_coefficients.set_a2(coefficients.a2);
        rs_coefficients.set_a3(coefficients.a3);
        rs_coefficients.set_a4(coefficients.a4);
        rs_coefficients.set_a5(coefficients.a5);

        rs_coefficients.into_reader()
    }

    /// Serialises K4aCoefficients into Cap'n Proto compatible struct.
    ///
    /// # Arguments
    /// * `builder` - Builder used for creation of the new struct.
    /// * `coefficients` - K4aCoefficients to serialise.
    fn serialise_k4a_coefficients<'a>(
        builder: &'a mut capnp::message::Builder<capnp::message::HeapAllocator>,
        coefficients: &K4aCoefficients,
    ) -> k4a_coefficients::Reader<'a> {
        let mut k4a_coefficients = builder.init_root::<k4a_coefficients::Builder>();

        k4a_coefficients.set_k1(coefficients.k1);
        k4a_coefficients.set_k2(coefficients.k2);
        k4a_coefficients.set_k3(coefficients.k3);
        k4a_coefficients.set_k4(coefficients.k4);
        k4a_coefficients.set_k5(coefficients.k5);
        k4a_coefficients.set_k6(coefficients.k6);
        k4a_coefficients.set_p1(coefficients.p1);
        k4a_coefficients.set_p2(coefficients.p2);

        k4a_coefficients.into_reader()
    }

    /// Deserialise CameraMeta by the use of Cap'n Proto.
    ///
    /// # Arguments
    /// * `buffer` - Buffer containing the serialised CameraMeta
    ///
    /// # Returns
    /// * Deserialised RotationMatrix.
    pub fn deserialise(buffer: &[u8]) -> Result<Self, capnp::Error> {
        // Read the serialised message from the byte array
        let serialised_message =
            capnp::serialize::read_message(buffer, capnp::message::ReaderOptions::new())?;

        // Get reader for the serialised message
        let camera_meta = serialised_message.get_root::<camera_meta::Reader>()?;

        // Get the serialised intrinsics and extrinsics
        let serialised_intrinsics = camera_meta.get_intrinsics()?;
        let serialised_extrinsics = camera_meta.get_extrinsics()?;

        // Create intrinsics and extrinsics with the corresponding capacity
        let mut intrinsics: HashMap<String, Intrinsics> =
            HashMap::with_capacity(serialised_intrinsics.len() as usize);
        let mut extrinsics: HashMap<(String, String), Transformation> =
            HashMap::with_capacity(serialised_extrinsics.len() as usize);

        // Deseriaise intrinsics
        for intrinsics_instance in serialised_intrinsics.iter() {
            let distortion = match intrinsics_instance.get_distortion().which() {
                Ok(intrinsics::distortion::Unknown(())) => Distortion::Unknown,
                Ok(intrinsics::distortion::None(())) => Distortion::None,
                Ok(intrinsics::distortion::RsBrownConrady(rs_coefficients)) => {
                    Distortion::RsBrownConrady(RsCoefficients::from(rs_coefficients?))
                }
                Ok(intrinsics::distortion::RsModifiedBrownConrady(rs_coefficients)) => {
                    Distortion::RsModifiedBrownConrady(RsCoefficients::from(rs_coefficients?))
                }
                Ok(intrinsics::distortion::RsInverseBrownConrady(rs_coefficients)) => {
                    Distortion::RsInverseBrownConrady(RsCoefficients::from(rs_coefficients?))
                }
                Ok(intrinsics::distortion::RsKannalaBrandt4(rs_coefficients)) => {
                    Distortion::RsKannalaBrandt4(RsCoefficients::from(rs_coefficients?))
                }
                Ok(intrinsics::distortion::RsFTheta(rs_coefficients)) => {
                    Distortion::RsFTheta(RsCoefficients::from(rs_coefficients?))
                }
                Ok(intrinsics::distortion::K4aBrownConrady(k4a_coefficients)) => {
                    Distortion::K4aBrownConrady(K4aCoefficients::from(k4a_coefficients?))
                }
                Err(capnp::NotInSchema(e)) => {
                    return Err(capnp::Error::unimplemented(format!(
                        "camera_meta: Cannot deserialise distortion with unknown variant: {}",
                        e
                    )))
                }
            };

            intrinsics.insert(
                intrinsics_instance.get_camera()?.to_string(),
                Intrinsics {
                    fx: intrinsics_instance.get_fx(),
                    fy: intrinsics_instance.get_fy(),
                    cx: intrinsics_instance.get_cx(),
                    cy: intrinsics_instance.get_cy(),
                    distortion,
                },
            );
        }

        // Deserialise extrinsics
        for extrinsics_entry in serialised_extrinsics.iter() {
            let translation = extrinsics_entry.get_translation()?;
            let rotation = extrinsics_entry.get_rotation()?;

            extrinsics.insert(
                (
                    extrinsics_entry.get_source()?.to_string(),
                    extrinsics_entry.get_target()?.to_string(),
                ),
                Transformation {
                    translation: Translation {
                        x: translation.get_x(),
                        y: translation.get_y(),
                        z: translation.get_z(),
                    },
                    rotation: RotationMatrix {
                        r11: rotation.get_r11(),
                        r12: rotation.get_r12(),
                        r13: rotation.get_r13(),
                        r21: rotation.get_r21(),
                        r22: rotation.get_r22(),
                        r23: rotation.get_r23(),
                        r31: rotation.get_r31(),
                        r32: rotation.get_r32(),
                        r33: rotation.get_r33(),
                    },
                },
            );
        }

        // Return CameraMeta containing the deserialised properties
        Ok(Self {
            extrinsics,
            intrinsics,
            depth_scale: camera_meta.get_depth_scale(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rand::random;
    const TEST_ITERATIONS: usize = 100;

    /// Initialise CameraMeta for a setup with "depth", "ir" and "color" streams.
    /// This struct contains the following, with randomised numerical entries.
    ///     Intrinsics
    ///         - "depth"
    ///         - "ir"
    ///         - "color"
    ///     Extrinsics
    ///         - "depth" -> "ir"
    ///         - "depth" -> "color"
    fn initialise_random_camera_meta() -> CameraMeta {
        let mut intrinsics: HashMap<String, Intrinsics> = HashMap::new();

        let intrinsics_depth = Intrinsics {
            fx: random(),
            fy: random(),
            cx: random(),
            cy: random(),
            distortion: Distortion::None,
        };
        intrinsics.insert("depth".to_string(), intrinsics_depth);

        let intrinsics_ir = Intrinsics {
            fx: random(),
            fy: random(),
            cx: random(),
            cy: random(),
            distortion: Distortion::RsBrownConrady {
                0: RsCoefficients {
                    a1: random(),
                    a2: random(),
                    a3: random(),
                    a4: random(),
                    a5: random(),
                },
            },
        };
        intrinsics.insert("ir".to_string(), intrinsics_ir);

        let intrinsics_color = Intrinsics {
            fx: random(),
            fy: random(),
            cx: random(),
            cy: random(),
            distortion: Distortion::K4aBrownConrady {
                0: K4aCoefficients {
                    k1: random(),
                    k2: random(),
                    k3: random(),
                    k4: random(),
                    k5: random(),
                    k6: random(),
                    p1: random(),
                    p2: random(),
                },
            },
        };
        intrinsics.insert("color".to_string(), intrinsics_color);

        let mut extrinsics: HashMap<(String, String), Transformation> = HashMap::new();

        let transformation_depth_ir = Transformation {
            translation: Translation {
                x: random(),
                y: random(),
                z: random(),
            },
            rotation: RotationMatrix {
                r11: random(),
                r12: random(),
                r13: random(),
                r21: random(),
                r22: random(),
                r23: random(),
                r31: random(),
                r32: random(),
                r33: random(),
            },
        };
        extrinsics.insert(
            ("depth".to_string(), "ir".to_string()),
            transformation_depth_ir,
        );

        let transformation_depth_color = Transformation {
            translation: Translation {
                x: random(),
                y: random(),
                z: random(),
            },
            rotation: RotationMatrix {
                r11: random(),
                r12: random(),
                r13: random(),
                r21: random(),
                r22: random(),
                r23: random(),
                r31: random(),
                r32: random(),
                r33: random(),
            },
        };
        extrinsics.insert(
            ("depth".to_string(), "color".to_string()),
            transformation_depth_color,
        );

        CameraMeta {
            intrinsics,
            extrinsics,
            depth_scale: random(),
        }
    }

    #[test]
    fn serialise_deserialise() {
        for _ in 0..TEST_ITERATIONS {
            let camera_meta = initialise_random_camera_meta();
            let camera_meta_assert_clone = camera_meta.clone();

            let serialised_camera_meta = camera_meta.serialise().unwrap();
            let deserialised_camera_meta =
                CameraMeta::deserialise(&serialised_camera_meta.to_vec()).unwrap();

            assert_eq!(deserialised_camera_meta, camera_meta_assert_clone);
        }
    }

    #[test]
    fn get_extrinsics_depth_ir_color_and_vice_versa() {
        let camera_meta = initialise_random_camera_meta();
        assert!(camera_meta
            .get_extrinsics("depth".to_string(), "color".to_string())
            .is_some());
        assert!(camera_meta
            .get_extrinsics("depth".to_string(), "ir".to_string())
            .is_some());
        assert!(camera_meta
            .get_extrinsics("color".to_string(), "depth".to_string())
            .is_some());
        assert!(camera_meta
            .get_extrinsics("ir".to_string(), "depth".to_string())
            .is_some());
    }
}
