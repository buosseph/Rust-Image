Rust-Image
==========

A (very) basic image library written in Rust.

The goal of this project is to explore the encoding and decoding of image formats for image processing and the algorithms used to process images. Currently, the build can only read and write 24-bit color PPM and BMP images and has implementations of some point processing algorithms and a convolution filter blurring function. The library decodes images and copies pixel data to an Image struct to allow conversion between image formats and to create image processing functions that are independent of the image format given as input. 
