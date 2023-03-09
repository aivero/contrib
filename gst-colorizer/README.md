# gst-colorizer

GStreamer plugin containing a colorizer, which converts 16-bit depth video into RGB.

# Getting started

## Install a tagged release

You may use conan to install a pre-built release of the gst-realsense package:

```bash
conan install gst-colorizer/0.1.1@ -if installation
export GST_PLUGIN_PATH=$GST_PLUGIN_PATH:$PWD/installation
# And validate that the colorizer is properly installed
gst-inspect-1.0 colorizer
```

## Build your own

If you have made changes to the `colorizer` that you wish to try, you may want to build the project locally:

```
cd gst-colorizer
conan install -if build .
source build/env.sh
cargo build --release
export GST_PLUGIN_PATH=$GST_PLUGIN_PATH:$PWD/target/release
```

> Note: `conan install -if build .` might require you to build extra packages. Just follow the instructions in the error message. 

Now you should see the plugin's element `colorizer`.

```
gst-inspect-1.0 colorizer
```

## Running in combination with `rgbddemux` and `realsensesrc`

Source and export `GST_PLUGIN_PATH` in a single terminal for both `realsensesrc`, `rgbddemux` and `colorizer` (if not done before).
```
source gst-realsense/build/env.sh --extend
export GST_PLUGIN_PATH=gst-realsense/target/release:${GST_PLUGIN_PATH}

source gst-rgbd/build/env.sh --extend
export GST_PLUGIN_PATH=gst-rgbd/target/release:${GST_PLUGIN_PATH}

source gst-colorizer/build/env.sh --extend
export GST_PLUGIN_PATH=gst-colorizer/target/release:${GST_PLUGIN_PATH}
```

An example of a pipeline:

```bash
# Please replace XXXXXXXX with the serial on your RealSense camera
export RS_SERIAL=XXXXXXXX
gst-launch-1.0 \
realsensesrc serial=$RS_SERIAL enable-depth=true ! \
rgbddemux name=realsense_demux \
realsense_demux.src_depth ! queue ! colorizer near-cut=300 far-cut=3000 ! glimagesink 
```