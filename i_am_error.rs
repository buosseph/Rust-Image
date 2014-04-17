use std::slice::from_elem;
use std::slice;
use std::path::posix::{Path};
use std::io::File;
use std::os;
use std::str;

pub struct Pixel {
  r: u8,
  g: u8,
  b: u8,
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


  pub fn read_image(&self, image_path_str: &str) {
    let path = Path::new(image_path_str);

    let mut p_num: ~[u8] = ~[0 as u8, 0 as u8];
    let mut comment: ~[u8] = ~[];
    let mut width: ~[u8] = ~[];

    match File::open(&path) {
      Ok(mut image) => {

        // Find P_ header
        match image.read(p_num) {
          Ok(num_of_bytes) =>  {
            match str::from_utf8(p_num) {
              Some(mode)  => {println!("{}", mode)},    // Check if valid header
              None    => {fail!("Something went wrong converting bytes to str (line 65)")}
            }
          },
          Err(e) => {println!("Something went wrong: {}", e)}
        }

        // Find and remove comments
        loop {
          match image.read_byte() {
            Ok(byte) =>  {
              let byte_string = str::from_byte(byte);

              // If newline, ignore it
              if str::eq(&byte_string, &~"\n") {
                continue;
              }

              // If #, ignore until next newline
              if str::eq(&byte_string, &~"#") {
                comment.push(byte);
                loop {
                  match image.read_byte() {
                    Ok(byte)  => {
                      if str::eq(&byte_string, &~"\n")  {
                        println!("Found new line");
                        break;
                      }
                      else {
                        /* Once this line is introduced into the code
                         * bash text becomes overwritten and unreadable.
                         *
                         * Initial thoughts: code should just be printing
                         * byte data as characters until end of file,
                         * which I believe it does. Maybe creating a buffer
                         * overflow? How can that be done with just printing 
                         * charactesr?
                         */
                        //println!("{}", str::from_byte(byte));
                        continue;
                      }
                    },
                    Err(e)    => {fail!("Something went wrong reading comment(s): {}", e);}
                  }
                }
                println!("{}", str::from_utf8(comment).unwrap());
              }
              break;
            },

            Err(e) => {
              println!("Something went wrong reading file: {}", e);
              break;
            }
          }
        }

      },
      Err(e)    => {println!("Error opening file: {}", e)}
    }
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
}


fn main() {
  let args = os::args();
  if args.len() < 2 {
    fail!("Image path not provided");
  }
  else {
    println!("Path to image: {}", args[1]);
    let ppm_image = PPM::new(0, 0);
    ppm_image.read_image(args[1]);
  }

}