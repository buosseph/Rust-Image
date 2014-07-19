// Image module, where all data is stored an manipulated

// extern crate time;

#![allow(unused_imports)]

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


/*

// Image processing traits and functions (Not implemented for all color types)
trait PointProcessor {
  fn negative(&mut self);
  fn brighten(&mut self, bias: int);
  fn contrast(&mut self, gain: f32);
  fn saturate(&mut self, gain: f32);
}

impl PointProcessor for Image {

  // Theta(n)
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


  // Theta(n)
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


  // Updated for all but GRAYSCALE8
  // Theta(n^2)
  #[allow(dead_code)]
  fn contrast(&mut self, gain: f32) {
    let mut total_luminance: f32 = 0.;

    for y in range(0, self.height){
      for x in range(0, self.width){

        match self.color_type {
          GRAYSCALE8 => {

            unimplemented!();

          },

          RGB8 => {

            let mut pixel_data: Vec<u8> = self.get_pixel(x,y);
            let b  = pixel_data.pop().unwrap();
            let g  = pixel_data.pop().unwrap();
            let r  = pixel_data.pop().unwrap();

            let red   = r as f32;
            let green = g as f32;
            let blue  = b as f32;

            let luminance: f32 = 0.2126 * red  + 0.7152 * green  + 0.0722 * blue;
            total_luminance += luminance;

          },

          RGBA8 => {

            let mut pixel_data: Vec<u8> = self.get_pixel(x,y);
            pixel_data.pop().unwrap();
            let b     = pixel_data.pop().unwrap();
            let g     = pixel_data.pop().unwrap();
            let r     = pixel_data.pop().unwrap();

            let red   = r as f32;
            let green = g as f32;
            let blue  = b as f32;

            let luminance: f32 = 0.2126 * red  + 0.7152 * green  + 0.0722 * blue;
            total_luminance += luminance;

          }
        }

      }
    }

    let mean_luminance: f32 = total_luminance/((self.width*self.height) as f32);

    for y in range(0, self.height){
      for x in range(0, self.width){

        match self.color_type {
          GRAYSCALE8 => {

            unimplemented!();

          },

          RGB8 => {

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

          },

          RGBA8 => {

            let mut pixel_data: Vec<u8> = self.get_pixel(x,y);
            let alpha = pixel_data.pop().unwrap();
            let b     = pixel_data.pop().unwrap();
            let g     = pixel_data.pop().unwrap();
            let r     = pixel_data.pop().unwrap();

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
            
            let pixel_data: Vec<u8> = vec!(red as u8, green as u8, blue as u8, alpha as u8);
            self.set_pixel(x,y, pixel_data);

          }
        }



      }
    }
  }


  // Updated for all but GRAYSCALE8
  // Theta(n)
  #[allow(dead_code)]
  fn saturate(&mut self, gain: f32) {
    for y in range(0, self.height){
      for x in range(0, self.width){

        match self.color_type {
          GRAYSCALE8 => {

            unimplemented!();

          },

          RGB8 => {

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

          },

          RGBA8 => {

            let mut pixel_data: Vec<u8> = self.get_pixel(x,y);
            let alpha = pixel_data.pop().unwrap();
            let b     = pixel_data.pop().unwrap();
            let g     = pixel_data.pop().unwrap();
            let r     = pixel_data.pop().unwrap();

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
            
            let pixel_data: Vec<u8> = vec!(red as u8, green as u8, blue as u8, alpha as u8);
            self.set_pixel(x,y, pixel_data);

          }
        }

      }
    }
  }

}*/

pub trait ConvolutionFilter {
  fn blur(&mut self);
}

impl ConvolutionFilter for Image {

  // Theta(n)
  #[allow(dead_code)]
  fn blur(&mut self) {
    // Brute force        Time: 264835676 ns
    // let start = time::precise_time_ns();
  
    let kernel = [[1, 1, 1], [1, 1, 1], [1, 1, 1]];
    let kernel_sum = 9;
    let kernel_center_x: uint = 3/2;
    let kernel_center_y: uint = 3/2;


    match self.color_type {

      GRAYSCALE8 => {

        for x in range(0, self.width){
          for y in range(0, self.height){

            let mut lum_sum = 0;

            for kernel_row in range(0, 3){
              for kernel_column in range(0, 3){

                let kx: int = kernel_row - (kernel_center_y - x) as int;
                let ky: int = kernel_column - (kernel_center_x - y) as int;

                if kx >= 0 && kx < (self.width as int) && ky >= 0 && ky < (self.height as int){

                  let kernel_value = kernel[kernel_row as uint][kernel_column as uint];

                  let mut pixel_data: Vec<u8> = self.get_pixel(kx as uint , ky as uint);
                  let l  = pixel_data.pop().unwrap();

                  lum_sum   += l as int * kernel_value;            

                }  

              }
            }

            lum_sum = lum_sum/kernel_sum;

            if lum_sum > 255 {lum_sum = 255;}

            if lum_sum < 0 {lum_sum = 0;}

            let pixel_data: Vec<u8> = vec!(lum_sum as u8);
            self.set_pixel(x as uint,y as uint, pixel_data);
            
          }
        }

      },

      RGB8 => {

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

      },

      RGBA8 => {

        for x in range(0, self.width){
          for y in range(0, self.height){

            let mut red_sum = 0;
            let mut green_sum = 0;
            let mut blue_sum = 0;
            let mut center_alpha = 0;

            for kernel_row in range(0, 3){
              for kernel_column in range(0, 3){

                let kx: int = kernel_row - (kernel_center_y - x) as int;
                let ky: int = kernel_column - (kernel_center_x - y) as int;

                if kx >= 0 && kx < (self.width as int) && ky >= 0 && ky < (self.height as int){

                  let kernel_value = kernel[kernel_row as uint][kernel_column as uint];

                  let mut pixel_data: Vec<u8> = self.get_pixel(kx as uint , ky as uint);
                  
                  if kernel_row == kernel_center_x as int && kernel_column == kernel_center_y as int {
                    center_alpha = pixel_data.pop().unwrap();
                  }
                  else {
                    pixel_data.pop().unwrap();
                  }


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

            let pixel_data: Vec<u8> = vec!(red_sum as u8, green_sum as u8, blue_sum as u8, center_alpha as u8);
            self.set_pixel(x as uint,y as uint, pixel_data);
            
          }
        }

      }
    }


    // let end = time::precise_time_ns();
    // let time = end as uint - start as uint;
    // println!("Time of brute force algorithm: {}", time);
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

#[cfg(test)]
mod test_processing {
  use super::*;
  use bmp::*;

  // All function results must be reviewed visually


  #[test]
  fn test_blur() {

    let test_images = vec!(
      "testimage_rgba_v5.bmp",
      "testimage_rgb_v4.bmp",
      "grayscale_v3.bmp",
    );

    for filename in test_images.iter() {

      let path_prefix: String = "../bmp/".to_string();
      let path_to_file: String = path_prefix.append(*filename);
      let image = read_bitmap(path_to_file.as_slice());
      
      match image {
        Some(mut image) => {
          let path_prefix: String = "../test/image/".to_string();
          let path_to_write: String = path_prefix.append("blur_").append(*filename);

          image.blur();

          assert!(write_bitmap(image, path_to_write.as_slice()));
        },
        None  => {
          fail!("Looks like you didn't get a valid image.");
        }
      }

    }

  }

}