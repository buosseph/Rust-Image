//extern crate time;

use std::slice::from_elem;
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
pub enum ColorType {    // Not yet implemented
  GRAYSCALE = 1,
  RGB = 24,
  RGBA = 32,
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

// BMP 3.x & 4.x Image formats
impl Image {
  /* NOTES:
   * BMP pixels stored as BGR, not RGB
   * If height is positive, scanlines stored BOTTOM UP --> store pixels starting from bottom row when writing
   * If height is negative, scanliens stored TOP DOWN  --> No flip required to match Image struct pixel array orientation
   * Image_width % 4 = # of bytes for padding per scanline
   */

  // Good example for furture development of image format: http://www.kalytta.com/bitmap.h
  // See http://msdn.microsoft.com/en-us/library/dd183381(v=vs.85).aspx for more information on meta data in various headers

  pub fn read_bmp(image_path_str: &str) -> Image{
    let path = Path::new(image_path_str);

    let mut signature: ~[u8] = ~[0 as u8, 0 as u8];
    let mut file_size: u32 = 0 as u32;      
    let mut offset: u32 = 0 as u32;
    let mut header_size: u32 = 0 as u32;      // 40 = BMP 3.x, 108 = BMP 4.x, 124 = BMP 5.x
    let mut image_width: u32 = 0 as u32;
    let mut image_height: u32 = 0 as u32;
    let mut planes: u16 = 0 as u16;
    let mut bits_per_pixel: u16 = 0 as u16;   // 1 = Monochrome (not grayscale), 24 = RGB, 32 = RGBA
    let mut compression_type: u32 = 0 as u32;
    let mut size_of_bitmap: u32 = 0 as u32;   

    let mut image_data_bytes: ~[u8] = ~[];
    let mut buffer: ~[u8] = ~[];


    match File::open(&path) {
      Ok(mut image) => {
        match image.read(signature) {
          Ok(num_of_bytes) =>  {
            match str::from_utf8_owned(signature) {
              Some(read_signature)  => {
                if !str::eq(&read_signature, &~"BM") {
                  fail!("Input image is not a valid BMP image");
                }
              },
              None => {fail!("Error wrong reading file signature")}
            }
          },
          Err(e) => {println!("Error reading BMP file signature: {}", e)}
        }

        // Total file size
        match image.read_le_u32() {
          Ok(read_file_size) => {
            file_size = read_file_size;
          },
          Err(e)  => {println!("Error reading the filesize: {}", e)}
        }
        // Reserved & pixel data offset
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

        // Bitmap header size
        match image.read_le_u32() {
          Ok(read_header_size) => {
            header_size = read_header_size;
          },
          Err(e) => (println!("Error reading bitmap header size: {}", e))
        }
        // Image width
        match image.read_le_u32() {
          Ok(read_image_width) => {
            image_width = read_image_width;
          },
          Err(e) => (println!("Error reading image width: {}", e))
        }
        // Image height
        match image.read_le_u32() {
          Ok(read_image_height) => {
            image_height = read_image_height;
          },
          Err(e) => (println!("Error reading image height: {}", e))
        }
        // Number of planes
        match image.read_le_u16() {
          Ok(read_planes) => {
            planes = read_planes;
          },
          Err(e) => (println!("Error reading bitmap planes: {}", e))
        }
        // Number of components
        match image.read_le_u16() {
          Ok(read_bits_per_pixel) => {
            bits_per_pixel = read_bits_per_pixel;
          },
          Err(e) => (println!("Error reading bitmap planes: {}", e))
        }
        // Compression (type)
        match image.read_le_u32() {
          Ok(read_compression_type) => {
            compression_type = read_compression_type;
          },
          Err(e) => (println!("Error reading image height: {}", e))
        }
        // Bitmap size
        match image.read_le_u32() {
          Ok(read_bitmap_size) => {
            size_of_bitmap = read_bitmap_size;
          },
          Err(e) => (println!("Error reading image height: {}", e))
        }

        let mut remainder = offset as int - 14 - 24; // offset - fileheader size - read bytes
        // BMP 3.x
        if header_size as int == 40 {
          println!("Reading BMP 3.x");
          for i in range(0, remainder) {
            match image.read_byte() {
              Ok(byte)  => {continue},
              Err(e)    => {fail!("Error reading BMP 3.x header: {}", e)}
            }
          }
          if compression_type as int == 0 {
            if bits_per_pixel as int == 24 {
              for y in range(0, image_height as int) {
                for x in range(0, image_width as int) {
                  match image.read_exact(3) {
                    Ok(mut pixel_data) => {
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

                // Padding based on image width, scanlines must be multiple of 4
                match image_width % 4 {
                  1 => {
                    match image.read_byte() {
                      Ok(padding) => {
                        if padding as uint == 0 {
                          continue;
                        }
                        else {
                          break;
                          fail!("Error reading padding at end of scanline");
                        }
                      },
                      Err(e) => {
                        fail!("Error checking padding at end of scanline");
                      }
                    }
                  },
                  2 => {
                    match image.read_le_u16() {
                      Ok(padding) => {
                        if padding as uint == 0 {
                          continue;
                        }
                        else {
                          break;
                          fail!("Error reading padding at end of scanline");
                        }
                      },
                      Err(e) => {
                        fail!("Error checking padding at end of scanline");
                      }
                    }
                  },
                  3 => {
                    match image.read_byte() {
                      Ok(padding) => {
                        if padding as uint == 0 {
                          match image.read_le_u16() {
                            Ok(padding) => {
                              if padding as uint == 0 {
                                continue;
                              }
                              else {
                                break;
                                fail!("Error reading padding at end of scanline");
                              }
                            },
                            Err(e) => {
                              fail!("Error checking padding at end of scanline");
                            }
                          }
                        }
                        else {
                          break;
                          fail!("Error reading padding at end of scanline");
                        }
                      },
                      Err(e) => {
                        fail!("Error checking padding at end of scanline");
                      }
                    }
                  },
                  _ => {
                    continue;
                  }
                }

              }              
            }
          }

        }
        // BMP 4.x
        else {
          for i in range(0, remainder) {
            match image.read_byte() {
              Ok(byte)  => {continue},
              Err(e)    => {fail!("Error reading BMP header: {}", e)}
            }
          }
          if compression_type as int == 0 {    
            if bits_per_pixel as int == 24 {
              for y in range(0, image_height) {
                for x in range(0, image_width) {
                  match image.read_exact(3) {
                    Ok(mut pixel_data) => {
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

                // Padding based on image width, scanlines must be multiple of 4
                match image_width % 4 {
                  1 => {
                    match image.read_byte() {
                      Ok(padding) => {
                        if padding as uint == 0 {
                          continue;
                        }
                        else {
                          break;
                          fail!("Error reading padding at end of scanline");
                        }
                      },
                      Err(e) => {
                        fail!("Error checking padding at end of scanline");
                      }
                    }
                  },
                  2 => {
                    match image.read_le_u16() {
                      Ok(padding) => {
                        if padding as uint == 0 {
                          continue;
                        }
                        else {
                          break;
                          fail!("Error reading padding at end of scanline");
                        }
                      },
                      Err(e) => {
                        fail!("Error checking padding at end of scanline");
                      }
                    }
                  },
                  3 => {
                    match image.read_byte() {
                      Ok(padding) => {
                        if padding as uint == 0 {
                          match image.read_le_u16() {
                            Ok(padding) => {
                              if padding as uint == 0 {
                                continue;
                              }
                              else {
                                break;
                                fail!("Error reading padding at end of scanline");
                              }
                            },
                            Err(e) => {
                              fail!("Error checking padding at end of scanline");
                            }
                          }
                        }
                        else {
                          break;
                          fail!("Error reading padding at end of scanline");
                        }
                      },
                      Err(e) => {
                        fail!("Error checking padding at end of scanline");
                      }
                    }
                  },
                  _ => {
                    continue;
                  }
                }

              }
            }
          }                   
        }
      },
      Err(e)  => {println!("Error opening file: {}", e)}
    }
    // Without this scanlines are flipped in image data
    if image_height as int > 0 {
      if bits_per_pixel as int == 24 {
        for i in range(0, image_height){
          let start_index: uint = (image_height as uint - i as uint - 1) * image_width as uint * 3;  // 3 because RGB
          let end_index: uint = start_index + (image_width as uint * 3); // Off by one as slice function doesn't include last index

          let scanline = buffer.slice(start_index, end_index);
          image_data_bytes.push_all(scanline);
        }
      }
    }
    Image{width: image_width as uint, height: image_height as uint, color_type: RGB, data: image_data_bytes}
  }

  pub fn write_bmp(&mut self, filename: &str) -> bool {
    let path = Path::new(filename);
    let mut file = File::create(&path);
    let version = 4;  // For testing purposes
    let signature = "BM";

    // Save as BMP 4.x
    if version == 4 {
      match self.color_type {
        // No padding needed for RGBA
        RGB => {
          let filesize: u32 = ((self.width * self.height * 3) + 108 + 14) as u32; 
          let reserved1: u16 = 0 as u16;
          let reserved2: u16 = 0 as u16;
          let bitmap_offset: u32 = 122 as u32; // Bitmap 3.x => 54, Bitmap 4.x => 122
          file.write(signature.as_bytes());
          file.write_le_u32(filesize);
          file.write_le_u16(reserved1);
          file.write_le_u16(reserved2);
          file.write_le_u32(bitmap_offset);

          let header_size: u32 = 108 as u32;  // Size in bytes
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
          let size_of_bitmap: u32 = (self.width * self.height * 3) as u32; // Size in bytes, 0 when uncompressed = 0
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

          let red_mask: u32 = 0x00FF0000 as u32; //BGRs when not compressed? This is unclear
          let green_mask: u32 = 0x0000FF00 as u32;
          let blue_mask: u32 = 0x000000FF as u32;
          let alpha_mask: u32 = 0x00000000 as u32;
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

          if compression_type == 0 {
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

              // Padding based on image width, scanlines must be multiple of 4
              match image_width % 4 {
                1 => {
                  file.write_u8(0);
                },
                2 => {
                  file.write_u8(0);
                  file.write_u8(0);
                },
                3 => {
                  file.write_u8(0);
                  file.write_u8(0);
                  file.write_u8(0);
                },
                _ => {
                  continue;
                }
              }
            }
          }
          true
        },
        _ => {false},
      }
    }
    // Save as BMP 3.x
    else if version == 3 {
      /* Total filesize in bytes (File header guaranteed 14 bytes)
       *    Bitmap 3.x => 54 + 3 * width * height bytes
       */
      let filesize: u32 = ((self.width * self.height * 3) + 40 + 14) as u32; 
      let reserved1: u16 = 0 as u16;
      let reserved2: u16 = 0 as u16;
      let bitmap_offset: u32 = 54 as u32; // Bitmap 3.x => 54, Bitmap 4.x => 108
      file.write(signature.as_bytes());
      file.write_le_u32(filesize);
      file.write_le_u16(reserved1);
      file.write_le_u16(reserved2);
      file.write_le_u32(bitmap_offset);

      // Bitmap 3.x Header, 40 bytes in size (no colorspace info)
      let header_size: u32 = 40 as u32;  // Size in bytes
      let image_width: u32 = self.width as u32;    // In pixels
      let image_height: u32 = self.height as u32;   // In pixels
      let planes: u16 = 1 as u16;         // Always 1
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

      // Color Palette (only if bits_per_pixel == 1, 4, or 8)

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
        // Padding based on image width, scanlines must be multiple of 4
        match image_width % 4 {
          1 => {
            file.write_u8(0);
          },
          2 => {
            file.write_u8(0);
          },
          3 => {
            file.write_u8(0);
            file.write_u8(0);
            file.write_u8(0);
          },
          _ => {
            continue;
          }
        }

      }
      true
    }
    else {
      false
    }
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
    // Vectorize by 8     Time:  5118442 ns
    //let start = time::precise_time_ns();
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
    // let end = time::precise_time_ns();
    // let time = end as uint - start as uint;
    // println!("Time of vectorized algorithm: {}", time);
  }
  fn brighten(&mut self, bias: int) {
    // Brute force        Time: 33111543 ns
    // let start = time::precise_time_ns();
    for y in range(0, self.height){
      for x in range(0, self.width){
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

    for y in range(0, self.height){
      for x in range(0, self.width){
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

    for y in range(0, self.height){
      for x in range(0, self.width){
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
  fn saturate(&mut self, gain: f32) {
    for y in range(0, self.height){
      for x in range(0, self.width){
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
    // NOTE: not optimized for format encoding
    for y in range(0, self.height){
      for x in range(0, self.width){
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

trait ConvolutionFilter {
  fn blur(&mut self);
}
impl ConvolutionFilter for Image {
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
}

fn main() {
  let args = os::args();
  if args.len() < 2 {
    fail!("Image path not provided");
  }
  else {
    let path_string = args[1].clone();
    let save_file = args[2].clone();
    let processor = args[3].clone();

    println!("Path to image: {}", &path_string);
    let mut image;
    if path_string.contains(&".ppm") {
      image = Image::read_ppm(path_string);
    }
    else if path_string.contains(&".bmp") {
      image = Image::read_bmp(path_string);
    }
    else {
      fail!("Couldn't read given image format");
    }

    if str::eq(&processor, &~"negative") {
      image.negative();
    }
    else if str::eq(&processor, &~"brighten") {
      image.brighten(125);
    }
    else if str::eq(&processor, &~"contrast") {
      image.contrast(2.5);
    }
    else if str::eq(&processor, &~"saturate") {
      image.saturate(2.5);
    }
    else if str::eq(&processor, &~"grayscale") {
      image.grayscale();
    }
    else if str::eq(&processor, &~"blur") {
      image.blur();
    }

    if save_file.contains(".bmp") {
      image.write_bmp(save_file);
    }
    else if save_file.contains(".ppm") {
      image.write_ppm(save_file);
    }
    else {
      fail!("Couln't save image");
    }

  }

  //   // Read     --> Write         Checklist
  //   // PPM      --> PPM           Works
  //   // PPM      --> BMP 3.x       Works
  //   // BMP 3.x  --> PPM           Works
  //   // BMP 3.x  --> BMP 3.x       Works

  //   // BMP 4.x  --> PPM           Works
  //   // BMP 4.x  --> BMP 4.x       Works
  //   // PPM      --> BMP 4.x       Works
  //   // BMP 3.x  --> BMP 4.x       Works

}
