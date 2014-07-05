#Rust-Image

A basic image processing library written in Rust.

Originally a course project, the goal of this project is to explore the encoding and decoding of image formats for image processing and the algorithms used to process images. 

Rust-Image can read and write 8, 24, and 32-bit BMP images and has implementations of some point processing algorithms and a convolution filter blurring function. The library decodes images and copies pixel data to an Image struct to allow conversion between image formats and to create image processing functions that are independent of the image format given as input. 


##Usage
Initializing an arbitrary empty image with some ```width```, ```height``` and ```color_type```.
<pre>
let empty_gray_image = Image::new(240, 360, GRAYSCALE8);
let empty_rgb_image  = Image::new(80, 80, RGB8);
let empty_rgba_image = Image::new(540, 720, RGBA8);
</pre>


Decoding stored bitmap image by providing a valid file path.
<pre>
let image = Image::read_bitmap("path/to/imagefile.bmp");

match image {
  Some(mut image) => {
    // Some image processing
    image.write_bitmap("path/to/save/imagefile.bmp");
  },
  None  => {
    println!("Looks like you didn't get a valid image.");
  }
}
</pre>





