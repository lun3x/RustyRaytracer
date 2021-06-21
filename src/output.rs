use std::fs::File;
use std::io::Write;
use crate::constants::*;

pub fn write_to_ppm_file(screen: &[u8]) {
    let mut file = File::create("./render.ppm").expect("Unable to create render file");
    write!(file, "P3\n{width} {height}\n255\n", width=SCREEN_WIDTH, height=SCREEN_HEIGHT).expect("Unable to write header to file");
    for pix in screen.chunks_exact(3) {
        write!(file, "{r} {g} {b}\n", r=pix[0], g=pix[1], b=pix[2]).expect("Unable to write pixel");
    }
}