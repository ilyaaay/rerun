---
title: "ImageFormat"
---
<!-- DO NOT EDIT! This file was auto-generated by crates/build/re_types_builder/src/codegen/docs/website.rs -->

The metadata describing the contents of a [`components.ImageBuffer`](https://rerun.io/docs/reference/types/components/image_buffer).

## Rerun datatype
[`ImageFormat`](../datatypes/image_format.md)


## Arrow datatype
```
Struct {
    width: uint32
    height: uint32
    pixel_format: nullable uint8
    color_model: nullable uint8
    channel_datatype: nullable uint8
}
```

## API reference links
 * 🌊 [C++ API docs for `ImageFormat`](https://ref.rerun.io/docs/cpp/stable/structrerun_1_1components_1_1ImageFormat.html)
 * 🐍 [Python API docs for `ImageFormat`](https://ref.rerun.io/docs/python/stable/common/components#rerun.components.ImageFormat)
 * 🦀 [Rust API docs for `ImageFormat`](https://docs.rs/rerun/latest/rerun/components/struct.ImageFormat.html)


## Used by

* [`DepthImage`](../archetypes/depth_image.md)
* [`Image`](../archetypes/image.md)
* [`Mesh3D`](../archetypes/mesh3d.md)
* [`SegmentationImage`](../archetypes/segmentation_image.md)
