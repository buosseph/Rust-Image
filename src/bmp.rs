// BMP Image format

use std::path::posix::{Path};
use std::io::File;
use std::str;
use image::*;

static SIGNATURE: &'static str = "BM";


/* NOTES:
 * BMP pixels stored as BGR, not RGB8
 * If height is positive, scanlines stored BOTTOM UP --> store pixels starting from bottom row when writing
 * If height is negative, scanliens stored TOP DOWN  --> No flip required to match Image struct pixel array orientation
 * Image_width % 4 = # of bytes for padding per scanline
 * Only v4 an v5 can produce RGBA8 images
 */

#[allow(dead_code)]
#[allow(dead_assignment)]
pub fn read_bitmap(image_path_str: &str) -> Option<Image>{

  let path = Path::new(image_path_str);

  let mut file_size: u32 = 0 as u32;      
  let mut offset: u32 = 0 as u32;
  let mut header_size: u32 = 0 as u32;      // 40 = BMPv3, 108 = BMPv4, 124 = BMPv5
  let mut image_width: u32 = 0 as u32;
  let mut image_height: u32 = 0 as u32;
  let mut planes: u16 = 0 as u16;
  let mut bits_per_pixel: u16 = 0 as u16;   // 8 = GRAYSCALE8, 24 = RGB8, 32 = RGBA8
  let mut compression_type: u32 = 0 as u32;
  let mut size_of_bitmap: u32 = 0 as u32;   

  let mut image_data_bytes: Vec<u8> = Vec::new();
  let mut buffer: Vec<u8> = Vec::new();


  match File::open(&path) {
    Ok(mut image) => {

      // Check signature

      // There's a better way of doing this?
      let read_sig = image.read_exact(2u).unwrap();
      let signature = read_sig.as_slice();
      if !str::eq_slice(str::from_utf8(signature).unwrap(), SIGNATURE) {
        fail!("Input image is not a valid BMP image");
      }

      file_size = image.read_le_u32().unwrap();
      image.read_le_u32().unwrap();   // Reserved
      offset = image.read_le_u32().unwrap();
      header_size = image.read_le_u32().unwrap();
      image_width = image.read_le_u32().unwrap();
      image_height = image.read_le_u32().unwrap();
      planes = image.read_le_u16().unwrap();
      bits_per_pixel = image.read_le_u16().unwrap();
      compression_type = image.read_le_u32().unwrap();
      size_of_bitmap = image.read_le_u32().unwrap();


      let remainder = offset as int - 14 - 24;    // offset - fileheader size - read bytes

  println!("
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
  );


      for _ in range(0, remainder) {
        match image.read_byte() {
          Ok(_)  => {continue},
          Err(e)    => {fail!("Error reading BMP header: {}", e)}
        }
      }

      // BI_RGB means uncompressed (BGR)
      if compression_type as uint == 0 {

        // GRAYSCALE8
        if bits_per_pixel as uint == 8 {
          println!("GRAYSCALE8 Image");
          for y in range(0, image_height) {
            for x in range(0, image_width) {
              match image.read_byte() {
                Ok(pixel_data) => {
                  buffer.push(pixel_data);
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

#[allow(dead_code)]
pub fn write_bitmap(image: Image, filename: &str) -> bool {

  let path = Path::new(filename);
  let mut file = File::create(&path);
  let padding = image.height * (image.width % 4);


  match image.color_type {

    // Save as BMP 4.x
    GRAYSCALE8 => {

      let filesize: u32 = ((image.width * image.height) + padding + 108 + 14) as u32; 
      let reserved: u32 = 0 as u32;
      let bitmap_offset: u32 = (122u + 1024u) as u32; // Add size of color palette

      file.write(SIGNATURE.as_bytes()).unwrap();
      file.write_le_u32(filesize).unwrap();
      file.write_le_u32(reserved).unwrap();
      file.write_le_u32(bitmap_offset).unwrap();


      let header_size: u32 = 108 as u32;  // Size in bytes
      let image_width: u32 = image.width as u32;    // In pixels
      let image_height: u32 = image.height as u32;   // In pixels
      let planes: u16 = 1 as u16;         // Number of color planes, in BMP this is always 1
      let bits_per_pixel: u16 = 8 as u16;  // Number of bits per pixel

      file.write_le_u32(header_size).unwrap();
      file.write_le_u32(image_width).unwrap();
      file.write_le_u32(image_height).unwrap();
      file.write_le_u16(planes).unwrap();
      file.write_le_u16(bits_per_pixel).unwrap();

      let compression_type: u32 = 0 as u32;    // 0 is uncompressed, 1 is RLE algorithm, 2 is 4-bit RLE algorithm
      let size_of_bitmap: u32 = (image.width * image.height + padding) as u32; // Size in bytes, 0 when uncompressed = 0
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
      for i in range(0u, 256) {
        file.write_u8(i as u8).unwrap();
        file.write_u8(i as u8).unwrap();
        file.write_u8(i as u8).unwrap();
        file.write_u8(0).unwrap();
      }


      if compression_type == 0 {
        for y in range(0, image.height) {

          let bmp_y = image.height - 1 - y;

          for x in range(0, image.width) {

            let index = x + image.width * bmp_y;
            file.write_u8(*image.data.get(index)).unwrap();

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


    // Save as BMP 4.x
    RGB8 => {

      let filesize: u32 = ((image.width * image.height * 3) + padding + 108 + 14) as u32; 
      let reserved: u32 = 0 as u32;
      let bitmap_offset: u32 = 122 as u32; // Bitmap 3.x => 54, Bitmap 4.x => 122

      file.write(SIGNATURE.as_bytes()).unwrap();
      file.write_le_u32(filesize).unwrap();
      file.write_le_u32(reserved).unwrap();
      file.write_le_u32(bitmap_offset).unwrap();


      let header_size: u32 = 108 as u32;  // Size in bytes
      let image_width: u32 = image.width as u32;    // In pixels
      let image_height: u32 = image.height as u32;   // In pixels
      let planes: u16 = 1 as u16;         // Number of color planes, in BMP this is always 1
      let bits_per_pixel: u16 = 24 as u16;  // Number of bits per pixel

      file.write_le_u32(header_size).unwrap();
      file.write_le_u32(image_width).unwrap();
      file.write_le_u32(image_height).unwrap();
      file.write_le_u16(planes).unwrap();
      file.write_le_u16(bits_per_pixel).unwrap();


      let compression_type: u32 = 0 as u32;    // 0 is uncompressed, 1 is RLE algorithm, 2 is 4-bit RLE algorithm
      let size_of_bitmap: u32 = (image.width * image.height * 3 + padding) as u32; // Size in bytes, 0 when uncompressed = 0
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
        for y in range(0, image.height) {

          let bmp_y = image.height - 1 - y;

          for x in range(0, image.width) {
            
            let mut pixel_data: Vec<u8> = image.get_pixel(x,bmp_y);
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

    // Save as BMP 5.x
    RGBA8 => {

      let filesize: u32 = (image.width * image.height * 4 + 124 + 14) as u32; 
      let reserved: u32 = 0 as u32;
      let bitmap_offset: u32 = 138 as u32;

      file.write(SIGNATURE.as_bytes()).unwrap();
      file.write_le_u32(filesize).unwrap();
      file.write_le_u32(reserved).unwrap();
      file.write_le_u32(bitmap_offset).unwrap();


      let header_size: u32 = 124 as u32;  // Size in bytes
      let image_width: u32 = image.width as u32;    // In pixels
      let image_height: u32 = image.height as u32;   // In pixels
      let planes: u16 = 1 as u16;         // Number of color planes, in BMP this is always 1
      let bits_per_pixel: u16 = 32 as u16;  // Number of bits per pixel

      file.write_le_u32(header_size).unwrap();
      file.write_le_u32(image_width).unwrap();
      file.write_le_u32(image_height).unwrap();
      file.write_le_u16(planes).unwrap();
      file.write_le_u16(bits_per_pixel).unwrap();


      let compression_type: u32 = 3 as u32;    // 0 is uncompressed, 1 is RLE algorithm, 2 is 4-bit RLE algorithm
      let size_of_bitmap: u32 = (image.width * image.height * 4) as u32; // Size in bytes, 0 when uncompressed = 0
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


      for y in range(0, image.height) {

        let bmp_y = image.height - 1 - y;

        for x in range(0, image.width) {

          let i = x * 4 + image.width * bmp_y * 4;

          // Write ABGR
          file.write_u8(*image.data.get(i+3)).unwrap();
          file.write_u8(*image.data.get(i+2)).unwrap();
          file.write_u8(*image.data.get(i+1)).unwrap();
          file.write_u8(*image.data.get(i)).unwrap();

        }
      }

      true

    },

  }

}


#[cfg(test)]
mod tests {
  use super::*;

  // Reading: verify meta data

  // Writing: write all test images to folder with no changes, verify image are correct visually
  #[test]
  fn test_writing() {

    let test_images = vec!(
      "rgba_v5.bmp",
      "rgb_v4.bmp",
      "rgba_info.bmp",
      "testimage_rgba_v5.bmp",
      "testimage_rgb_v4.bmp",
      "testimage_rgb_v3.bmp",
      "grayscale_v4.bmp",
      "grayscale_v3.bmp",
      "padding3_rgb_v4.bmp",
      "padding2_rgb_v4.bmp",
      "padding1_rgb_v4.bmp",
    );

    for filename in test_images.iter() {

      let path_prefix: String = "../bmp/".to_string();
      let path_to_file: String = path_prefix.append(*filename);
      let image = read_bitmap(path_to_file.as_slice());

      
      match image {
        Some(image) => {
          let path_prefix: String = "../test/".to_string();
          let path_to_write: String = path_prefix.append(*filename);
          assert!(write_bitmap(image, path_to_write.as_slice()));
        },
        None  => {
          println!("Looks like you didn't get a valid image.");
        }
      }

    }

  }
}