mod image_creator;
use image::{GenericImageView, imageops::FilterType};
use std::env;
use std::fs;

fn generate_unique_filename() -> String {
    for i in 1.. {
        let filename = format!("ascii_output_{}.png", i);
        if !fs::metadata(&filename).is_ok() {
            return filename;
        }
    }
    "ascii_output.png".to_string() 
}

const ASCII_CHARS: &[u8] = b"`.-':_,^=;><+!rc*/z?sLTv)J7(|Fi{C}fI31tlu[neoZ5Yxjya]2ESwqkP6h9d4VpOGbUAKXHm8RD#$Bg0MNWQ%&@";

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <image_path>", args[0]);
        return;
    }

    let img_path = &args[1];
    let img = image::open(img_path).expect("Failed to open image");

    let (orig_width, orig_height) = img.dimensions();
    let target_width = 300;
    let char_aspect_ratio = 0.5;
    let scale_ratio = target_width as f32 / orig_width as f32;
    let new_height = (orig_height as f32 * scale_ratio * char_aspect_ratio) as u32;

    let resized = image::imageops::resize(&img, target_width, new_height, FilterType::Nearest);

    let mut ascii: Vec<Vec<char>> = Vec::new();

    for y in 0..resized.height() {
        let mut row: Vec<char> = Vec::new();
        for x in 0..resized.width() {
            let pixel = resized.get_pixel(x, y);
            let r = pixel[0] as u32;
            let g = pixel[1] as u32;
            let b = pixel[2] as u32;

            let avg_brightness = (0.299 * r as f32 + 0.587 * g as f32 + 0.114 * b as f32) as u8;
            let index = ((255 - avg_brightness) as usize * (ASCII_CHARS.len() - 1)) / 255;
            let c = ASCII_CHARS[index] as char;

            row.push(c);
        }
        ascii.push(row);
    }

   let filename = generate_unique_filename();
    image_creator::create_image_from_ascii(&ascii, &filename);
    println!("Done! The ASCII image has been saved to {}", filename);
}