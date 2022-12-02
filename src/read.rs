extern crate image;

pub mod read {

    use image::GenericImageView;

    use super::image_self::GrayscaleImage;

    pub struct Reader {        
    }


    impl Reader {
        pub fn read(filename: &str) -> GrayscaleImage {
            let img = image::open(filename);
            let unwrap_img = img.expect("No valid path provided!");
            let (width, height): (u32, u32) = unwrap_img.dimensions();
            let mut out_vec: Vec<i32> = Vec::new();
            let mut complete_out_vec: Vec<Vec<i32>> = Vec::new();
            for x in 0..height {
                complete_out_vec.push(Vec::new());
                for y in 0..width {
                    let p = unwrap_img.get_pixel(y, x);
                    let luminance = p.0[0] as f32 * 0.33 + p.0[1] as f32 * 0.33 + p.0[2] as f32 * 0.33; 
                    let luminance_i32: i32 = luminance as i32;
                    out_vec.push(luminance_i32);
                    complete_out_vec[x as usize].push(luminance_i32);
                }
            }
            GrayscaleImage { image_vector: out_vec, height: height.try_into().unwrap(), width: width.try_into().unwrap(), complete_img_vector: complete_out_vec }
        }

    }
}

mod math {
    pub(super) fn map_value(upper: i32, real_upper: i32, val: i32) -> i32 {
        let factor: f32 = upper as f32 / real_upper as f32;
        (val as f32 * factor).round() as i32
    }
}

pub mod image_self {

    static ASCII_CHARS: [&'static str; 69] = ["$", "@", "B", "%", "8", "&", "W", "M", "#", "*", "o", "a", "h", "k", "b", "d", "p", "q", "w", "m", 
    "Z", "O", "0", "Q", "L", "C", "J", "U", "Y", "X", "z", "c", "v", "u", "n", "x", "r", "j", "f", "t", "/", "\\", "|", "(", ")", "1", "{", "}", "[", "]", "?", "-", "_", "+", "~",
     "<", ">", "i", "!", "l", "I", ";", ":", ",", "\"", "^", "`", "'", "."]; 

    use std::{fs::File, io::Write};

    use image::{RgbImage, ImageBuffer, Rgb};

    pub struct GrayscaleImage {
        pub image_vector: Vec<i32>,
        pub height: i32,
        pub width: i32,
        pub(super) complete_img_vector: Vec<Vec<i32>>
    }

    impl GrayscaleImage {


        /**
         * 
         */
        pub fn downscale(&mut self, factor: i32) -> () {
            /*
             * Revision 3. Hoffentlich funktioniert das jetzt
            */
            let scaled_height= (self.height as f32 / (factor as f32)) as i32;
            let scaled_width = (self.width as f32 / (factor as f32)) as i32;

            let mut image_d: Vec<i32> = Vec::new();           

                for h in 0..scaled_height {
                    for y in 0..scaled_width {
                        let mut blob = 0;
                        let mut count = 0;
                        for w in 0..factor {
                            for h_i in 0..factor {
                                if !(h+h_i >= self.height) && !(w+y*factor >= self.width) {
                                    blob += self.complete_img_vector[(h*factor+h_i) as usize][(y*factor+w) as usize];
                                    count += 1;
                                }
                            }
                        }
                        blob /= if count != 0 { count } else { 1 };

                        image_d.push(blob);
                    }
                }
            

            self.height = scaled_height;
            self.width = scaled_width;

            self.image_vector = image_d;

            self.map_to_complete_img_vec();

        }

        pub fn save_as(&self, filename: &str) {
            let mut gray_img: RgbImage = ImageBuffer::new(self.width as u32, self.height as u32);
            let mut count = 0;
            for h in 0..self.height {
                for w in 0..self.width {
                    let color_val: u8 = self.image_vector.get(count).unwrap().clone().try_into().unwrap();
                    let pixel = Rgb([color_val, color_val, color_val]);
                    gray_img.put_pixel(w as u32, h as u32, pixel);
                    count += 1;
                }
            }
            let _ = gray_img.save(filename); 
        }


        pub(super) fn map_to_complete_img_vec(&mut self) {
            let mut cnt = 0;
            let mut compl_vec: Vec<Vec<i32>> = Vec::new();
            for h in 0..self.height {
                compl_vec.push(Vec::new());
                for _ in 0..self.width {
                    compl_vec[h as usize].push(self.image_vector[cnt]);
                    cnt += 1;
                }
            }

            self.complete_img_vector = compl_vec;
        }

        pub fn write_as_ascii(&mut self, filename: &str, dark_mode: bool) {
            self.map_to_complete_img_vec();
            let mut out_string = String::new();
            for h in 0..self.height {
                for w in 0..self.width {
                    let val = super::math::map_value(68, 255, self.complete_img_vector[h as usize][w as usize]);
                    let real_val = if dark_mode { (ASCII_CHARS.len() - 1) - val as usize } else { val as usize };
                    out_string += ASCII_CHARS[real_val];
                    out_string += ASCII_CHARS[real_val];
                }
                out_string += "\n";
            }
            
            let mut file = File::create(filename).unwrap();
            let _ = write!(file, "{}", out_string);
        }

    }
}