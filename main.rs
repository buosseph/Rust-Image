extern crate time;
extern crate sync;

use std::slice::from_elem;
use std::slice;
use std::path::posix::{Path};
use std::io::File;
use std::os;
use std::str;
use std::uint;

pub struct RGB_Pixel {
  r: u8,
  g: u8,
  b: u8,
}
pub enum ColorType {
  RGB, GRAYSCALE
}

pub struct Image {
  width: uint,
  height: uint,
  color_type: ColorType,
  data: ~[u8],
}
impl Image {
  pub fn new(height: uint, width: uint) -> Image {
    let size = 3 * height * width;
    let mut buffer = from_elem(size, 0u8);
    Image{height: height, width: width, color_type: RGB, data: buffer}
  }
 
  fn buffer_size(&self) -> uint {
    // if self.color_type == RGB {  // This doesn't work
    //   3 * self.width * self.height
    // }
    // else {
    //   self.width * self.height
    // }
    3 * self.width * self.height
  }
 
  fn get_offset(&self, x: uint, y: uint) -> Option<uint> {
    let offset = (x * 3) + (y * self.width * 3);
    if offset < self.buffer_size() {
      Some(offset)
    }else{
      None
    }
  }
 
  pub fn get_pixel(&self, x: uint, y: uint) -> Option<RGB_Pixel> {
    match self.get_offset(x, y) {
      Some(offset) => {
        let r1 = self.data[offset];
        let g1 = self.data[offset + 1];
        let b1 = self.data[offset + 2];
        Some(RGB_Pixel{r: r1, g: g1, b: b1})
      },
      None => None
    }
  }
 
  pub fn set_pixel(&mut self, x: uint, y: uint, color: RGB_Pixel) -> bool {
    match self.get_offset(x, y) {
      Some(offset) => {
        self.data[offset] = color.r;
        self.data[offset + 1] = color.g;
        self.data[offset + 2] = color.b;
        true
      },
      None => false
    }
  }
}

// PPM Image format
impl Image {  // Not complete, and may never be
  fn read_ppm(image_path_str: &str) -> Image {
    let path = Path::new(image_path_str);

    let mut p_num: ~[u8] = ~[0 as u8, 0 as u8];
    let mut comment_bytes: ~[u8] = ~[];
    let mut width_bytes: ~[u8] = ~[];
    let mut height_bytes: ~[u8] = ~[];
    let mut color_mode_bytes: ~[u8] = ~[];
    let mut image_data_bytes: ~[u8] = ~[];

    match File::open(&path) {
      Ok(mut image) => {
        // Find P Number
        match image.read(p_num) {
          Ok(num_of_bytes) =>  {
            // Works, to view hex in array need to iterate and println!("{:x}")
            // for i in range(0, p_num.len()) {
            //   let byte = p_num[i];
            //   println!("{:x}", byte);
            // }
            match str::from_utf8(p_num) {
              Some(file_type)  => {
                // Check if valid header
              },    
              None        => {fail!("Something went wrong reading the file type")}
            }
          },
          Err(e) => {println!("Something went wrong: {}", e)}
        }

        // Getting header data
        let mut isComment: bool = false;
        let mut isWidth: bool = false;
        let mut isHeight: bool = false;
        let mut isColorMode: bool = false;
        loop {
          match image.read_byte() {
            Ok(byte) =>  {
              let byte_string = str::from_byte(byte);

              // Checking for comment
              if str::eq(&byte_string, &~"#") {
                isComment = true;
              }
              if isComment && str::eq(&byte_string, &~"\n") {
                comment_bytes.push(byte);
                isComment = false;
                isWidth = true;
                continue;
              }
              if isComment {
                comment_bytes.push(byte);
              }
            
              // Read width, ends at space or newline
              if isWidth && (str::eq(&byte_string, &~"\n") || str::eq(&byte_string, &~" ")){
                isWidth = false;
                isHeight = true;
                continue;
              }
              if isWidth {
                width_bytes.push(byte);
              }

              // Read height, ends at space or newline
              if isHeight && (str::eq(&byte_string, &~"\n") || str::eq(&byte_string, &~" ")) {
                isHeight = false;
                isColorMode = true;
                continue;
              }
              if isHeight {
                height_bytes.push(byte);
              }

              // Read color mode
              if isColorMode && (str::eq(&byte_string, &~"\n") || str::eq(&byte_string, &~" ")) {
                isColorMode = false;
                break;
              }
              if isColorMode {
                color_mode_bytes.push(byte);
              }

              if str::eq(&byte_string, &~"\n") {
                continue;
              }
            },

            Err(e) => {
              println!("Error reading file header: {}", e);
              break;
            }
          }
        }
        // println!("Comment: {}", str::from_utf8(comment_bytes).unwrap());
        // println!("Width: {}", str::from_utf8(width_bytes).unwrap());
        // println!("Height: {}", str::from_utf8(height_bytes).unwrap());
        // println!("Color Mode: {}", str::from_utf8(color_mode_bytes).unwrap());
        
        // Would want a more appropriate way of filling image_data_bytes
        loop {
          match image.read_byte() {
            Ok(byte) => {image_data_bytes.push(byte)},
            Err(e)   => {break;}
          }
        }
      },
      Err(e)    => {println!("Error opening file: {}", e)}
    };

    let mut width = match uint::parse_bytes(width_bytes, 10){
      Some(number) => {number},
      None    => {0 as uint}
    };
    let mut height = match uint::parse_bytes(height_bytes, 10){
      Some(number) => {number},
      None    => {0 as uint}
    };
    // Only testing color images
    Image{width: width, height: height, color_type: RGB, data: image_data_bytes}
  }  
  fn write_ppm(&self, filename: &str) -> bool {
    let path = Path::new(filename);
    let mut file = File::create(&path);
    let header = format!("P6 {} {} 255\n", self.width, self.height);
    file.write(header.as_bytes());
    file.write(self.data);
    true
  }
}

// BMP 3.x Image format (4.x and 5.x not functional)
impl Image {  // Need to fix pixel ordering difference, screws up format translation and convolution
  // NOTE: BMP pixels stored as BGR, not RGB
  // NOTE: BMP scanlines stored left to right, BOTTOM UP --> store pixels starting from bottom row when writing

  // Good example for furture development of image format: http://www.kalytta.com/bitmap.h
  pub fn read_bmp(image_path_str: &str) -> Image{
    let path = Path::new(image_path_str);

    let mut file_type: ~[u8] = ~[0 as u8, 0 as u8];
    let mut file_size: u32 = 0 as u32;
    let mut offset: u32 = 0 as u32;
    let mut header_size: u32 = 0 as u32;
    let mut image_width: u32 = 0 as u32;
    let mut image_height: u32 = 0 as u32;
    let mut planes: u16 = 0 as u16;
    let mut bits_per_pixel: u16 = 0 as u16;  

    let mut compression_type: u32 = 0 as u32;
    let mut size_of_bitmap: u32 = 0 as u32; 
    let mut horizontal_resolution: u32 = 0 as u32;
    let mut vertical_resolution: u32 = 0 as u32;
    let mut colors_used: u32 = 0 as u32;
    let mut colors_important: u32 = 0 as u32;

    let mut image_data_bytes: ~[u8] = ~[];
    let mut buffer: ~[u8] = ~[];


    match File::open(&path) {
      Ok(mut image) => {
        // Check file type
        match image.read(file_type) {
          Ok(num_of_bytes) =>  {
            match str::from_utf8_owned(file_type) {
              Some(read_file_type)  => {
                if !str::eq(&read_file_type, &~"BM") {
                  fail!("Input image was not a valid BMP 3.x image");
                }
              },
              None        => {fail!("Error wrong reading the file type")}
            }
          },
          Err(e) => {println!("Error reading BMP file header: {}", e)}
        }

        // Read remaining BMP file header
        match image.read_le_u32() {
          Ok(read_file_size) => {
            file_size = read_file_size;
          },
          Err(e)  => {println!("Error reading the filesize: {}", e)}
        }
        match image.read_le_u16() {
          Ok(read_reserved1) => {
            match image.read_le_u16() {
              Ok(read_reserved2) => {
                match image.read_le_u32() {
                  Ok(read_offset) => {
                    offset = read_offset;
                  },
                  Err(e) => {println!("Error reading the bitmap offset: {}", e)}
                }
              },
              Err(e)  => {println!("Error reading the second reserved word: {}", e)}
            }
          },
          Err(e)  => {println!("Error reading the first reserved word: {}", e)}
        }

        // Read bitmap header
        match image.read_le_u32() {
          Ok(read_header_size) => {
            header_size = read_header_size;
          },
          Err(e) => (println!("Error reading bitmap header size: {}", e))
        }
        match image.read_le_u32() {
          Ok(read_image_width) => {
            image_width = read_image_width;
          },
          Err(e) => (println!("Error reading image width: {}", e))
        }
        match image.read_le_u32() {
          Ok(read_image_height) => {
            image_height = read_image_height;
          },
          Err(e) => (println!("Error reading image height: {}", e))
        }
        match image.read_le_u16() {
          Ok(read_planes) => {
            planes = read_planes;
          },
          Err(e) => (println!("Error reading bitmap planes: {}", e))
        }
        match image.read_le_u16() {
          Ok(read_bits_per_pixel) => {
            bits_per_pixel = read_bits_per_pixel;
          },
          Err(e) => (println!("Error reading bitmap planes: {}", e))
        }
        match image.read_le_u32() {
          Ok(read_compression_type) => {
            compression_type = read_compression_type;
          },
          Err(e) => (println!("Error reading image height: {}", e))
        }
        match image.read_le_u32() {
          Ok(read_bitmap_size) => {
            // The end of scanline bytes are included (thus + 2*height)
            // if RGB: width*height*3 + 2*height  --> testimage bitmapsize = 926400 bytes
            size_of_bitmap = read_bitmap_size;
          },
          Err(e) => (println!("Error reading image height: {}", e))
        }
        match image.read_le_u32() {
          Ok(read_horizontal_resolution) => {
            horizontal_resolution = read_horizontal_resolution;
          },
          Err(e) => (println!("Error reading image height: {}", e))
        }
        match image.read_le_u32() {
          Ok(read_vertical_resolution) => {
            vertical_resolution = read_vertical_resolution;
          },
          Err(e) => (println!("Error reading image height: {}", e))
        }
        match image.read_le_u32() {
          Ok(read_colors_used) => {
            colors_used = read_colors_used;
          },
          Err(e) => (println!("Error reading image height: {}", e))
        }
        match image.read_le_u32() {
          Ok(read_important_colors) => {
            colors_important = read_important_colors;
          },
          Err(e) => (println!("Error reading image height: {}", e))
        }

        // Read remaining data
        // Should read based on image_size in file header
        // let image_size = (file_size as uint) - 14 - (header_size as uint);
        // println!("Image size: {}", image_size);

        // Would want a more appropriate way of filling image_data_bytes
        // NOTE: BMP is not encoded as RGB from top to bottom, thus need to reorganize pixel data before returning Image
        // loop {
        //   match image.read_byte() {
        //     Ok(byte) => {image_data_bytes.push(byte)},
        //     Err(e)   => {break;}
        //   }
        // }

        //let mut pixel_buffer: ~[u8] = ~[0, 0, 0];
        for y in range(0, image_height) {
          for x in range(0, image_width) {
            match image.read_exact(3) {
              Ok(mut pixel_data) => {
                // Determine where to put byte here

                match pixel_data.pop() {
                  Some(red) => {buffer.push(red)},
                  None  => {fail!("Error getting red component for BMP pixel")}
                }
                match pixel_data.pop() {
                  Some(green) => {buffer.push(green)},
                  None  => {fail!("Error getting green component for BMP pixel")}
                }
                match pixel_data.pop() {
                  Some(blue) => {buffer.push(blue)},
                  None  => {fail!("Error getting blue component for BMP pixel")}
                }

              },
              Err(e)    => {fail!("Error reading BMP pixel")}
            }
          }
          // Should read two bytes as 0
          match image.read_le_u16() {
            Ok(end_of_scanline) => {
              if end_of_scanline as uint == 0 {
                continue;
              }
              else {
                break;
                fail!("Error reading end of scanline")
              }
            },
            Err(e) => {
              fail!("Error chekcing end of scanline")
            }
          }
        }

      },
      Err(e)  => {println!("Error opening file: {}", e)}
    }

    // Without this scanlines are flipped in image data
    for i in range(0, image_height){
      let start_index: uint = (image_height as uint - i as uint - 1) * image_width as uint * 3;  // 3 because RGB
      let end_index: uint = start_index + (image_width as uint * 3); // Off by one as slice function doesn't include last index

      let scanline = buffer.slice(start_index, end_index);
      image_data_bytes.push_all(scanline);
    }

    Image{width: image_width as uint, height: image_height as uint, color_type: RGB, data: image_data_bytes}
  }

  pub fn write_bmp(&mut self, filename: &str) -> bool {
    let path = Path::new(filename);
    let mut file = File::create(&path);

    // File Header, 14 bytes in size
    let filetype = "BM";
    /* Total filesize in bytes (File header guaranteed 14 bytes)
     *    Bitmap 3.x => 54 + 3 * width * height bytes
     *    Bitmap 4.x => 108 + 3 * width * height bytes
     */
    let filesize: u32 = 0 as u32; 
    let reserved1: u16 = 0 as u16;
    let reserved2: u16 = 0 as u16;
    let bitmap_offset: u32 = 54 as u32; // Bitmap 3.x => 54, Bitmap 4.x => 108
    file.write(filetype.as_bytes());
    file.write_le_u32(filesize);
    file.write_le_u16(reserved1);
    file.write_le_u16(reserved2);
    file.write_le_u32(bitmap_offset);

    // Bitmap 4.x Header, 108 bytes in size if color space info included
    // Bitmap 3.x Header, 40 bytes in size (no colorspace info)
    let header_size: u32 = 40 as u32;  // Size in bytes
    let image_width: u32 = self.width as u32;    // In pixels
    let image_height: u32 = self.height as u32;   // In pixels
    let planes: u16 = 1 as u16;         // Number of color planes, in BMP this is always 1
    let bits_per_pixel: u16 = 24 as u16;  // Number of bits per pixel
    file.write_le_u32(header_size);
    file.write_le_u32(image_width);
    file.write_le_u32(image_height);
    file.write_le_u16(planes);
    file.write_le_u16(bits_per_pixel);

    let compression_type: u32 = 0 as u32;    // 0 is uncompressed, 1 is RLE algorithm, 2 is 4-bit RLE algorithm
    let size_of_bitmap: u32 = 0 as u32; // Size in bytes, 0 when uncompressed
    let horizontal_resolution: u32 = 2835 as u32;  // In pixels per meter
    let vertical_resolution: u32 = 2835 as u32; // In pixels per meter
    let colors_used: u32 = 0 as u32;        // Number of colors in palette, 0 if no palette
    let colors_important: u32 = 0 as u32;   // 0 if all colors are important
    file.write_le_u32(compression_type);
    file.write_le_u32(size_of_bitmap);
    file.write_le_u32(horizontal_resolution);
    file.write_le_u32(vertical_resolution);
    file.write_le_u32(colors_used);
    file.write_le_u32(colors_important);

    // Color space info
    /*
    let red_mask: u32 = 0 as u32;
    let green_mask: u32 = 0 as u32;
    let blue_mask: u32 = 0 as u32;
    let alpha_mask: u32 = 0 as u32;
    let cs_type: u32 = 0 as u32;
    let endpoint_red_x: u32 = 0 as u32;
    let endpoint_red_y: u32 = 0 as u32;
    let endpoint_red_z: u32 = 0 as u32;
    let endpoint_green_x: u32 = 0 as u32;
    let endpoint_green_y: u32 = 0 as u32;
    let endpoint_green_z: u32 = 0 as u32;
    let endpoint_blue_x: u32 = 0 as u32;
    let endpoint_blue_y: u32 = 0 as u32;
    let endpoint_blue_z: u32 = 0 as u32;
    let gamma_red: u32 = 0 as u32;
    let gamma_green: u32 = 0 as u32;
    let gamma_blue: u32 = 0 as u32;
    file.write_le_u32(red_mask);
    file.write_le_u32(green_mask);
    file.write_le_u32(blue_mask);
    file.write_le_u32(alpha_mask);
    file.write_le_u32(cs_type);
    file.write_le_u32(endpoint_red_x);
    file.write_le_u32(endpoint_red_y);
    file.write_le_u32(endpoint_red_z);
    file.write_le_u32(endpoint_green_x);
    file.write_le_u32(endpoint_green_y);
    file.write_le_u32(endpoint_green_z);
    file.write_le_u32(endpoint_blue_x);
    file.write_le_u32(endpoint_blue_y);
    file.write_le_u32(endpoint_blue_z);
    file.write_le_u32(gamma_red);
    file.write_le_u32(gamma_green);
    file.write_le_u32(gamma_blue);
    */

    // Color Palette (only if bits_per_pixel == 1, 4, or 8)


    // After writing, 090903 should occur at byte 63 in testimage.bmp
    // Read every scanline from left to right, bottom up; but read each pixel value as BGR
    for y in range(0, self.height) {
      let bmp_y = self.height - 1 - y;
      for x in range(0, self.width) {
        match self.get_pixel(x,bmp_y) {
          Some(pixel) => {
            let blue = pixel.b;
            let green = pixel.g;
            let red = pixel.r;

            file.write_u8(blue);
            file.write_u8(green);
            file.write_u8(red);
          },
          None => {fail!("Error writing image as BMP file")}
        }
      }
      // Needs 0000 after every scanline, should occur at 1596 in testimage.bmp
      file.write_u8(0);
      file.write_u8(0);
    }
    true
  }
}

// Image processing traits and functions
trait PointProcessor {
  fn negative(&mut self);
  fn brighten(&mut self, bias: int);
  fn contrast(&mut self, gain: f32);
  fn saturate(&mut self, gain: f32);
  fn grayscale(&mut self);
}
impl PointProcessor for Image {
  fn negative(&mut self) {
    // Brute force        Time: 19257397 ns
    // let start = time::precise_time_ns();
    // let mut i = 0;
    // for i in range(0, self.data.len()) {
    //   self.data[i] = 255 - self.data[i];
    // }
    // let end = time::precise_time_ns();
    // let time = end as uint - start as uint;
    // println!("Time of brute force algorithm: {}", time);

    // Vectorize by 8     Time:  5118442 ns
    let start = time::precise_time_ns();
    let mut i = 0;
    let length = self.data.len();
    let remainder = length % 8;
    let difference = length - remainder;
    while i < difference {
      self.data[i] = 255 - self.data[i];
      self.data[i+1] = 255 - self.data[i+1];
      self.data[i+2] = 255 - self.data[i+2];
      self.data[i+3] = 255 - self.data[i+3];
      self.data[i+4] = 255 - self.data[i+4];
      self.data[i+5] = 255 - self.data[i+5];
      self.data[i+6] = 255 - self.data[i+6];
      self.data[i+7] = 255 - self.data[i+7];
      i += 8;
    }
    if remainder > 0 {
      for i in range(difference, length) {
        self.data[i] = 255 - self.data[i];
      }
    }
    let end = time::precise_time_ns();
    let time = end as uint - start as uint;
    println!("Time of vectorized algorithm: {}", time);
  }
  fn brighten(&mut self, bias: int) {
    // NOTE: For brightness, there are two ways of implementing: adding values to RGB values via a int bias value, or multiplying RGB values with a f32 gain value

    // Brute force        Time: 33111543 ns
    // let start = time::precise_time_ns();
    for x in range(0, self.width){
      for y in range(0, self.height){
        match self.get_pixel(x,y){
          Some(pixel) => {
            let mut red = pixel.r as int + bias;
            let mut green = pixel.g as int + bias;
            let mut blue = pixel.b as int + bias;

            if red > 255 {red = 255;}
            if green > 255 {green = 255;}
            if blue > 255 {blue = 255;}

            if red < 0 {red = 0;}
            if green < 0 {green = 0;}
            if blue < 0 {blue = 0;}
            
            self.set_pixel(x,y, RGB_Pixel{r: red as u8, g: green as u8, b: blue as u8});
          },
          None  => {fail!("Error retrieving pixel ({}, {})", x, y)}
        }
      }
    }
    // let end = time::precise_time_ns();
    // let time = end as uint - start as uint;
    // println!("Time of algorithm: {}", time);
  }
  fn contrast(&mut self, gain: f32) {
    let mut total_luminance: f32 = 0.;

    for x in range(0, self.width){
      for y in range(0, self.height){
        match self.get_pixel(x,y){
          Some(pixel) => {
            let mut red     = pixel.r as f32;
            let mut green   = pixel.g as f32;
            let mut blue    = pixel.b as f32;
            let luminance: f32 = (0.2126 * red  + 0.7152 * green  + 0.0722 * blue);
            total_luminance += luminance;
          },
          None  => {fail!("Error retrieving pixel ({}, {})", x, y)}
        }
      }
    }

    let mean_luminance: f32 = total_luminance/((self.width*self.height) as f32);

    for x in range(0, self.width){
      for y in range(0, self.height){
        match self.get_pixel(x,y){
          Some(pixel) => {
            let mut red     = pixel.r as int;
            let mut green   = pixel.g as int;
            let mut blue    = pixel.b as int;

            let dRed: f32 = red as f32 - mean_luminance;
            let dGreen: f32 = green as f32 - mean_luminance;
            let dBlue: f32 = blue as f32 - mean_luminance;

            red     = (red as f32 - dRed * (1. - gain)) as int;
            green   = (green as f32 - dGreen * (1. - gain)) as int;
            blue    = (blue as f32 - dBlue * (1. - gain)) as int;

            if red > 255 {red = 255;}
            if green > 255 {green = 255;}
            if blue > 255 {blue = 255;}

            if red < 0 {red = 0;}
            if green < 0 {green = 0;}
            if blue < 0 {blue = 0;}
            
            self.set_pixel(x,y, RGB_Pixel{r: red as u8, g: green as u8, b: blue as u8});

          },
          None  => {fail!("Error retrieving pixel ({}, {})", x, y)}
        }
      }
    }
  }
  // Aliasing introduced
  fn saturate(&mut self, gain: f32) {
    for x in range(0, self.width){
      for y in range(0, self.height){
        match self.get_pixel(x,y){
          Some(pixel) => {

            let mut red     = pixel.r as int;
            let mut green   = pixel.g as int;
            let mut blue    = pixel.b as int;

            let luminance: f32 = (0.2126 * red as f32 + 0.7152 * green as f32 + 0.0722 * blue as f32);
            let dRed: f32 = red as f32 - luminance;
            let dGreen: f32 = green as f32 - luminance;
            let dBlue: f32 = blue as f32 - luminance;

            red     = (red as f32 - dRed * (1. - gain)) as int;
            green   = (green as f32 - dGreen * (1. - gain)) as int;
            blue    = (blue as f32 - dBlue * (1. - gain)) as int;

            if red > 255 {red = 255;}
            if green > 255 {green = 255;}
            if blue > 255 {blue = 255;}

            if red < 0 {red = 0;}
            if green < 0 {green = 0;}
            if blue < 0 {blue = 0;}
            
            self.set_pixel(x,y, RGB_Pixel{r: red as u8, g: green as u8, b: blue as u8});
         
          },
          None  => {fail!("Error retrieving pixel ({}, {})", x, y)}
        }
      }
    }
  }
  fn grayscale(&mut self) {
    // NOTE: not optimized for format encoding, will be addressed later
    for x in range(0, self.width){
      for y in range(0, self.height){
        match self.get_pixel(x,y){
          Some(pixel) => {
            let mut red     = pixel.r as int;
            let mut green   = pixel.g as int;
            let mut blue    = pixel.b as int;

            let mut luminance = (0.2126 * red as f32 + 0.7152 * green as f32 + 0.0722 * blue as f32) as int;
            if luminance < 0 {
              luminance = 0;
            }
            if luminance > 255 {
              luminance = 255;
            }
            
            self.set_pixel(x,y, RGB_Pixel{r: luminance as u8, g: luminance as u8, b: luminance as u8});
          },
          None  => {fail!("Error retrieving pixel ({}, {})", x, y)}
        }
      }
    }
  }
}

trait Convolution {
  fn edge(&mut self);
  fn blur(&mut self);
  fn emboss(&mut self);
}
impl Convolution for Image {
  // Works
  fn blur(&mut self) {
    // Brute force        Time: 264835676 ns
    // let start = time::precise_time_ns();
  
    let kernel = [[1, 1, 1], [1, 1, 1], [1, 1, 1]];
    let kernel_sum = 9;
    let mut kernel_center_x: uint = 3/2;
    let mut kernel_center_y: uint = 3/2;

    for x in range(0, self.width){
      for y in range(0, self.height){

        let mut red_sum = 0;
        let mut green_sum = 0;
        let mut blue_sum = 0;

        for kernel_row in range(0, 3){
          for kernel_column in range(0, 3){

            let kx: int = kernel_row - (kernel_center_y - x) as int;
            let ky: int = kernel_column - (kernel_center_x - y) as int;

            if kx >= 0 && kx < (self.width as int) && ky >= 0 && ky < (self.height as int){

              let kernel_value = kernel[kernel_row as uint][kernel_column as uint];
              match self.get_pixel(kx as uint, ky as uint) {
                Some(pixel) => {
                  red_sum += (pixel.r as int * kernel_value);
                  green_sum += (pixel.g as int * kernel_value);
                  blue_sum += (pixel.b as int * kernel_value);
                },
                None  => {fail!("Error retrieving kernel pixel ({}, {}) at image pixel ({}, {})", kx, ky, x, y)}
              }

            }  

          }
        }

        red_sum = red_sum/kernel_sum;
        green_sum = green_sum/kernel_sum;
        blue_sum = blue_sum/kernel_sum;

        if red_sum > 255 {red_sum = 255;}
        if green_sum > 255 {green_sum = 255;}
        if blue_sum > 255 {blue_sum = 255;}

        if red_sum < 0 {red_sum = 0;}
        if green_sum < 0 {green_sum = 0;}
        if blue_sum < 0 {blue_sum = 0;}

        self.set_pixel(x as uint,y as uint, RGB_Pixel{r: red_sum as u8, g: green_sum as u8, b: blue_sum as u8});
        
      }
    }

    // let end = time::precise_time_ns();
    // let time = end as uint - start as uint;
    // println!("Time of brute force algorithm: {}", time);
  }

  fn edge(&mut self) {
    // Brute force        Time: 
    // let start = time::precise_time_ns();
    // let kernel = [[-1, -1, -1], [-1, 8, -1], [-1, -1, -1]];
    let kernel1 = [[-1, 0, 1], [-2, 0, 2], [-1, 0, 1]];    

    let number_of_kernel_rows = 3;
    let number_of_kernel_columns = 3;
    let mut kernel_center_x: uint = 3/2;
    let mut kernel_center_y: uint = 3/2;

    for y in range(0, self.height){
      for x in range(0, self.width){
        let mut red_sum: int = 0;
        let mut green_sum: int = 0;
        let mut blue_sum: int = 0;

        for kernel_row in range(0, number_of_kernel_rows){
          let mm = number_of_kernel_rows - 1 - kernel_row;

          for kernel_column in range(0, number_of_kernel_columns){
           let nn = number_of_kernel_columns - 1 - kernel_column;

            // Index of input signal
            let xx: int = x as int - (kernel_row - kernel_center_y as int);
            let yy: int = y as int - (kernel_column - kernel_center_x as int);
            
            if xx >= 0 && xx < (self.width as int) && yy >= 0 && yy < (self.height as int){
              let kernel_value = kernel1[mm as uint][nn as uint];
              match self.get_pixel(xx as uint, yy as uint) {
                Some(pixel) => {
                  red_sum += (pixel.r as int * kernel_value);
                  green_sum += (pixel.g as int * kernel_value);
                  blue_sum += (pixel.b as int * kernel_value);
                },
                None  => {fail!("Error retrieving kernel pixel ({}, {}) at image pixel ({}, {})", xx, yy, x, y)}
              }
            }  

          }
        }
      }
    }

        // println!("hi");
        // if y+1 < self.height && y-1 > 0 && x+1 < self.width && x-1 > 0 {
        //   red_sum = self.get_pixel(x-1,y-1).unwrap().r as int*kernel[0][0]
        //     + self.get_pixel(x,y-1).unwrap().r as int*kernel[0][1]
        //     + self.get_pixel(x+1,y-1).unwrap().r as int*kernel[0][2]
        //     + self.get_pixel(x-1,y).unwrap().r as int*kernel[1][0]
        //     + self.get_pixel(x,y).unwrap().r as int*kernel[1][1]
        //     + self.get_pixel(x+1,y).unwrap().r as int*kernel[1][2]
        //     + self.get_pixel(x-1,y+1).unwrap().r as int*kernel[2][0]
        //     + self.get_pixel(x,y+1).unwrap().r as int*kernel[2][1]
        //     + self.get_pixel(x+1,y+1).unwrap().r as int*kernel[2][2];

        //   green_sum = self.get_pixel(x-1,y-1).unwrap().g as int*kernel[0][0]
        //     + self.get_pixel(x,y-1).unwrap().g as int*kernel[0][1]
        //     + self.get_pixel(x+1,y-1).unwrap().g as int*kernel[0][2]
        //     + self.get_pixel(x-1,y).unwrap().g as int*kernel[1][0]
        //     + self.get_pixel(x,y).unwrap().g as int*kernel[1][1]
        //     + self.get_pixel(x+1,y).unwrap().g as int*kernel[1][2]
        //     + self.get_pixel(x-1,y+1).unwrap().g as int*kernel[2][0]
        //     + self.get_pixel(x,y+1).unwrap().g as int*kernel[2][1]
        //     + self.get_pixel(x+1,y+1).unwrap().g as int*kernel[2][2];

        //   blue_sum = self.get_pixel(x-1,y-1).unwrap().b as int*kernel[0][0]
        //     + self.get_pixel(x,y-1).unwrap().b as int*kernel[0][1]
        //     + self.get_pixel(x+1,y-1).unwrap().b as int*kernel[0][2]
        //     + self.get_pixel(x-1,y).unwrap().b as int*kernel[1][0]
        //     + self.get_pixel(x,y).unwrap().b as int*kernel[1][1]
        //     + self.get_pixel(x+1,y).unwrap().b as int*kernel[1][2]
        //     + self.get_pixel(x-1,y+1).unwrap().b as int*kernel[2][0]
        //     + self.get_pixel(x,y+1).unwrap().b as int*kernel[2][1]
        //     + self.get_pixel(x+1,y+1).unwrap().b as int*kernel[2][2];
        // }


    let kernel2 = [[1, 2, 1], [0, 0, 0], [-1, -2, -1]];    

    for y in range(0, self.height){
      for x in range(0, self.width){
        let mut red_sum: int = 0;
        let mut green_sum: int = 0;
        let mut blue_sum: int = 0;

        for kernel_row in range(0, number_of_kernel_rows){
          let mm = number_of_kernel_rows - 1 - kernel_row;

          for kernel_column in range(0, number_of_kernel_columns){
           let nn = number_of_kernel_columns - 1 - kernel_column;

            // Index of input signal
            let xx: int = x as int - (kernel_row - kernel_center_y as int);
            let yy: int = y as int - (kernel_column - kernel_center_x as int);
            
            if xx >= 0 && xx < (self.width as int) && yy >= 0 && yy < (self.height as int){
              let kernel_value = kernel2[mm as uint][nn as uint];
              match self.get_pixel(xx as uint, yy as uint) {
                Some(pixel) => {
                  red_sum += (pixel.r as int * kernel_value);
                  green_sum += (pixel.g as int * kernel_value);
                  blue_sum += (pixel.b as int * kernel_value);
                },
                None  => {fail!("Error retrieving kernel pixel ({}, {}) at image pixel ({}, {})", xx, yy, x, y)}
              }
            }  

          }
        }


        if red_sum > 255 {red_sum = 255;}
        if green_sum > 255 {green_sum = 255;}
        if blue_sum > 255 {blue_sum = 255;}

        if red_sum < 0 {red_sum = 0;}
        if green_sum < 0 {green_sum = 0;}
        if blue_sum < 0 {blue_sum = 0;}

        println!("Pixel color ({}, {}, {})", red_sum, green_sum, blue_sum);
        self.set_pixel(x as uint,y as uint, RGB_Pixel{r: red_sum as u8, g: green_sum as u8, b: blue_sum as u8});    
      }
    }
    // let end = time::precise_time_ns();
    // let time = end as uint - start as uint;
    // println!("Time of brute force algorithm: {}", time);
  }

  fn emboss(&mut self) {
    // Brute force        Time: 
    // let start = time::precise_time_ns();
  
    let kernel = [[-1, -1, 0], [-1, 0, 1], [0, 1, 1]];
    let bias = 128;
    let mut kernel_center_x: uint = 3/2;
    let mut kernel_center_y: uint = 3/2;

    let mut initial_red = 0;
    let mut initial_green = 0;
    let mut initial_blue = 0;

    for x in range(0, self.width){
      for y in range(0, self.height){

        let mut red_sum = 0;
        let mut green_sum = 0;
        let mut blue_sum = 0;

        for kernel_row in range(0, 3){
          for kernel_column in range(0, 3){

            let kx: int = kernel_row - (kernel_center_y - x) as int;
            let ky: int = kernel_column - (kernel_center_x - y) as int;
            
            if kx >= 0 && kx < (self.width as int) && ky >= 0 && ky < (self.height as int){
              let kernel_value = kernel[kernel_row as uint][kernel_column as uint];
              match self.get_pixel(kx as uint, ky as uint) {
                Some(pixel) => {
                  red_sum += (pixel.r as int * kernel_value);
                  green_sum += (pixel.g as int * kernel_value);
                  blue_sum += (pixel.b as int * kernel_value);
                  // println!("r: {}, g: {}, b:{}", red_sum, green_sum, blue_sum);
                  //println!("\tKernel pixel: ({},{},{}) at ({},{})", pixel.r, pixel.g, pixel.b, kx, ky);
                },
                None  => {fail!("Error retrieving kernel pixel ({}, {}) at image pixel ({}, {})", kx, ky, x, y)}
              }
            }  

          }
        }

        // red_sum = red_sum/kernel_sum;
        // green_sum = green_sum/kernel_sum;
        // blue_sum = blue_sum/kernel_sum;

        red_sum += bias;
        green_sum += bias;
        blue_sum += bias;

        if red_sum > 255 {red_sum = 255;}
        if green_sum > 255 {green_sum = 255;}
        if blue_sum > 255 {blue_sum = 255;}

        if red_sum < 0 {red_sum = 0;}
        if green_sum < 0 {green_sum = 0;}
        if blue_sum < 0 {blue_sum = 0;}

        // println!("Output pixel: ({},{},{}) at ({},{})", red_sum, green_sum, blue_sum, x, y);
        self.set_pixel(x as uint,y as uint, RGB_Pixel{r: red_sum as u8, g: green_sum as u8, b: blue_sum as u8});
        
      }
    }

    // let end = time::precise_time_ns();
    // let time = end as uint - start as uint;
    // println!("Time of brute force algorithm: {}", time);
  }
}

fn main() {
  let args = os::args();
  if args.len() < 2 {
    fail!("Image path not provided");
  }
  else {
    println!("Path to image: {}", args[1]);

    // Read --> Write     Checklist
    // PPM  --> PPM       Works
    // PPM  --> BMP       Works
    // BMP  --> PPM       Works
    // BMP  --> BMP       Works

    let mut image = Image::read_bmp(args[1]);
    image.write_ppm("image.ppm");


    // // Point Processor
    // let mut image1 = Image::read_bmp(args[1]);
    // image1.negative();
    // image1.write_bmp("output1.bmp");

    // let mut image2 = Image::read_bmp(args[1]);
    // image2.brighten(100);
    // image2.write_bmp("output2.bmp");

    // let mut image3 = Image::read_bmp(args[1]);
    // image3.contrast(12.5);
    // image3.write_bmp("output3.bmp");

    // let mut image4 = Image::read_bmp(args[1]);
    // image4.saturate(12.5);
    // image4.write_bmp("output4.bmp");

    // let mut image5 = Image::read_bmp(args[1]);
    // image5.grayscale();
    // image5.write_bmp("output5.bmp");    


    // // Convolution Filter
    // let mut image6 = Image::read_bmp(args[1]);
    // image6.blur();
    // image6.write_bmp("output6.bmp");

  }

}



































// Need better understand of byte-wise file writing before implementing
pub struct PNG {
  width: uint,
  height: uint,
  data: ~[u8]
}
impl PNG {
  // All notes based on W3 Documentation:
  // http://www.w3.org/TR/PNG/#5PNG-file-signature

  // Magic Number (ASCII/decimal):
  //    89 P N G 0d 0a 1a 0a
  // or 
  //    0x89 0x50 0x4E 0x47 0x0D 0x0A 0x1A 0x0A

  // Chunk Format:
  // Length - Type - Data - CRC
  // 4 bytes - 4 bytes - [length] bytes - 4 byte

  /******** Critical Chunks ********/
  // IHDR Chunk: 0x49 0x48 0x44 0x52
  // PLTE Chunk: 0x50 0x4C 0x54 0x45
  // IDAT Chunk: 0x49 0x44 0x41 0x54
  // IEND Chunk: 0x49 0x45 0x4E 0x44 
  // These chunks are absolutely required to render PNG images

  /* IHDR                 (Remember: 2 hex digits = 1 byte)
     Width: 4 bytes       (1px = 0x00000001)
     Height: 4 bytes
     Bit depth: 1 byte              Must be 0x1, 0x2, 0x4, 0x8, or 0x16, depends on color type
     Color type: 1 byte             Must be 0x00, 0x02, 0x03, 0x04 or 0x06
     Compression method: 1 byte     Not implementing, set to 0x00
     Filter method: 1 byte          Not implementing, set to 0x00
     Interlace method: 1 byte       Not implementing, set to 0x00
  */
  // fn png_IHDR(width: uint, height: uint) -> ~[u8]{
  //   let IHDR: ~[u8] = [0x49, 0x48, 0x44, 0x52, ];
  //   return ()
  // }

  /* PLTE Chunk is ...
   * Required for indexed color,
   * Optional for truecolor & truecolor with alpha, 
   * Cannot exist for grayscale & grayscale with alpha
   */

  pub fn new(width: uint, height: uint) -> PNG {
    let size = 4 * height * width;
    let mut buffer = from_elem(size, 0u8);
    PNG{width: width, height: height, data: buffer}
  }
  
  // pub fn write_file(&self, filename: &str) -> bool {
  //   let path = Path::new(filename);
  //   let mut file = File::create(&path);

  //   let header = format!("89 50 4E 47 0D 0A 1A 0A");
  //   file.write(header.as_bytes());
  //   file.write(self.data);

  //   true
  // }
}