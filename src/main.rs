mod read;

use read::read::Reader;


fn main() {
    // Read the images into the Variables
    let mut cat = Reader::read("./testing/cat.jpg");
    let mut heatmap = Reader::read("./testing/heatmap.jpg");
    let mut phonescreen = Reader::read("./testing/phone_screen.jpeg");

    // Downscale the images by a factor
    cat.downscale(2);
    heatmap.downscale(2);
    phonescreen.downscale(10);


    // Save the grayscaled and downscaled images
    cat.save_as("./grayscaled/cat_small_gray.jpg");
    heatmap.save_as("./grayscaled/heatmap_small_gray.jpg");
    phonescreen.save_as("./grayscaled/phone_screen_small_gray.jpeg");


    // Write them as ascii
    cat.write_as_ascii("./ascii_img/cat.txt", true);
    heatmap.write_as_ascii("./ascii_img/heatmap.txt", true);
    phonescreen.write_as_ascii("./ascii_img/phonescreen.txt", true);
}
