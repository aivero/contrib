# gst-frame-aligner

## Summary
This repository contains a Gstreamer frame aligner element for Aivero's `video/rgbd` streaming pipelines.

It aligns the depth stream to the color stream using the camera parameters provided in the `calib` folder.

## Building

To build this repo from source, just do:
```bash
cd gst-frame-aligner
conan create . 
```

And then copy the .so file from your Conan build folder to your Gstreamer environment folder, where the other gstreamer elements reside. You can check this by gst-inspecting a Gstreamer element, e.g.:
```bash
gst-inspect-1.0 rgbdmux
```

## Running 
To run this element plese use it in a gstreamer pipeline.

For example:
```bash
gst-launch-1.0 realsensesrc serial=728312070140 timestamp-mode=clock_all enable-color=true ! framealigner calib-file=calib/rs728312070140.yaml ! rgbddemux name=depth_demux depth_demux.src_depth ! queue ! glimagesink depth_demux.src_color ! queue ! glimagesink
```
The only necessary parameter is the path to the YAML camera parameter file. We provide an example one taken from a D435 Realsense camera.