# Aivero Contrib

These are the open source componets developed and maintained by Aivero.

- The `aivero-rgbd-toolkit` is the GStreamer based toolkit for interfacing with RGB-D cameras such as the Intel RealSense and Microsoft Azure Kinect cameras. We very much welcome MR/PRs for additional camera support.
- Every `gst-` prefixed subfolder contains a GStreamer element.
- The `recipes` subfolder contains `conanfile.py`-only conan.io packages of external software.


----


## How to Contribute


This project welcomes third-party code via merge requests.

You are welcome to propose and discuss enhancements using issues. Please label issues with the following labels:

- Bug: A bug in the code. Please make sure to describe thoroughly how to reproduce the bug.
- Enhancement: A proposed feature, which would improve the project somehow.

> Branching Policy: The master branch is considered stable, at all times. Features are to be implemented on feature
> branches, which are reviewed before being merged into master.

Please adhere to the following standards:

- Every example/source file must include correct copyright notice
- For indentation we are using spaces and not tabs
- Line-endings must be Unix and not DOS style
- Use `cargo clippy` to lint your code - fix all issues before commiting
- Use `cargo fmt` to format code before commiting

## Setup Conan

```bash
# Install Conan (Requires Python 3 and pip)
pip3 install conan
# Install Conan config
conan config install https://gitlab.com/aivero/open-source/conan-config/-/archive/master/conan-config-master.tar.gz -sf conan-config-master
# Set arch specific profile
conan config set general.default_profile=linux-x86_64
```