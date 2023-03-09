# k4a-rs
Unofficial Rust FFI bindings for Azure Kinect SDK (`k4a`). For more information, please visit the [official API documentation](https://microsoft.github.io/Azure-Kinect-Sensor-SDK).

## Overview

These bindings are based on [`k4a-sys`](https://gitlab.com/aivero/hardware/k4a-sys), which automatically exposes the C API of `k4a` by the use of [`bindgen`](https://rust-lang.github.io/rust-bindgen/). This repository (`k4a-rs`) makes the auto-generated `k4a-sys` bindings more idiomatic and safe for use in applications, e.g. all structs are implemented to automatically release the memory they use on `drop()`.

### Missing features

**This repository does not implement bindings to the SDK for work with microphone array!**

Prototypes for (hopefully) all other functionalities are written into the code and marked as `unimplemented!()`. Open a MR or an issue if you need to implement bindings to some of them.

> Note: Several functionalities have not yet been tested! Open an issue if you experience some problems that are related to the Rust bindings.

## Error and Event logging - Environment Variables

#### K4A_ENABLE_LOG_TO_STDOUT
* `0` - Disable logging to stdout.
* `else` - Log all messages to stdout.

#### K4A_ENABLE_LOG_TO_A_FILE
* `0` - Disable logging to a file.
* `~\log\custom.log` - Log all messages to the path and file specified, which must end in ".log" to be considered a valid entry. When enabled, this takes precedence over the value of K4A_ENABLE_LOG_TO_STDOUT.

#### K4A_LOG_LEVEL
* `c` - Log all messages of level *critical* criticality.
* `e` (DEFAULT) - Log all messages of level *error* or higher criticality.
* `w` - Log all messages of level *warning* or higher criticality.
* `i` - Log all messages of level *info* or higher criticality.
* `t` - Log all messages of level *trace* or higher criticality.
