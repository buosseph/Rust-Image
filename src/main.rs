//extern crate time;

#![allow(unused_imports)]
#![feature(globs)]

use std::path::posix::{Path};
use std::io::File;
use std::os;
use std::str;
use std::uint;

mod image;
mod bmp;

#[allow(dead_code)]
static version: &'static str = "rustc 0.11.0-pre-nightly (380657557cb3793d39dfc0d2321fc946cb3496f5 2014-07-02 00:21:36 +0000)";







/* Processing traits need to updated and tested separately

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

  }

  trait ConvolutionFilter {
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
*/



#[allow(dead_code)]
fn main() {
  let args = os::args();
  if args.len() < 2 {
    fail!("Image path not provided");
  }
  else {

    /*
    let image = bmp::read_bitmap(args.get(1).as_slice());
    match image {
      Some(mut image) => {
        //image.blur();
        bmp::write_bitmap(image, "../image.bmp");
      },
      None  => {
        println!("Looks like you didn't get a valid image.");
      }
    }*/
    
    /*
    match image {
      Some(image) => {

        match image.color_type {
          GRAYSCALE8 => {},
          RGB8 => {},
          RGBA8 => {}
        }

        /*for y in range(0, image.height) {
          for x in range(0, image.width) {
            match image.color_type {
              GRAYSCALE8 => {
                let i = x + image.width * y;
                print!("({}) ", image.data.get(i));
              },
              RGB8 => {
                let pixel = image.get_pixel(x,y);
                print!("{} ", pixel);  
              },
              RGBA8 => {
                let pixel = image.get_pixel(x,y);
                print!("{} ", pixel);  
              },
            }
          }
          print!("\n");
        }*/

      },
      None => {},
    }*/

    

    

  }

}


