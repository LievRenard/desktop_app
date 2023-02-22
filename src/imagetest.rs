use image::{GenericImage, GenericImageView, ImageBuffer, DynamicImage};

fn main() {
    let order = 1;
    let mut icon_pixels = get_icon();
    let icon_size = 24.0;
    for i in 0..(24*24) {
        let mut coords = [(12.0 + (icon_size + 12.0) * (order % 5) as f64 + (i%24) as f64, 12.0 + (icon_size + 12.0) * (3 - order / 5) as f64 + (i/24) as f64)];
        let mut color = (icon_pixels[i].0[0], icon_pixels[i].0[1], icon_pixels[i].0[2]);
        if i == 24*24-1 {
            println!("No Problem with coord {}, {} and color {}, {}, {}", coords[0].0, coords[0].1, color.0, color.1, color.2);
        }
    }
}

fn get_icon() -> Vec<image::Rgb<u8>> {
    let mut icon_pixels: Vec<image::Rgb<u8>> = Vec::new();
    let img = image::open("/home/evan/Downloads/firefox.png").unwrap().into_rgb8();
    let resized_img = DynamicImage::from(img).resize(24, 24, image::imageops::Triangle).into_rgb8();
    for x in 0..24 {
        for y in 0..24 {
            icon_pixels.push(*resized_img.get_pixel(x,y));
        }
    }
    icon_pixels
}