
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
  pub width: uint,
  pub height: uint,
  pub color_type: ColorType,
  pub data: Vec<u8>,
}

impl Image {

  #[allow(dead_code)]
  pub fn new(width: uint, height: uint, color_type: ColorType) -> Image {
    match color_type {

      GRAYSCALE8   => {
        let size: uint = width * height;
        let buffer: Vec<u8> = Vec::from_elem(size, 0u8);
        Image{width: width, height: height, color_type: GRAYSCALE8, data: buffer}
      },

      RGB8         => {
        let size: uint = 3 * width * height;
        let buffer: Vec<u8> = Vec::from_elem(size, 0u8);
        Image{width: width, height: height, color_type: RGB8, data: buffer}
      },

      RGBA8        => {
        let size: uint = 4 * width * height;
        let buffer: Vec<u8> = Vec::from_elem(size, 0u8);
        Image{width: width, height: height, color_type: RGBA8, data: buffer}
      }

    }
  }

  fn buffer_size(&self) -> uint {
    match self.color_type {
      GRAYSCALE8   => { self.width * self.height     },
      RGB8         => { self.width * self.height * 3 },
      RGBA8        => { self.width * self.height * 4 }
    }
  }
 
  fn get_offset(&self, x: uint, y: uint) -> Option<uint> {
    match self.color_type {

      GRAYSCALE8 => {
        let offset: uint = x + self.width * y;
        if offset < self.buffer_size() {
          Some(offset)
        }else{
          None
        }        
      },

      RGB8 => {
        let offset: uint = (x + self.width * y) * 3;
        if offset < self.buffer_size() {
          Some(offset)
        }else{
          None
        }
      },

      RGBA8 => {
        let offset: uint = (x + self.width * y) * 4;
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
            let pixel_data: Vec<u8> = vec!(self.data.get(offset).clone());
            pixel_data
          },
          None => {fail!("Couldn't get GRAYSCALE8 pixel")}
        }        
      },

      RGB8 => {
        match self.get_offset(x, y) {
          Some(offset) => {
            let pixel_data: Vec<u8> = vec!(
              self.data.get(offset).clone(),
              self.data.get(offset + 1).clone(),
              self.data.get(offset + 2).clone()
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
              self.data.get(offset).clone(),
              self.data.get(offset + 1).clone(),
              self.data.get(offset + 2).clone(),
              self.data.get(offset + 3).clone()
              );
            pixel_data
          },
          None => {fail!("Couldn't get RGBA8 pixel")}
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
            *self.data.get_mut(offset) = color.pop().unwrap();
            true
          },
          None => false
        }           
      },

      RGB8 => {
        match self.get_offset(x, y) {
          Some(offset) => {
            *self.data.get_mut(offset + 2)  = color.pop().unwrap();
            *self.data.get_mut(offset + 1)  = color.pop().unwrap();
            *self.data.get_mut(offset)      = color.pop().unwrap();
            true
          },
          None => false
        }
      },

      RGBA8 => {
        match self.get_offset(x, y) {
          Some(offset) => {
            *self.data.get_mut(offset + 3)  = color.pop().unwrap();
            *self.data.get_mut(offset + 2)  = color.pop().unwrap();
            *self.data.get_mut(offset + 1)  = color.pop().unwrap();
            *self.data.get_mut(offset)      = color.pop().unwrap();
            true
          },
          None => false
        }
      }

    }
  }

  #[allow(dead_code)]
  pub fn convert_to_grayscale8(&mut self) -> bool {
    match self.color_type {

      GRAYSCALE8 => {
        println!("Image already GRAYSCALE8");
        return true
      },

      RGB8      => {
        let mut new_pixel_array: Vec<u8> = Vec::new();
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
        let mut new_pixel_array: Vec<u8> = Vec::new();
        for y in range(0, self.height){
          for x in range(0, self.width){
            let mut pixel_data: Vec<u8> = self.get_pixel(x,y);
            pixel_data.pop().unwrap();  // Unneeded alpha component
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
  pub fn convert_to_rgb8(&mut self) -> bool {
    match self.color_type {

      GRAYSCALE8 => {
        let mut new_pixel_array: Vec<u8> = Vec::new();
        for i in range(0, self.data.len()) {
          let lum = self.data.get(i).clone();
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
        let mut new_pixel_array: Vec<u8> = Vec::new();
        for i in range(0, self.data.len()) {
          if i % 4 ==3 {
            continue;
          }
          let component = self.data.get(i).clone();
          new_pixel_array.push(component);
        }
        self.data = new_pixel_array;
        self.color_type = RGB8;
        true
      }

    }
  }

  #[allow(dead_code)]
  pub fn convert_to_rgba8(&mut self) -> bool {
    match self.color_type {

      GRAYSCALE8 => {
        let mut new_pixel_array: Vec<u8> = Vec::new();
        for i in range(0, self.data.len()) {
          let lum = self.data.get(i).clone();
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
        let mut new_pixel_array: Vec<u8> = Vec::new();
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

}



// Use actual images to test functions
#[cfg(test)] 
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let image = Image::new(4, 4, GRAYSCALE8);
    assert_eq!(image.width, 4);
    assert_eq!(image.height, 4);

    let image = Image::new(20, 20, RGB8);
    assert_eq!(image.width, 20);
    assert_eq!(image.height, 20);

    let image = Image::new(1000, 1000, RGB8);
    assert_eq!(image.width, 1000);
    assert_eq!(image.height, 1000);
  }

  #[test]
  fn test_get_pixel() {
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
  fn test_set_pixel() {
    let mut image = Image::new(20, 20, GRAYSCALE8);
    image.set_pixel(0, 0, vec!(255));
    let grayscale_pixel = image.get_pixel(0, 0);
    assert_eq!(grayscale_pixel, vec!(255));

    image.set_pixel(0, 0, vec!(127));
    let grayscale_pixel = image.get_pixel(0, 0);
    assert_eq!(grayscale_pixel, vec!(127));



    let mut image = Image::new(20, 20, RGB8);
    image.set_pixel(0, 0, vec!(127, 54, 0));
    let rgb_pixel = image.get_pixel(0, 0);
    assert_eq!(rgb_pixel, vec!(127, 54, 0));

    image.set_pixel(0, 0, vec!(255, 21, 98));
    let rgb_pixel = image.get_pixel(0, 0);
    assert_eq!(rgb_pixel, vec!(255, 21, 98));



    let mut image = Image::new(20, 20, RGBA8);
    image.set_pixel(0, 0, vec!(23, 68, 144, 174));
    let rgba_pixel = image.get_pixel(0, 0);
    assert_eq!(rgba_pixel, vec!(23, 68, 144, 174));

    image.set_pixel(0, 0, vec!(64, 82, 1, 248));
    let rgba_pixel = image.get_pixel(0, 0);
    assert_eq!(rgba_pixel, vec!(64, 82, 1, 248));  

    // Test: randomly select a pixel from image, set it then get it to confirm
  }

  #[test]
  fn test_convert_to_grayscale8() {
    // RGB8 to GRAYSCALE8
    {
      // Red component
      let mut image = Image::new(1, 1, RGB8);
      image.set_pixel(0, 0, vec!(255, 0, 0));
      assert!(image.convert_to_grayscale8());
      assert_eq!(image.get_pixel(0, 0), vec!(54));

      // Green component
      image = Image::new(1, 1, RGB8);
      image.set_pixel(0, 0, vec!(0, 255, 0));
      assert!(image.convert_to_grayscale8());
      assert_eq!(image.get_pixel(0, 0), vec!(182));

      // Blue component
      image = Image::new(1, 1, RGB8);
      image.set_pixel(0, 0, vec!(0, 0, 255));
      assert!(image.convert_to_grayscale8());
      assert_eq!(image.get_pixel(0, 0), vec!(18));

      // Already grayscale
      assert!(image.convert_to_grayscale8());
    }


    // RGBA8 to GRAYSCALE8
    {
      // Red component
      let mut image = Image::new(1, 1, RGBA8);
      image.set_pixel(0, 0, vec!(255, 0, 0, 0));
      assert!(image.convert_to_grayscale8());
      assert_eq!(image.get_pixel(0, 0), vec!(54));

      // Green component
      image = Image::new(1, 1, RGBA8);
      image.set_pixel(0, 0, vec!(0, 255, 0, 0));
      assert!(image.convert_to_grayscale8());
      assert_eq!(image.get_pixel(0, 0), vec!(182));

      // Blue component
      image = Image::new(1, 1, RGBA8);
      image.set_pixel(0, 0, vec!(0, 0, 255, 0));
      assert!(image.convert_to_grayscale8());
      assert_eq!(image.get_pixel(0, 0), vec!(18));

      // Alpha component
      image = Image::new(1, 1, RGBA8);
      image.set_pixel(0, 0, vec!(0, 0, 0, 255));
      assert!(image.convert_to_grayscale8());
      assert_eq!(image.get_pixel(0, 0), vec!(0));  

      // Already grayscale
      assert!(image.convert_to_grayscale8());
    }
  }

  #[test]
  fn test_convert_to_rgb8() {
    // GRAYSCALE8 to RGB8
    {
      let mut image = Image::new(1, 1, GRAYSCALE8);
      image.set_pixel(0, 0, vec!(255));
      assert!(image.convert_to_rgb8());
      assert_eq!(image.get_pixel(0, 0), vec!(255, 255, 255));

      image = Image::new(1, 1, GRAYSCALE8);
      assert!(image.convert_to_rgb8());
      assert_eq!(image.get_pixel(0, 0), vec!(0, 0, 0));


      // Already rgb
      assert!(image.convert_to_rgb8());
    }


    // RGBA8 to RGB8
    {
      // Red component
      let mut image = Image::new(1, 1, RGBA8);
      image.set_pixel(0, 0, vec!(255, 0, 0, 0));
      assert!(image.convert_to_rgb8());
      assert_eq!(image.get_pixel(0, 0), vec!(255, 0, 0));

      // Green component
      image = Image::new(1, 1, RGBA8);
      image.set_pixel(0, 0, vec!(0, 255, 0, 0));
      assert!(image.convert_to_rgb8());
      assert_eq!(image.get_pixel(0, 0), vec!(0, 255, 0));

      // Blue component
      image = Image::new(1, 1, RGBA8);
      image.set_pixel(0, 0, vec!(0, 0, 255, 0));
      assert!(image.convert_to_rgb8());
      assert_eq!(image.get_pixel(0, 0), vec!(0, 0, 255));

      // Alpha component
      image = Image::new(1, 1, RGBA8);
      image.set_pixel(0, 0, vec!(0, 0, 0, 255));
      assert!(image.convert_to_rgb8());
      assert_eq!(image.get_pixel(0, 0), vec!(0, 0, 0));  

      // Already rgb
      assert!(image.convert_to_rgb8());
    }
  }

  #[test]
  fn test_convert_to_rgba8() {
    // GRAYSCALE8 to RGBA8
    {
      let mut image = Image::new(1, 1, GRAYSCALE8);
      image.set_pixel(0, 0, vec!(255));
      assert!(image.convert_to_rgba8());
      assert_eq!(image.get_pixel(0, 0), vec!(255, 255, 255, 255));

      image = Image::new(1, 1, GRAYSCALE8);
      assert!(image.convert_to_rgba8());
      assert_eq!(image.get_pixel(0, 0), vec!(0, 0, 0, 255));


      // Already rgb
      assert!(image.convert_to_rgba8());
    }


    // RGB8 to RGBA8
    {
      // Red component
      let mut image = Image::new(1, 1, RGB8);
      image.set_pixel(0, 0, vec!(255, 0, 0));
      assert!(image.convert_to_rgba8());
      assert_eq!(image.get_pixel(0, 0), vec!(255, 0, 0, 255));

      // Green component
      image = Image::new(1, 1, RGB8);
      image.set_pixel(0, 0, vec!(0, 255, 0));
      assert!(image.convert_to_rgba8());
      assert_eq!(image.get_pixel(0, 0), vec!(0, 255, 0, 255));

      // Blue component
      image = Image::new(1, 1, RGB8);
      image.set_pixel(0, 0, vec!(0, 0, 255));
      assert!(image.convert_to_rgba8());
      assert_eq!(image.get_pixel(0, 0), vec!(0, 0, 255, 255));

      // Black pixel
      image = Image::new(1, 1, RGB8);
      image.set_pixel(0, 0, vec!(0, 0, 0));
      assert!(image.convert_to_rgba8());
      assert_eq!(image.get_pixel(0, 0), vec!(0, 0, 0, 255));  

      // Already rgb
      assert!(image.convert_to_rgba8());
    }
  }

}