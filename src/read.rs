extern crate bmp;


pub mod read {
    use std::{fs::OpenOptions};

    use super::image_self::GrayscaleImage;

    pub struct Reader {        
    }

    impl Reader {
        pub fn read_and_grayscale(filename: &str) -> GrayscaleImage {
            let f = OpenOptions::new().read(true).open(filename);
            let mut file = f.expect("Valid file is not given");
            let image = bmp::from_reader(&mut file);
            let unwrap_img: bmp::Image = image.expect("Expected valid image!");
            let (height, width): (u32, u32) = (unwrap_img.get_height(), unwrap_img.get_width());
            let mut out_vec: Vec<i32> = Vec::new();
            let mut complete_out_vec: Vec<Vec<i32>> = Vec::new();
            for x in 0..height {
                complete_out_vec.push(Vec::new());
                for y in 0..width {
                    let p = unwrap_img.get_pixel(y, x);
                    let luminance = p.r as f32 * 0.2126 + p.g as f32 * 0.7152 + p.b as f32 * 0.0722; // Von Wikipedia geklaut!
                    // let luminance = p.r as f32 * 0.33 + p.g as f32 * 0.33 + p.b as f32 * 0.33; eigener Versuch!
                    let luminance_i32: i32 = luminance as i32;
                    out_vec.push(luminance_i32);
                    complete_out_vec[x as usize].push(luminance_i32);
                }
            }
            GrayscaleImage { image_vector: out_vec, height: height.try_into().unwrap(), width: width.try_into().unwrap(), complete_img_vector: complete_out_vec }
        }
    }
}

pub mod image_self {

    use bmp::{Image, Pixel};

    pub struct GrayscaleImage {
        pub image_vector: Vec<i32>,
        pub height: i32,
        pub width: i32,
        pub(super) complete_img_vector: Vec<Vec<i32>>
    }

    impl GrayscaleImage {
        pub fn downscale_and_blur(&mut self) -> bool {

            false
        }

        pub fn save_as(&self, filename: &str) {
            let mut gray_img = Image::new(self.width as u32, self.height as u32);
            let mut count = 0;
            for h in 0..self.height {
                for w in 0..self.width {
                    let color_val: u8 = self.image_vector.get(count).unwrap().clone().try_into().unwrap();
                    gray_img.set_pixel(w as u32, h as u32, Pixel::new(color_val, color_val, color_val));
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
                for w in 0..self.width {
                    compl_vec[h as usize].push(self.image_vector[cnt]);
                    cnt += 1;
                }
            }
        }
    }
}