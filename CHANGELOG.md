# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [1.2.0] - 2020-07-02
### Added
- `ProcessingBlock`
- `log::log_to_file()`
- Constructor and Debug for `StreamResolution`

### Changed
- Applied clippy suggestions

## [1.1.0] - 2020-03-03
### Added
- Wrappers for `Intrinsics` and `Extrinsics`
  - Acquisition of `Intrinsics` via `StreamProfile::get_intrinsics()`
  - Acquisition of `Extrinsics` via `StreamProfile::get_extrinsics()` or `StreamProfile::get_extrinsics_to()`

## [1.0.1] - 2019-12-12
### Added
- Add `Pipeline::poll_for_frames()`
- Add `Playback` and the functionality for setting rosbag playback to non real-time

## [1.0.0] - 2019-11-11
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
