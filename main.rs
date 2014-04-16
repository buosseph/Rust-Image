use std::slice::from_elem;
use std::path::posix::{Path};
use std::io::File;
use std::os;
use std::str;

// From http://rosettacode.org/wiki/Bitmap/Write_a_PPM_file#Rust
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


  pub fn read_image(&self, path: Path) {
    // let path = Path::new(image_path_str);
    // let mut image = File::open(&path).unwrap();

    // let mut w: uint = 0;
    // let mut h: uint = 0;
    // let size = 3 * h * w;
    // let mut data: ~[u8] = from_elem(size, 0u8);

    // match image.read_byte() {
    //   Ok(byte)   => {
        
    //     if byte == 0x50 {
    //       println!("P: {:x}", byte);
    //     }
    //   },
    //   Err(EOF)  => {
    //     println!("End of file"); 
    //     break;}
    // }

    // // Use to fill buffer
    // loop {
    //   match image.read_byte() {
    //     Ok(byte)   => {
          
    //       if byte == 0x50 {
    //         println!("P: {:x}", byte);
    //       }
    //     },
    //     Err(EOF)  => {
    //       println!("End of file"); 
    //       break;}
    //   }
    // }

    // Gets Px header
    // println!("{:x}", image.read_byte().unwrap());
    // println!("{:x}", image.read_byte().unwrap());

    // If 0x20 || 0x0A, ignore byte (is space or newline)
    // If 0x23, ignore all bytes until newline (0x0A)
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
    let header = format!("P6 {} {} 255\n", self.height, self.width);
    file.write(header.as_bytes());
    file.write(self.data);
    true
  }

  // pub fn new_test_image(&self) -> PPM {
  //   let TEST_WIDTH = 514;
  //   let TEST_HEIGHT = 600;
  //   let size = 3 * TEST_WIDTH * TEST_HEIGHT;
  //   let data_str = "";
  //   PPM{height: height, width: width, data: buffer}
  // }
}

pub struct PNG {
  width: uint,
  height: uint,
  data: ~[u8]
}
impl PNG {
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


impl Inversible for PPM {
  fn inverse(&mut self) {
    // Brute Force
    for i in range(0, self.data.len()) {
      self.data[i] = 255 - self.data[i];
    }

  }
}


fn main() {
  let TEST_IMAGE = PPM::new(524, 600);

  let image = PNG::new(0,0);

  // let mut image = PPM::new(360, 240);

  // for x in range(0, 360){
  //   for y in range(0,240){
  //     image.set_pixel(x as uint,y as uint, Pixel{r: 255, g: 255, b: 255});
  //   }
  // }
  // image.inverse();
  // image.write_file("test.ppm");




  // For later use, reading .ppm files for processing
  // let argv = os::args();
  // let image_path_str = argv[1];

}