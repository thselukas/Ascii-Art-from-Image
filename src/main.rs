mod read;

extern crate bmp;

use read::read::Reader;

use read::image_self::GrayscaleImage;

fn main() {
    let mut img: GrayscaleImage = Reader::read_and_grayscale("file.bmp");
    println!("Width: {} * {}", img.width, img.height);
    println!("{:?}", img.image_vector.len());
    img.downscale_and_blur();
    println!("New SIZE: {} * {}", img.width, img.height);
    img.save_as("gray_new.bmp");
}
