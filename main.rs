use std::slice::from_elem;
use std::slice;
use std::path::posix::{Path};
use std::io::File;
use std::os;
use std::str;
use std::uint;

pub struct Pixel {
  r: u8,
  g: u8,
  b: u8,
}
pub struct RGBA_Pixel {
  r: u8,
  g: u8,
  b: u8,
  a: u8,
}

pub struct Image {
  width: uint,
  height: uint,
  pixels: ~[RGBA_Pixel]
}
impl Image {
  pub fn new(width: uint, height: uint) -> Image {
    let pixel_array: ~[RGBA_Pixel] = ~[];
    Image {width: width, height: height, pixels: pixel_array}
  }
}

trait Inversible {
  fn inverse(&mut self);
}
 
pub struct PPM {
  height: uint,
  width: uint,
  data: ~[u8]
}
impl PPM {
  pub fn new(height: uint, width: uint) -> PPM {
    let size = 3 * height * width;
    let mut buffer = from_elem(size, 0u8);
    PPM{height: height, width: width, data: buffer}
  }

  pub fn read_image(image_path_str: &str) -> PPM {
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
              Some(mode)  => {println!("{}", mode)},    // Check if valid header
              None        => {fail!("Something went wrong converting bytes to str (line 65)")}
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
    PPM{height: height, width: width, data: image_data_bytes}
  }

 
  fn buffer_size(&self) -> uint {
    3 * self.height * self.width
  }
 
  fn get_offset(&self, x: uint, y: uint) -> Option<uint> {
    let offset = (y * self.height * 3) + (x * 3);
    if offset < self.buffer_size() {
      Some(offset)
    }else{
      None
    }
  }
 
  pub fn get_pixel(&self, x: uint, y: uint) -> Option<Pixel> {
    match self.get_offset(x, y) {
      Some(offset) => {
        let r1 = self.data[offset];
        let g1 = self.data[offset + 1];
        let b1 = self.data[offset + 2];
        Some(Pixel{r: r1, g: g1, b: b1})
      },
      None => None
    }
  }
 
  pub fn set_pixel(&mut self, x: uint, y: uint, color: Pixel) -> bool {
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
 
  pub fn write_file(&self, filename: &str) -> bool {
    let path = Path::new(filename);
    let mut file = File::create(&path);
    let header = format!("P6 {} {} 255\n", self.width, self.height);
    file.write(header.as_bytes());
    file.write(self.data);
    true
  }
}

impl Inversible for PPM {
  fn inverse(&mut self) {
    // Brute Force
    for i in range(0, self.data.len()) {
      self.data[i] = 255 - self.data[i];
    }

  }
}


fn main() {
  let args = os::args();
  if args.len() < 2 {
    fail!("Image path not provided");
  }
  else {
    println!("Path to image: {}", args[1]);
    let mut ppm_image = PPM::read_image(args[1]);
    ppm_image.inverse();
    ppm_image.write_file("output.ppm");
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