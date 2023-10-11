# Changelog
All notable changes to the contrib mono repo project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## unreleased - 2023-08-27 
- Bump gst to 1.22.5
- Bump libnice to 0.1.21, required by gst-plugins-bad
- Add dockerfiles for s3sfs

----

## ARCHIVE pre-mono: gst-depth-meta
## [0.3.1] - 2020-07-22
### Added
- Take gst version from conan settings

## [0.3.0] - 2020-06-22
### Added
- Bump gstreamer deps to 1.17.1


## [0.2.1] - Prior to 2019-10-21
Before [0.2.1] this project did not have a changelog.

----

## ARCHIVE pre-mono: gst-rgbd

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.0.4] - 2021-Jan-15

### Fixed

- rgbddemux: Source pads are now reconfigured if new upstream CAPS are received.

## [1.0.3] - 2020-Dec-15

### Fixed

- Enforce that some streams become the main stream if they are available.
  - This fixes a bug where `dddqmeta` could become the main buffer for non determenistic
    pad adds.

## [1.0.2] - 2020-Dec-1

### Fixed

- `rgbdmux`
  - Some files contain framerate numerator equal to 0. Added condition where numerator is changed to 30 when this happens.

## [1.0.1] - 2020-Oct-26

### Fixed

- `rgbdmux`
  - Downstream requests for video format are now correctly sent to the upstream element(s), even if their linking is delayed. Previously, video format forwarding could fail if `rgbdmux` was linked with upstream (e.g. in NULL state) before downsteam (e.g. in READY state).

## [1.0.0] - 2020-Oct-02

### Changed

- Refactored elements into submodules.
- `rgbdmux`
  - Default value for property `drop-if-missing` to _false_, as it is not needed in most pipelines and it causes unnecessary amount of buffers to get dropped.
- `rgbddemux`
  - Changed presence of src pad to _sometimes_ instead of _request_.

### Fixed

- Several review comments from Sebastian Dröge for both `rgbdmux` and `rgbddemux`. Thank you for your feedback!
  - Refactored mutexes to eliminate known places of possible deadlocks.
  - `rgbdmux`
    - Downstream requests for video format are now correctly sent to the upstream element(s).
    - Forwarding of EOS - EOS is now send in `aggregate()` when all sink pads are marked as EOS.
    - Output CAPS for MJPG stream, i.e. format is now properly specified for downstream elements.
  - `rgbddemux`
    - Element no longer attempts to push buffers to pads that are not linked.
      - This allowed to make use of FlowCombiner in sink chain, which was disabled until now.
    - Group ID of upstream is now used during pushing of stream-start events.
    - Unneeded src pads are now removed if new sink CAPS were negotiated.
    - Internal src pads now contain the name of the stream instead of the pad name, as it was not consistent with the documentation.

## [0.5.4] - 2020-Sep-07

### Fixed

- Timestamps in GAP events - use proper timestamps instead of current clock time.
- Fix incorrect computation of the frame duration for GAP event.

## [0.5.3] - 2020-Sep-04

### Fixed

- Building this plugin as Gst version 1.16 instead of 1.17.

## [0.5.2] - 2020-Sep-02

### Updated

- Update GStreamer bindings version and various minor cleanups/fixes.

## [0.5.1] - 2020-Aug-21

### Changed

- Fall back to the default pad event handling - fixes EOS not being forwarded correctly in ~50% if cases. Credit to @slomo for the fix.

## [0.5.0] - 2020-Jul-22

### Changed

- Re-implemented the stream-id generation on rgbddemux, so that it follows the GStreamer
  documentation:

> The stream_id should be a unique string that consists of the upstream stream-id, / as separator
> and a unique stream-id for this specific stream. A new stream-id should only be created for a
> stream if the upstream stream is split into (potentially) multiple new streams, e.g. in a demuxer,
> but not for every single element in the pipeline.

## [0.4.1] - 2020-Jul-01

### Changed

- Add gstreamer conan config, bump cargo deps to gst-depth-meta-rs

## [0.4.0] - 2020-Apr-07

### Changed

- Update dependencies
  - `glib` to 0.9
  - `gst` to 0.15

## 0.3.1 - 2020-Apr-02

### Fixed

- `rgbdmux` - Fixed EOS forwarding by implementing custom `sink_event()` handling.
- `rgbdmux` - No longer tries to renegotiate CAPS once EOS was received and the element is shutting down.

## 0.3.0 - 2020-Mar-10

### Added

- `rgbddemux` - Added `distribute-timestamps` property that enables distribution of the main buffer timestamps to the auxiliary buffers embedded within the `video/rbgd` stream. The property defaults to _false_ and must be explicitly enabled to cause any difference from the previous version.

## 0.2.3 - 2020-Mar-04

### Modified

- Use of standard function for work with `video/rgbd` CAPS from `gst_depth_meta::rgbd` module, instead of custom-local implementation.

## 0.2.2 - 2020-Jan-28

### Added

- Support for MJPG

## 0.2.1 - 2020-Jan-24

### Added

- `rgbdmux` - CAPS negotiation of video formats with downstream element, conversion of these formats from `video/rgbd` to `video/x-raw` and subsequent use of the requested formats during creation of new sink pads.

## 0.2.0 - 2020-Jan-21

### Removed

- Partial KLV packing. `rgbddemux` now forwards the buffers simply as Cap'n Proto serialised buffers

### Changed

- `rgbddemux` and `rgbdmux` now treats per-frame metadata as top-level streams. This means that they're also included in the `streams` field of the `video/rgbd` CAPS.

## 0.1.6

### Added

- `rgbdmux` - If `drop-if-missing` is set to false, there is no deadline for the aggregation. This makes deadline based aggregation optional. If `drop-if-missing` is set to true, the behaviour is identical to `0.1.5`.
- `rgbdmux` - Added `deadline-multiplier` property that controls the duration of deadline if `drop-if-missing` is enabled.

### Changed

- `rgbdmux` - Split `internals` into `sink_pads` and `settings` to avoid the possibility of deadlock caused by Mutex, which was introduced by the new additions.

## 0.1.5

### Added

- `rgbdmux` - Dropping of all other frames if one of the sink pads does not have a buffer queued, see property `drop-if-missing`.
- `rgbdmux` - Synchronisation of buffers by dropping buffers that are late, see property `drop-to-synchronise`.

## 0.1.4

### Added

- Metadata support: The `rgbddemux` reads per-frame metadata on the frame buffers and pushes them to a sink-pad called _dddqmeta_. Similarly, if the rgbd CAPS contains a stream called _dddqmeta_, its content is also outputted on the _dddqmeta_ sink.

### Changed

- Peer elements may now request pads on the `rgbddemux`, which may be linked when CAPS are available
- `rgbdmux` does downstream CAPS negotiation when sink pads are requested on the element

### Known issues

- Requesting a pad on the `rgbddemux` which is not in the `video/rgbd` stream will cause the pipeline to freeze

## 0.1.3

### Patched

- This patch fixes a bug introduces in 0.1.2 which caused the rgbddemux to return a gst::FlowError when it attempts to push meta buffer for which it does not have a pad.

## Before

Prior to 0.1.2 this repository did not have a changelog.

----

## ARCHIVE pre-mono: gst-colorizer

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).


## [0.1.5] - 2020-07-01
### Changed
- Add gstreamer conan setting and depend on it for gst deps
## [0.1.1] - 2019-09-23
### Changed
- The colorizer now sets all pixels have `value >= farcut || value < nearcut`
to 0. This solves an issue that arose when the near-cut or far-cut on the
colorizer does not align with that of the `dddqdec`.

## [0.1.0] - Prior to 2019-09-23
Before [0.1.1] this project did not have a changelog.

----
## ARCHIVE pre-mono: librealsense-rs-sys
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.3] - 2020-Dec-15
### Changed
- Set build dep bindgen to 0.56

## [0.5.2] - 2020-08-24
### Changed
- Set build dep bindgen to 0.55.0
- Set build dep pkgconf to 0.3.18

## [0.5.1] - 2020-07-14
### Changed
- Set build dep bindgen to 0.54.3
- Set build dep pkgconf to 0.3.14

## [0.4.0] - 2020-03-25
### Changed
- Set minimum requirement for librealsense2 to version 2.33.1 as it fixes issues with timestamps.

## [0.3.0] - 2020-02-20
### Added
- Rustification of `rs2_distortion` enum.

## [0.2.0]
### Added
- Prior to 0.3.0 this repository did not have a changelog.

----

## ARCHIVE pre-mono: librealsense-rs
# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.3.0] - 2021-Jan-19

### Changed

- `Context` now implements `Send`

### Fixed

- Fixes memory leak in `Context::query_devices`

## [2.2.6] - 2020-Dec-15

### Fixed

- Updated to librealsense 2.40 to fix .so loading issues

## [2.2.5] - 2020-Dec-08

### Added

- `Pipeline::wait_for_frameset()` that returns the entire frameset instead of a list of individual frames.
- `Frame::extract_frames()` for extracting frames embedded in frameset.
- Derived `Debug` for `StreamData`.

### Fixed

- Processing blocks. Tested with align processing block.

## [2.2.4] - 2020-Dec-03

### Fixed

- Removed `&self` parameter on processing block's `create_*` method, as they couldn't be constructed otherwise.

## [2.2.3] - 2020-08-24
### Added
- Bump version of librealsense-rs to 0.5.2

## [2.2.2] - 2020-07-14
### Added
- Bump version of librealsense-rs

## [2.2.1] - 2020-Jul-13
### Added
- Display for `StreamResolution` in addition to Constructor and Debug introduced in '2.2.0'

## [2.2.0] - 2020-Jul-02
### Added
- `ProcessingBlock`
- `log::log_to_file()`
- Constructor and Debug for `StreamResolution`

### Changed
- Applied clippy suggestions

## [1.1.0] - 2020-Mar-03
### Added
- Wrappers for `Intrinsics` and `Extrinsics`
  - Acquisition of `Intrinsics` via `StreamProfile::get_intrinsics()`
  - Acquisition of `Extrinsics` via `StreamProfile::get_extrinsics()` or `StreamProfile::get_extrinsics_to()`

## [1.0.1] - 2019-Dec-12
### Added
- Add `Pipeline::poll_for_frames()`
- Add `Playback` and the functionality for setting rosbag playback to non real-time

## [1.0.0] - 2019-Nov-11
### Added
- Add new features
- Add documentation, README and CHANGELOG
- Add function prototypes
- Add file prototypes
- Add few high-level functionalities
### Modified
- Standardise naming with C/C++ API, deprecate old names
- Make handles to C objects public only to the crate
#### Breaking changes
- The following methods now take `&str` as parameter instead of `String`:
  - `Config::enable_device()`
  - `Config::enable_device_from_file()`
  - `Config::enable_device_from_file_repeat_option()`
  - `Config::enable_record_to_file()`
  - `Device::load_json()`


## [0.5.0]
### Added
- Prior to 0.6.0 this repository did not have a changelog.


----
## ARCHIVE pre-mono: libk4a-rs-sys
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.6] - 2020-07-31
### Changed
- Bump cargo.toml dep on bindgen to 0.55.0 due to yanked 0.54.1

## [0.1.5] - 2020-07-31
### Changed
- Bump cargo.toml dep on bindgen to 0.54.1

## [0.1.4] - 2020-05-26
### Changed
- Moved repo to public subgroup
- Added MIT license

## [0.1.3] - 2020-01-28
### Added
- Support for linking with `pkgconfig`.


## [0.1.2] - 2019-12-04
### Added
- Compiler linking of `k4arecord` library.


## [0.1.1] - 2019-11-13
### Added
- Add public access to k4a C enumerations under Rustified names. E.g. `k4a_image_format_t` as `ImageFormat`.
- Add `README.md` and this `CHANGELOG.md`.
### Modified
- Fixed compiler linking of `k4a` library.


## [0.1.0]
### Added
- Prior to 0.1.1 this repository did not have a changelog.

----

## ARCHIVE pre-mono: libk4a-rs

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.5.0] - 2020-Nov-18

### Added
- `Device::open_first_unused()`, which allows opening of first UNUSED device as per https://gitlab.com/aivero/public/gstreamer/gst-k4a/-/issues/16.

### Changed
- Fixed multi-camera support

## [0.4.7] - 2020-07-31
### Changed
- Bump dep version k4a-sys to 0.1.6 due to bindgen 0.55.0

## [0.4.6] - 2020-07-31
### Fixed
- Cast data type for vec lengths returned from bindgen bindings from u64 to usize. Necessary to support bindgen 0.54.1

## [0.4.5] - 2020-07-31
### Changed
- Bump dep version k4a-sys to 0.1.5

## [0.4.4] - 2020-06-24

### Fixed
- Data type for byte array pointer, used by `k4a_calibration_get_from_raw()`, which needs to be different for x86_64 and aarch64 builds

## [0.4.3] - 2020-06-17

### Fixed
- Error implementation of K4aError, which was causing infinite loop

## [0.4.2] - 2020-05-26

### Changed
- Added MIT license
- Bumped k4a-sys dependency to 0.1.4
- Make conan dependencies dynamic
- Move to public subgroup

## [0.4.1] - 2020-02-27

### Changed

- Made parameters contained within `IntrinsicsParameters` public.

## [0.4.0] - 2020-02-18

### Added

- `Calibration::default()` implementation.
- Allow `Transformation` to move between threads by adding `Send` and `Sync` implementations.

### Changed

- Replaced the `Transformation` member `&Calibration` with `Resolution` for depth and color as that is all we need.

## [0.3.0] - 2020-02-18

### Added

- `Calibration::extrinsics(source,target)` function, which allows the user to obtain the extrinsics between source and target.

## [0.2.0] - 2020-02-18

### Added

- `CameraCalibration` wrapper, which allows the user to inspect and use the camera calibration.
- `Intrinsics` wrapper.
- `Extrinsics` wrapper.

## [0.1.3] - 2020-01-31
### Modified
- Return `Result<Image>` from `Capture::get_X_image()` instead of just `Image` to accound for possible failure.

## [0.1.2] - 2019-12-04
### Added
- Small helper functions for work with generated enums under `utilities.rs`
### Performed
- Performed first tests with physical device and recording
- Fixed issues with passing of handles
- Fixed issues with allocating memory for CString returned by several functions

## [0.1.1] - 2019-11-13
### Added
- Add `K4aError`.
- Add linking to structs and methods in documentation.
- Add this `CHANGELOG.md`.
### Modified
- Use `VectorXYZ` to contain output from `ImuSample` accelerometer and gyroscope instead of `[f32, 3]`.
- Make use of `K4aError`.
- Make use of the Rustified struct implemented in `k4a-sys` "0.1.1".
- Format documentation.
- Update `README.md`.

## [0.1.0]
### Added
- Prior to 0.1.1 this repository did not have a changelog.
