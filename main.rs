//extern crate time;

#![allow(unused_imports)]


use std::slice::from_elem;
use std::path::posix::{Path};
use std::io::File;
use std::os;
use std::str;
use std::uint;

pub enum ColorType {
  GRAYSCALE8 = 8,
  RGB8 = 24,
  RGBA8 = 32,
}
pub struct Image {
  width: uint,
  height: uint,
  color_type: ColorType,
  data: ~[u8],
}
impl Image {

  #[allow(dead_code)]
  pub fn new(width: uint, height: uint, color_type: ColorType) -> Image {
    match color_type {
      GRAYSCALE8   => {
        let size = width * height;
        let buffer = from_elem(size, 0u8);
        Image{width: width, height: height, color_type: GRAYSCALE8, data: buffer}
      },
      RGB8         => {
        let size = 3 * width * height;
        let buffer = from_elem(size, 0u8);
        Image{width: width, height: height, color_type: RGB8, data: buffer}
      },
      RGBA8        => {
        let size = 4 * width * height;
        let buffer = from_elem(size, 0u8);
        Image{width: width, height: height, color_type: RGBA8, data: buffer}
      }
    }
  }

  fn buffer_size(&self) -> uint {
    match self.color_type {
      GRAYSCALE8   => {  self.width * self.height    },
      RGB8         => {  self.width * self.height * 3},
      RGBA8        => {  self.width * self.height * 4}
    }
  }
 
  fn get_offset(&self, x: uint, y: uint) -> Option<uint> {
    match self.color_type {
      GRAYSCALE8 => {
        let offset = x + self.width * y;
        if offset < self.buffer_size() {
          Some(offset)
        }else{
          None
        }        
      },
      RGB8 => {
        let offset = (x + self.width * y) * 3;
        if offset < self.buffer_size() {
          Some(offset)
        }else{
          None
        }
      },
      RGBA8 => {
        let offset = (x + self.width * y) * 4;
        if offset < self.buffer_size() {
          Some(offset)
        }else{
          None
        }        
      }
    }
  }

  #[allow(dead_code)]
  pub fn get_pixel(&self, x: uint, y: uint) -> Vec<u8>{
    match self.color_type {
      GRAYSCALE8 => {
        match self.get_offset(x, y) {
          Some(offset) => {
            let pixel_data: Vec<u8> = vec!(self.data[offset]);
            pixel_data
          },
          None => {fail!("Couldn't get RGB8 pixel")}
        }        
      },
      RGB8 => {
        match self.get_offset(x, y) {
          Some(offset) => {
            let pixel_data: Vec<u8> = vec!(
              self.data[  offset      ],
              self.data[  offset + 1  ],
              self.data[  offset + 2  ]
              );
            pixel_data
          },
          None => {fail!("Couldn't get RGB8 pixel")}
        }
      },
      RGBA8 => {
        match self.get_offset(x, y) {
          Some(offset) => {
            let pixel_data: Vec<u8> = vec!(
              self.data[  offset      ],
              self.data[  offset + 1  ],
              self.data[  offset + 2  ],
              self.data[  offset + 3  ]
              );
            pixel_data
          },
          None => {fail!("Couldn't get RGB8 pixel")}
        }        
      }
    }
  }
 
  #[allow(dead_code)]
  pub fn set_pixel(&mut self, x: uint, y: uint, mut color: Vec<u8>) -> bool {
    match self.color_type {
      GRAYSCALE8 => {
        match self.get_offset(x, y) {
          Some(offset) => {
            self.data[  offset      ] = color.pop().unwrap();
            true
          },
          None => false
        }           
      },
      RGB8 => {
        match self.get_offset(x, y) {
          Some(offset) => {
            self.data[  offset + 2  ] = color.pop().unwrap();
            self.data[  offset + 1  ] = color.pop().unwrap();
            self.data[  offset      ] = color.pop().unwrap();
            true
          },
          None => false
        }        
      },
      RGBA8 => {
        match self.get_offset(x, y) {
          Some(offset) => {
            self.data[  offset + 3  ] = color.pop().unwrap();
            self.data[  offset + 2  ] = color.pop().unwrap();
            self.data[  offset + 1  ] = color.pop().unwrap();
            self.data[  offset      ] = color.pop().unwrap();
            true
          },
          None => false
        }         
      }
    }
  }


  // Conversion functions not yet tested for need of update

  #[allow(dead_code)]
  pub fn convert_to_GRAYSCALE8(&mut self) -> bool {
    match self.color_type {
      GRAYSCALE8 => {
        println!("Image already GRAYSCALE8");
        return true
      },
      RGB8      => {
        let mut new_pixel_array: ~[u8] = ~[];
        for y in range(0, self.height){
          for x in range(0, self.width){
            let mut pixel_data: Vec<u8> = self.get_pixel(x,y);
            let b  = pixel_data.pop().unwrap();
            let g  = pixel_data.pop().unwrap();
            let r  = pixel_data.pop().unwrap();

            let red     = r as int;
            let green   = g as int;
            let blue    = b as int;

            let mut luminance = (0.2126 * red as f32 + 0.7152 * green as f32 + 0.0722 * blue as f32) as int;
            if luminance < 0 {
              luminance = 0;
            }
            if luminance > 255 {
              luminance = 255;
            }

            new_pixel_array.push(luminance as u8);
          }
        }
        self.data = new_pixel_array;
        self.color_type = GRAYSCALE8;
        true
      },
      RGBA8     => {
        let mut new_pixel_array: ~[u8] = ~[];
        for y in range(0, self.height){
          for x in range(0, self.width){
            let mut pixel_data: Vec<u8> = self.get_pixel(x,y);
            pixel_data.pop().unwrap();
            let b  = pixel_data.pop().unwrap();
            let g  = pixel_data.pop().unwrap();
            let r  = pixel_data.pop().unwrap();

            let red     = r as int;
            let green   = g as int;
            let blue    = b as int;

            let mut luminance = (0.2126 * red as f32 + 0.7152 * green as f32 + 0.0722 * blue as f32) as int;
            if luminance < 0 {
              luminance = 0;
            }
            if luminance > 255 {
              luminance = 255;
            }

            new_pixel_array.push(luminance as u8);
          }
        }
        self.data = new_pixel_array;
        self.color_type = GRAYSCALE8;
        true
      }
    }
  } 

  #[allow(dead_code)]
  pub fn convert_to_RGB8(&mut self) -> bool {
    match self.color_type {
      GRAYSCALE8 => {
        let mut new_pixel_array: ~[u8] = ~[];
        for i in range(0, self.data.len()) {
          let lum = self.data[i];
          new_pixel_array.push(lum);
          new_pixel_array.push(lum);
          new_pixel_array.push(lum);
        }
        self.data = new_pixel_array;
        self.color_type = RGB8;
        true
      },
      RGB8      => {
        println!("Image already RGB8");
        return true
      },
      RGBA8     => {
        let mut new_pixel_array: ~[u8] = ~[];
        for i in range(0, self.data.len()) {
          if i % 4 ==3 {
            continue;
          }
          let component = self.data[i];
          new_pixel_array.push(component);
        }
        self.data = new_pixel_array;
        self.color_type = RGB8;
        true
      }
    }
  }

  #[allow(dead_code)]
  pub fn convert_to_RGBA8(&mut self) -> bool {
    match self.color_type {
      GRAYSCALE8 => {
        let mut new_pixel_array: ~[u8] = ~[];
        for i in range(0, self.data.len()) {
          let lum = self.data[i];
          new_pixel_array.push(lum);
          new_pixel_array.push(lum);
          new_pixel_array.push(lum);
          new_pixel_array.push(255 as u8);
        }
        self.data = new_pixel_array;
        self.color_type = RGBA8;
        true
      },
      RGB8      => {
        let mut new_pixel_array: ~[u8] = ~[];
        for y in range(0, self.height) {
          for x in range(0, self.width) {
            let mut pixel: Vec<u8> = self.get_pixel(x,y);
            let b = pixel.pop().unwrap();
            let g = pixel.pop().unwrap();
            let r = pixel.pop().unwrap();

            new_pixel_array.push(r);
            new_pixel_array.push(g);
            new_pixel_array.push(b);
            new_pixel_array.push(255 as u8);
          }
        }
        self.data = new_pixel_array;
        self.color_type = RGBA8;
        true
      },
      RGBA8     => {
        println!("Image already RGBA8");
        return true
      }
    }
  }  


  pub fn get_width(&self) -> uint { self.width }
  pub fn get_height(&self) -> uint { self.height }
  pub fn get_color_type(&self) -> ColorType { self.color_type }  
}

// PPM Image format
/*impl Image {  // Not complete, and may never be
  #[allow(dead_code)]
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
    Image{width: width, height: height, color_type: RGB8, data: image_data_bytes}
  }

  #[allow(dead_code)]
  fn write_ppm(&self, filename: &str) -> bool {
    let path = Path::new(filename);
    let mut file = File::create(&path);
    let header = format!("P6 {} {} 255\n", self.width, self.height);
    file.write(header.as_bytes()).unwrap();
    file.write(self.data).unwrap();
    true
  }
}*/

// BMP Image format
trait BMP {
  fn read_image(image_path_str: &str) -> Option<Self>;
  fn write_image(&mut self, filename: &str) -> bool;
}
impl BMP for Image {
  /* NOTES:
   * BMP pixels stored as BGR, not RGB8
   * If height is positive, scanlines stored BOTTOM UP --> store pixels starting from bottom row when writing
   * If height is negative, scanliens stored TOP DOWN  --> No flip required to match Image struct pixel array orientation
   * Image_width % 4 = # of bytes for padding per scanline
   * Only v4 an v5 can produce RGBA8 images
   */

  /* Reader Status:
   *  - 24-bit read correctly
   *  - 8-bit read as 24-bit images (need color pallete for GRAYSCALE8 images)
   *  - 32-bit read correctly
   */

  #[allow(dead_code)]
  #[allow(unused_variable)]
  #[allow(dead_assignment)]
  fn read_image(image_path_str: &str) -> Option<Image>{
    let path = Path::new(image_path_str);


    let mut signature: ~[u8] = ~[0 as u8, 0 as u8];
    let mut file_size: u32 = 0 as u32;      
    let mut offset: u32 = 0 as u32;
    let mut header_size: u32 = 0 as u32;      // 40 = BMPv3, 108 = BMPv4, 124 = BMPv5
    let mut image_width: u32 = 0 as u32;
    let mut image_height: u32 = 0 as u32;
    let mut planes: u16 = 0 as u16;
    let mut bits_per_pixel: u16 = 0 as u16;   // 8 = GRAYSCALE8, 24 = RGB8, 32 = RGBA8
    let mut compression_type: u32 = 0 as u32;
    let mut size_of_bitmap: u32 = 0 as u32;   

    let mut image_data_bytes: ~[u8] = ~[];
    let mut buffer: ~[u8] = ~[];


    match File::open(&path) {
      Ok(mut image) => {
        match image.read(signature) {
          Ok(_) =>  {
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
          Ok(_) => {
            match image.read_le_u16() {
              Ok(_) => {
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

        let remainder = offset as int - 14 - 24; // offset - fileheader size - read bytes

        // Debug header data
        /*println!("
          Total file size (in bytes): {}
          Pixel data starting byte: {}
          Header size: {}\t(40 = BMP 3.x, 108 = BMP 4.x, 124 = BMP 5.x)
          Dimensions: {}px x {}px
          Number of color planes: {}\t(Should always be 1 in BMPs)
          Bits per pixel: {}\t(8 = GRAYSCALE8, 24 = RGB8, 32 = RGBA8)
          Compression type: {}
          Size of bitmap (in bytes): {}\t(May be 0 if uncompressed)
          
          Number of bytes left unread in header: {}
          ",
          file_size,
          offset,
          header_size,
          image_width,
          image_height,
          planes,
          bits_per_pixel,
          compression_type,
          size_of_bitmap,
          remainder
        );*/

        for _ in range(0, remainder) {
          match image.read_byte() {
            Ok(_)  => {continue},
            Err(e)    => {fail!("Error reading BMP header: {}", e)}
          }
        }

        // BI_RGB means uncompressed (BGR)
        if compression_type as int == 0 {
          // GRAYSCALE8
          if bits_per_pixel as int == 8 {
            println!("GRAYSCALE8 Image");
            for y in range(0, image_height) {
              for x in range(0, image_width) {
                match image.read_byte() {
                  Ok(pixel_data) => {
                    // Saving as RGB8
                    buffer.push(pixel_data);
                    // buffer.push(pixel_data);
                    // buffer.push(pixel_data);
                  },
                  Err(e)    => {fail!("Error reading BMP pixel at ({}, {}): {}", x, y, e)}
                }
              }

              // Padding based on image width, all scanlines must be multiple of 4
              match image_width % 4 {
                1 => {
                  match image.read_byte() {
                    Ok(padding) => {
                      if padding as uint == 0 {
                        continue;
                      }
                      else {
                        break;
                      }
                    },
                    Err(e) => {
                      fail!("Error checking padding at end of scanline: {}", e);
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
                      }
                    },
                    Err(e) => {
                      fail!("Error checking padding at end of scanline: {}", e);
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
                            }
                          },
                          Err(e) => {
                            fail!("Error checking padding at end of scanline: {}", e);
                          }
                        }
                      }
                      else {
                        break;
                      }
                    },
                    Err(e) => {
                      fail!("Error checking padding at end of scanline: {}", e);
                    }
                  }
                },
                _ => {
                  continue;
                }
              }

            }
          }
          
          // RGB8    
          if bits_per_pixel as int == 24 {
            println!("RGB8 Image");
            for y in range(0, image_height) {
              for x in range(0, image_width) {
                match image.read_exact(3) {
                  Ok(mut pixel_data) => {
                    match pixel_data.pop() {
                      Some(red) => {buffer.push(red)},
                      None  => {fail!("Error getting red component for BMP pixel at ({}, {})", x, y)}
                    }
                    match pixel_data.pop() {
                      Some(green) => {buffer.push(green)},
                      None  => {fail!("Error getting green component for BMP pixel at ({}, {})", x, y)}
                    }
                    match pixel_data.pop() {
                      Some(blue) => {buffer.push(blue)},
                      None  => {fail!("Error getting blue component for BMP pixel at ({}, {})", x, y)}
                    }
                  },
                  Err(e)    => {fail!("Error reading BMP pixel at ({}, {}): {}", x, y, e)}
                }
              }

              // Padding based on image width, all scanlines must be multiple of 4
              match image_width % 4 {
                1 => {
                  match image.read_byte() {
                    Ok(padding) => {
                      if padding as uint == 0 {
                        continue;
                      }
                      else {
                        break;
                      }
                    },
                    Err(e) => {
                      fail!("Error checking padding at end of scanline: {}", e);
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
                      }
                    },
                    Err(e) => {
                      fail!("Error checking padding at end of scanline: {}", e);
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
                            }
                          },
                          Err(e) => {
                            fail!("Error checking padding at end of scanline: {}", e);
                          }
                        }
                      }
                      else {
                        break;
                      }
                    },
                    Err(e) => {
                      fail!("Error checking padding at end of scanline: {}", e);
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


        // If bits_per_pixel = 16 or 32, then compresion must be 3 
        // BI_BITFEILDS means image is uncompressed and components values are stored according to component masks in header
        // - Should be able to identify color channels through masks. Is it common to store masks and data differently than ABGR?
        if compression_type as int == 3 { 
          // RGBA8
          if bits_per_pixel as int == 32 {
            println!("RGBA8 Image");
            for y in range(0, image_height) {
              for x in range(0, image_width) {

                match image.read_exact(4) {
                  Ok(mut pixel_data) => {       
                    match pixel_data.pop() {
                      Some(red) => {buffer.push(red)},
                      None  => {fail!("Error getting red component for BMP pixel at ({}, {})", x, y)}
                    }
                    match pixel_data.pop() {
                      Some(green) => {buffer.push(green)},
                      None  => {fail!("Error getting green component for BMP pixel at ({}, {})", x, y)}
                    }
                    match pixel_data.pop() {
                      Some(blue) => {buffer.push(blue)},
                      None  => {fail!("Error getting blue component for BMP pixel at ({}, {})", x, y)}
                    }
                    match pixel_data.pop() {
                      Some(alpha) => {buffer.push(alpha)},
                      None  => {fail!("Error getting alpha component for BMP pixel at ({}, {})", x, y)}
                    }                    
                  },
                  Err(e)    => {
                    fail!("Error reading BMP pixel at ({}, {}): {}", x, y, e)}
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
        for i in range(0, image_height){

          if bits_per_pixel == 8 {
            let start_index: uint = (image_height as uint - i as uint - 1) * image_width as uint ;  // 3 because RGB8
            let end_index: uint = start_index + image_width as uint; // Off by one as slice function doesn't include last index

            let scanline = buffer.slice(start_index, end_index);
            image_data_bytes.push_all(scanline);          
          }
          if bits_per_pixel == 24 {
            let start_index: uint = (image_height as uint - i as uint - 1) * image_width as uint * 3;  // 3 because RGB8
            let end_index: uint = start_index + (image_width as uint * 3); // Off by one as slice function doesn't include last index

            let scanline = buffer.slice(start_index, end_index);
            image_data_bytes.push_all(scanline);
          }
          if bits_per_pixel == 32 {
            let start_index: uint = (image_height as uint - i as uint - 1) * image_width as uint * 4;  // 4 because RGBA8
            let end_index: uint = start_index + (image_width as uint * 4); // Off by one as slice function doesn't include last index

            let scanline = buffer.slice(start_index, end_index);
            image_data_bytes.push_all(scanline);
          }          

        }
    }


    // GRAYSCALE8 not properly implemented yet, saved as RGB8
    if bits_per_pixel == 8 {
      Some(Image{width: image_width as uint, height: image_height as uint, color_type: GRAYSCALE8, data: image_data_bytes})
    }
    else if bits_per_pixel == 24 {
      Some(Image{width: image_width as uint, height: image_height as uint, color_type: RGB8, data: image_data_bytes})
    }
    else if bits_per_pixel == 32 {
      Some(Image{width: image_width as uint, height: image_height as uint, color_type: RGBA8, data: image_data_bytes})
    }
    else {
      println!("Error writing image as valid colorspace");
      None
    }
  }

  /* Writer Status: 
   *  - 32-bit BMPv5 write correctly
   *  - 24-bit BMPv4 write correctly
   *  - 8-bit BMPv4 doesn't work at all.
   */

  // Look into updating with write! macros
  #[allow(dead_code)]
  fn write_image(&mut self, filename: &str) -> bool {
    let path = Path::new(filename);
    let mut file = File::create(&path);
    let mut version;
    let signature = "BM";

    let padding = self.height * (self.width % 4);

    match self.color_type {
      RGBA8 => {version = 5},
      _ => {version = 4}
    }

    // Save as BMP 5.x (espcially if RGBA8)
    if version == 5 {
      match self.color_type {
        RGBA8 => {
          let filesize: u32 = (self.width * self.height * 4 + 124 + 14) as u32; 
          let reserved1: u16 = 0 as u16;
          let reserved2: u16 = 0 as u16;
          let bitmap_offset: u32 = 138 as u32;
          file.write(signature.as_bytes()).unwrap();
          file.write_le_u32(filesize).unwrap();
          file.write_le_u16(reserved1).unwrap();
          file.write_le_u16(reserved2).unwrap();
          file.write_le_u32(bitmap_offset).unwrap();

          let header_size: u32 = 124 as u32;  // Size in bytes
          let image_width: u32 = self.width as u32;    // In pixels
          let image_height: u32 = self.height as u32;   // In pixels
          let planes: u16 = 1 as u16;         // Number of color planes, in BMP this is always 1
          let bits_per_pixel: u16 = 32 as u16;  // Number of bits per pixel
          file.write_le_u32(header_size).unwrap();
          file.write_le_u32(image_width).unwrap();
          file.write_le_u32(image_height).unwrap();
          file.write_le_u16(planes).unwrap();
          file.write_le_u16(bits_per_pixel).unwrap();

          let compression_type: u32 = 3 as u32;    // 0 is uncompressed, 1 is RLE algorithm, 2 is 4-bit RLE algorithm
          let size_of_bitmap: u32 = (self.width * self.height * 4) as u32; // Size in bytes, 0 when uncompressed = 0
          let horizontal_resolution: u32 = 2835 as u32;  // In pixels per meter
          let vertical_resolution: u32 = 2835 as u32; // In pixels per meter
          let colors_used: u32 = 0 as u32;        // Number of colors in palette, 0 if no palette
          let colors_important: u32 = 0 as u32;   // 0 if all colors are important
          file.write_le_u32(compression_type).unwrap();
          file.write_le_u32(size_of_bitmap).unwrap();
          file.write_le_u32(horizontal_resolution).unwrap();
          file.write_le_u32(vertical_resolution).unwrap();
          file.write_le_u32(colors_used).unwrap();
          file.write_le_u32(colors_important).unwrap();        

          let red_mask: u32 = 0xFF000000 as u32;
          let green_mask: u32 = 0x00FF0000 as u32;
          let blue_mask: u32 = 0x0000FF00 as u32;
          let alpha_mask: u32 = 0x000000FF as u32;
          let cs_type: u32 = 0x73524742 as u32;   // write sRGB in little endian -> BGRs
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
          file.write_le_u32(red_mask).unwrap();
          file.write_le_u32(green_mask).unwrap();
          file.write_le_u32(blue_mask).unwrap();
          file.write_le_u32(alpha_mask).unwrap();
          file.write_le_u32(cs_type).unwrap();
          file.write_le_u32(endpoint_red_x).unwrap();
          file.write_le_u32(endpoint_red_y).unwrap();
          file.write_le_u32(endpoint_red_z).unwrap();
          file.write_le_u32(endpoint_green_x).unwrap();
          file.write_le_u32(endpoint_green_y).unwrap();
          file.write_le_u32(endpoint_green_z).unwrap();
          file.write_le_u32(endpoint_blue_x).unwrap();
          file.write_le_u32(endpoint_blue_y).unwrap();
          file.write_le_u32(endpoint_blue_z).unwrap();
          file.write_le_u32(gamma_red).unwrap();
          file.write_le_u32(gamma_green).unwrap();
          file.write_le_u32(gamma_blue).unwrap();

          let intent: u32 = 2 as u32; // Rendering intent values not specified
          let profile_data: u32 = 0 as u32;
          let profile_size: u32 = 0 as u32;
          let reserved: u32 = 0 as u32;
          file.write_le_u32(intent).unwrap();
          file.write_le_u32(profile_data).unwrap();
          file.write_le_u32(profile_size).unwrap();
          file.write_le_u32(reserved).unwrap();



          for y in range(0, self.height) {
            let bmp_y = self.height - 1 - y;
            for x in range(0, self.width) {

              let i = x * 4 + self.width * bmp_y * 4;
    
              // Write ABGR
              file.write_u8(self.data[i+3]).unwrap();
              file.write_u8(self.data[i+2]).unwrap();
              file.write_u8(self.data[i+1]).unwrap();
              file.write_u8(self.data[i]).unwrap();

            }
          }

          true

        },
        _ => {false}
      }
    }

    // Save as BMP 4.x
    else if version == 4 {
      match self.color_type {
        GRAYSCALE8 => {
          let filesize: u32 = ((self.width * self.height) + padding + 108 + 14) as u32; 
          let reserved1: u16 = 0 as u16;
          let reserved2: u16 = 0 as u16;
          let bitmap_offset: u32 = (122 + 1024) as u32; // Add size of color palette
          file.write(signature.as_bytes()).unwrap();
          file.write_le_u32(filesize).unwrap();
          file.write_le_u16(reserved1).unwrap();
          file.write_le_u16(reserved2).unwrap();
          file.write_le_u32(bitmap_offset).unwrap();

          let header_size: u32 = 108 as u32;  // Size in bytes
          let image_width: u32 = self.width as u32;    // In pixels
          let image_height: u32 = self.height as u32;   // In pixels
          let planes: u16 = 1 as u16;         // Number of color planes, in BMP this is always 1
          let bits_per_pixel: u16 = 8 as u16;  // Number of bits per pixel
          file.write_le_u32(header_size).unwrap();
          file.write_le_u32(image_width).unwrap();
          file.write_le_u32(image_height).unwrap();
          file.write_le_u16(planes).unwrap();
          file.write_le_u16(bits_per_pixel).unwrap();

          let compression_type: u32 = 0 as u32;    // 0 is uncompressed, 1 is RLE algorithm, 2 is 4-bit RLE algorithm
          let size_of_bitmap: u32 = (self.width * self.height + padding) as u32; // Size in bytes, 0 when uncompressed = 0
          let horizontal_resolution: u32 = 2835 as u32;  // In pixels per meter
          let vertical_resolution: u32 = 2835 as u32; // In pixels per meter
          let colors_used: u32 = 0 as u32;        // Number of colors in palette, 0 if no palette
          let colors_important: u32 = 0 as u32;   // 0 if all colors are important
          file.write_le_u32(compression_type).unwrap();
          file.write_le_u32(size_of_bitmap).unwrap();
          file.write_le_u32(horizontal_resolution).unwrap();
          file.write_le_u32(vertical_resolution).unwrap();
          file.write_le_u32(colors_used).unwrap();
          file.write_le_u32(colors_important).unwrap();        

          let red_mask: u32 = 0x00000000 as u32; //BGRs when not compressed? This is unclear
          let green_mask: u32 = 0x00000000 as u32;
          let blue_mask: u32 = 0x00000000 as u32;
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
          file.write_le_u32(red_mask).unwrap();
          file.write_le_u32(green_mask).unwrap();
          file.write_le_u32(blue_mask).unwrap();
          file.write_le_u32(alpha_mask).unwrap();
          file.write_le_u32(cs_type).unwrap();
          file.write_le_u32(endpoint_red_x).unwrap();
          file.write_le_u32(endpoint_red_y).unwrap();
          file.write_le_u32(endpoint_red_z).unwrap();
          file.write_le_u32(endpoint_green_x).unwrap();
          file.write_le_u32(endpoint_green_y).unwrap();
          file.write_le_u32(endpoint_green_z).unwrap();
          file.write_le_u32(endpoint_blue_x).unwrap();
          file.write_le_u32(endpoint_blue_y).unwrap();
          file.write_le_u32(endpoint_blue_z).unwrap();
          file.write_le_u32(gamma_red).unwrap();
          file.write_le_u32(gamma_green).unwrap();
          file.write_le_u32(gamma_blue).unwrap();

          // GRAYSCALE8 PALETTE
          for i in range(0, 256) {
            file.write_u8(i as u8).unwrap();
            file.write_u8(i as u8).unwrap();
            file.write_u8(i as u8).unwrap();
            file.write_u8(0).unwrap();
          }

          if compression_type == 0 {
            for y in range(0, self.height) {
              let bmp_y = self.height - 1 - y;
              for x in range(0, self.width) {
                let index = x + self.width * bmp_y;
                file.write_u8(self.data[index]).unwrap();
              }

              // Padding based on image width, scanlines must be multiple of 4
              match image_width % 4 {
                1 => {
                  file.write_u8(0).unwrap();
                },
                2 => {
                  file.write_u8(0).unwrap();
                  file.write_u8(0).unwrap();
                },
                3 => {
                  file.write_u8(0).unwrap();
                  file.write_u8(0).unwrap();
                  file.write_u8(0).unwrap();
                },
                _ => {
                  continue;
                }
              }
            }
          }
          true

        }


        RGB8 => {
          let filesize: u32 = ((self.width * self.height * 3) + padding + 108 + 14) as u32; 
          let reserved1: u16 = 0 as u16;
          let reserved2: u16 = 0 as u16;
          let bitmap_offset: u32 = 122 as u32; // Bitmap 3.x => 54, Bitmap 4.x => 122
          file.write(signature.as_bytes()).unwrap();
          file.write_le_u32(filesize).unwrap();
          file.write_le_u16(reserved1).unwrap();
          file.write_le_u16(reserved2).unwrap();
          file.write_le_u32(bitmap_offset).unwrap();

          let header_size: u32 = 108 as u32;  // Size in bytes
          let image_width: u32 = self.width as u32;    // In pixels
          let image_height: u32 = self.height as u32;   // In pixels
          let planes: u16 = 1 as u16;         // Number of color planes, in BMP this is always 1
          let bits_per_pixel: u16 = 24 as u16;  // Number of bits per pixel
          file.write_le_u32(header_size).unwrap();
          file.write_le_u32(image_width).unwrap();
          file.write_le_u32(image_height).unwrap();
          file.write_le_u16(planes).unwrap();
          file.write_le_u16(bits_per_pixel).unwrap();

          let compression_type: u32 = 0 as u32;    // 0 is uncompressed, 1 is RLE algorithm, 2 is 4-bit RLE algorithm
          let size_of_bitmap: u32 = (self.width * self.height * 3 + padding) as u32; // Size in bytes, 0 when uncompressed = 0
          let horizontal_resolution: u32 = 2835 as u32;  // In pixels per meter
          let vertical_resolution: u32 = 2835 as u32; // In pixels per meter
          let colors_used: u32 = 0 as u32;        // Number of colors in palette, 0 if no palette
          let colors_important: u32 = 0 as u32;   // 0 if all colors are important
          file.write_le_u32(compression_type).unwrap();
          file.write_le_u32(size_of_bitmap).unwrap();
          file.write_le_u32(horizontal_resolution).unwrap();
          file.write_le_u32(vertical_resolution).unwrap();
          file.write_le_u32(colors_used).unwrap();
          file.write_le_u32(colors_important).unwrap();        

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
          file.write_le_u32(red_mask).unwrap();
          file.write_le_u32(green_mask).unwrap();
          file.write_le_u32(blue_mask).unwrap();
          file.write_le_u32(alpha_mask).unwrap();
          file.write_le_u32(cs_type).unwrap();
          file.write_le_u32(endpoint_red_x).unwrap();
          file.write_le_u32(endpoint_red_y).unwrap();
          file.write_le_u32(endpoint_red_z).unwrap();
          file.write_le_u32(endpoint_green_x).unwrap();
          file.write_le_u32(endpoint_green_y).unwrap();
          file.write_le_u32(endpoint_green_z).unwrap();
          file.write_le_u32(endpoint_blue_x).unwrap();
          file.write_le_u32(endpoint_blue_y).unwrap();
          file.write_le_u32(endpoint_blue_z).unwrap();
          file.write_le_u32(gamma_red).unwrap();
          file.write_le_u32(gamma_green).unwrap();
          file.write_le_u32(gamma_blue).unwrap();

          if compression_type == 0 {
            for y in range(0, self.height) {
              let bmp_y = self.height - 1 - y;
              for x in range(0, self.width) {
                
                let mut pixel_data: Vec<u8> = self.get_pixel(x,bmp_y);
                let blue  = pixel_data.pop().unwrap();
                let green = pixel_data.pop().unwrap();
                let red   = pixel_data.pop().unwrap();

                file.write_u8(blue).unwrap();
                file.write_u8(green).unwrap();
                file.write_u8(red).unwrap();

              }

              // Padding based on image width, scanlines must be multiple of 4
              match image_width % 4 {
                1 => {
                  file.write_u8(0).unwrap();
                },
                2 => {
                  file.write_u8(0).unwrap();
                  file.write_u8(0).unwrap();
                },
                3 => {
                  file.write_u8(0).unwrap();
                  file.write_u8(0).unwrap();
                  file.write_u8(0).unwrap();
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
      file.write(signature.as_bytes()).unwrap();
      file.write_le_u32(filesize).unwrap();
      file.write_le_u16(reserved1).unwrap();
      file.write_le_u16(reserved2).unwrap();
      file.write_le_u32(bitmap_offset).unwrap();

      // Bitmap 3.x Header, 40 bytes in size (no colorspace info)
      let header_size: u32 = 40 as u32;  // Size in bytes
      let image_width: u32 = self.width as u32;    // In pixels
      let image_height: u32 = self.height as u32;   // In pixels
      let planes: u16 = 1 as u16;         // Always 1
      let bits_per_pixel: u16 = 24 as u16;  // Number of bits per pixel
      file.write_le_u32(header_size).unwrap();
      file.write_le_u32(image_width).unwrap();
      file.write_le_u32(image_height).unwrap();
      file.write_le_u16(planes).unwrap();
      file.write_le_u16(bits_per_pixel).unwrap();

      let compression_type: u32 = 0 as u32;    // 0 is uncompressed, 1 is RLE algorithm, 2 is 4-bit RLE algorithm
      let size_of_bitmap: u32 = 0 as u32; // Size in bytes, 0 when uncompressed
      let horizontal_resolution: u32 = 2835 as u32;  // In pixels per meter
      let vertical_resolution: u32 = 2835 as u32; // In pixels per meter
      let colors_used: u32 = 0 as u32;        // Number of colors in palette, 0 if no palette
      let colors_important: u32 = 0 as u32;   // 0 if all colors are important
      file.write_le_u32(compression_type).unwrap();
      file.write_le_u32(size_of_bitmap).unwrap();
      file.write_le_u32(horizontal_resolution).unwrap();
      file.write_le_u32(vertical_resolution).unwrap();
      file.write_le_u32(colors_used).unwrap();
      file.write_le_u32(colors_important).unwrap();

      // Color Palette (only if bits_per_pixel == 1, 4, or 8)

      // Read every scanline from left to right, bottom up; but read each pixel value as BGR
      for y in range(0, self.height) {
        let bmp_y = self.height - 1 - y;
        for x in range(0, self.width) {

          let mut pixel_data: Vec<u8> = self.get_pixel(x,bmp_y);
          let blue  = pixel_data.pop().unwrap();
          let green = pixel_data.pop().unwrap();
          let red   = pixel_data.pop().unwrap();

          file.write_u8(blue).unwrap();
          file.write_u8(green).unwrap();
          file.write_u8(red).unwrap();

        }
        // Padding based on image width, scanlines must be multiple of 4
        match image_width % 4 {
          1 => {
            file.write_u8(0).unwrap();
          },
          2 => {
            file.write_u8(0).unwrap();
          },
          3 => {
            file.write_u8(0).unwrap();
            file.write_u8(0).unwrap();
            file.write_u8(0).unwrap();
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


// Image processing traits and functions (Not implemented for all color types)
trait PointProcessor {
  fn negative(&mut self);
  fn brighten(&mut self, bias: int);
  fn contrast(&mut self, gain: f32);
  fn saturate(&mut self, gain: f32);
}

impl PointProcessor for Image {

  // Update for all color types
  #[allow(dead_code)]
  fn negative(&mut self) {
    // Brute force        Time: 19257397 ns
    // Vectorize by 8     Time:  5118442 ns
    //let start = time::precise_time_ns();


    let mut i = 0;
    let length = self.data.len();
    let remainder = length % 8;
    let difference = length - remainder;

    match self.color_type {
      RGBA8 => {
        while i < length {
          self.data[i] = 255 - self.data[i];
          self.data[i+1] = 255 - self.data[i+1];
          self.data[i+2] = 255 - self.data[i+2];

          i += 4;
        }
      },

      _ => {
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
      }
    }


    // let end = time::precise_time_ns();
    // let time = end as uint - start as uint;
    // println!("Time of vectorized algorithm: {}", time);
  }

  // Update for all color types
  #[allow(dead_code)]
  fn brighten(&mut self, bias: int) {
    // Brute force        Time: 33111543 ns
    // let start = time::precise_time_ns();

    for y in range(0, self.height){
      for x in range(0, self.width){

        match self.color_type {
          GRAYSCALE8 => {

            let offset:  uint = x + self.width * y;
            let pixel_data: u8 = self.data[offset];

            let mut lum = pixel_data as int + bias;

            if lum > 255 {lum = 255;}
            if lum < 0 {lum = 0;}

            self.data[offset] = lum as u8;

          },
          RGB8 => {

            let mut pixel_data: Vec<u8> = self.get_pixel(x,y);
            let b  = pixel_data.pop().unwrap();
            let g  = pixel_data.pop().unwrap();
            let r  = pixel_data.pop().unwrap();

            let mut red   = r as int + bias;
            let mut green = g as int + bias;
            let mut blue  = b as int + bias;

            if red > 255 {red = 255;}
            if green > 255 {green = 255;}
            if blue > 255 {blue = 255;}

            if red < 0 {red = 0;}
            if green < 0 {green = 0;}
            if blue < 0 {blue = 0;}
            
            let pixel_data: Vec<u8> = vec!(red as u8, green as u8, blue as u8);
            self.set_pixel(x,y, pixel_data);

          },
          RGBA8 => {

            let mut pixel_data: Vec<u8> = self.get_pixel(x,y);
            let alpha   = pixel_data.pop().unwrap();
            let b       = pixel_data.pop().unwrap();
            let g       = pixel_data.pop().unwrap();
            let r       = pixel_data.pop().unwrap();

            let mut red   = r as int + bias;
            let mut green = g as int + bias;
            let mut blue  = b as int + bias;

            if red > 255 {red = 255;}
            if green > 255 {green = 255;}
            if blue > 255 {blue = 255;}

            if red < 0 {red = 0;}
            if green < 0 {green = 0;}
            if blue < 0 {blue = 0;}
            
            let pixel_data: Vec<u8> = vec!(red as u8, green as u8, blue as u8, alpha as u8);
            self.set_pixel(x,y, pixel_data);

          }
        }


      }
    }

    // let end = time::precise_time_ns();
    // let time = end as uint - start as uint;
    // println!("Time of algorithm: {}", time);
  }

  #[allow(dead_code)]
  fn contrast(&mut self, gain: f32) {
    let mut total_luminance: f32 = 0.;

    for y in range(0, self.height){
      for x in range(0, self.width){

        let mut pixel_data: Vec<u8> = self.get_pixel(x,y);
        let b  = pixel_data.pop().unwrap();
        let g  = pixel_data.pop().unwrap();
        let r  = pixel_data.pop().unwrap();

        let red   = r as f32;
        let green = g as f32;
        let blue  = b as f32;

        let luminance: f32 = 0.2126 * red  + 0.7152 * green  + 0.0722 * blue;
        total_luminance += luminance;
      }
    }

    let mean_luminance: f32 = total_luminance/((self.width*self.height) as f32);

    for y in range(0, self.height){
      for x in range(0, self.width){

        let mut pixel_data: Vec<u8> = self.get_pixel(x,y);
        let b  = pixel_data.pop().unwrap();
        let g  = pixel_data.pop().unwrap();
        let r  = pixel_data.pop().unwrap();

        let mut red     = r as int;
        let mut green   = g as int;
        let mut blue    = b as int;

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
        
        let pixel_data: Vec<u8> = vec!(red as u8, green as u8, blue as u8);
        self.set_pixel(x,y, pixel_data);

      }
    }
  }

  #[allow(dead_code)]
  fn saturate(&mut self, gain: f32) {
    for y in range(0, self.height){
      for x in range(0, self.width){

        let mut pixel_data: Vec<u8> = self.get_pixel(x,y);
        let b  = pixel_data.pop().unwrap();
        let g  = pixel_data.pop().unwrap();
        let r  = pixel_data.pop().unwrap();

        let mut red     = r as int;
        let mut green   = g as int;
        let mut blue    = b as int;

        let luminance: f32 = 0.2126 * red as f32 + 0.7152 * green as f32 + 0.0722 * blue as f32;
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
        
        let pixel_data: Vec<u8> = vec!(red as u8, green as u8, blue as u8);
        self.set_pixel(x,y, pixel_data);

      }
    }
  }
}

trait ConvolutionFilter {
  fn blur(&mut self);
}

impl ConvolutionFilter for Image {
  #[allow(dead_code)]
  fn blur(&mut self) {
    // Brute force        Time: 264835676 ns
    // let start = time::precise_time_ns();
  
    let kernel = [[1, 1, 1], [1, 1, 1], [1, 1, 1]];
    let kernel_sum = 9;
    let kernel_center_x: uint = 3/2;
    let kernel_center_y: uint = 3/2;

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

              let mut pixel_data: Vec<u8> = self.get_pixel(kx as uint , ky as uint);
              let b  = pixel_data.pop().unwrap();
              let g  = pixel_data.pop().unwrap();
              let r  = pixel_data.pop().unwrap();

              red_sum   += r as int * kernel_value;
              green_sum += g as int * kernel_value;
              blue_sum  += b as int * kernel_value;              

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

        let pixel_data: Vec<u8> = vec!(red_sum as u8, green_sum as u8, blue_sum as u8);
        self.set_pixel(x as uint,y as uint, pixel_data);
        
      }
    }

    // let end = time::precise_time_ns();
    // let time = end as uint - start as uint;
    // println!("Time of brute force algorithm: {}", time);
  }
}


#[allow(dead_code)]
fn main() {
  let args = os::args();
  if args.len() < 2 {
    fail!("Image path not provided");
  }
  else {
    
    let image: Option<Image> = BMP::read_image(args[1]);

    match image {
      Some(mut image) => {
        image.brighten(10);
        image.write_image("image.bmp");
      },
      None  => {
        println!("Looks like you didn't get a valid image.");
      }
    }
    
    
    /*for y in range(0, image.height) {
      for x in range(0, image.width) {
        match image.color_type {
          GRAYSCALE8 => {
            let i = x + image.width * y;
            print!("({}) ", image.data[i]);
          }
          RGB8 => {
            let pixel = image.get_pixel(x,y);
            print!("{} ", pixel);  
          },
          RGBA8 => {
            let pixel = image.get_pixel(x,y);
            print!("{} ", pixel);  
          }
        }
      }
      print!("\n");
    }*/
    

    

  }

}


// Use actual images to test functions
// Testing image module
#[test]
fn test_image_new() {
  let image = Image::new(4, 4, GRAYSCALE8);
  assert_eq!(image.get_width(), 4);
  assert_eq!(image.get_height(), 4);

  let image = Image::new(20, 20, RGB8);
  assert_eq!(image.get_width(), 20);
  assert_eq!(image.get_height(), 20);

  let image = Image::new(1000, 1000, RGB8);
  assert_eq!(image.get_width(), 1000);
  assert_eq!(image.get_height(), 1000);
}

#[test]
fn test_get_empty_pixel() {
  let image = Image::new(20, 20, GRAYSCALE8);
  let grayscale_pixel = image.get_pixel(0, 0);
  assert_eq!(grayscale_pixel, vec!(0));

  let image = Image::new(20, 20, RGB8);
  let rgb_pixel = image.get_pixel(0, 0);
  assert_eq!(rgb_pixel, vec!(0,0,0));

  let image = Image::new(20, 20, RGBA8);
  let rgba_pixel = image.get_pixel(0, 0);
  assert_eq!(rgba_pixel, vec!(0,0,0,0));
}

#[test]
fn test_set_empty_pixel() {
  let mut image = Image::new(20, 20, GRAYSCALE8);
  image.set_pixel(0, 0, vec!(255));
  let grayscale_pixel = image.get_pixel(0, 0);
  assert_eq!(grayscale_pixel, vec!(255));

  let mut image = Image::new(20, 20, RGB8);
  image.set_pixel(0, 0, vec!(127, 54, 0));
  let rgb_pixel = image.get_pixel(0, 0);
  assert_eq!(rgb_pixel, vec!(127, 54, 0));

  let mut image = Image::new(20, 20, RGBA8);
  image.set_pixel(0, 0, vec!(23, 68, 144, 174));
  let rgba_pixel = image.get_pixel(0, 0);
  assert_eq!(rgba_pixel, vec!(23, 68, 144, 174));

  // Test: randomly select a pixel from image, set it then get it to confirm
}
