#![allow(dead_code)]

use std::fs::File;
use std::io::Write;
use std::vec::Vec;

pub struct Bitmap {
    pub width: u32,
    pub height: u32,
    data: Vec<u8>,
}

impl Bitmap {
    pub fn new(width: u32, height: u32) -> Bitmap {
        Bitmap {
            width,
            height,
            data: vec![0; (width * height * 4) as usize],
        }
    }

    pub fn fill(&mut self, r: u8, g: u8, b: u8, a: u8) {
        for pixel in self.data.chunks_mut(4) {
            pixel[0] = r;
            pixel[1] = g;
            pixel[2] = b;
            pixel[3] = a;
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        let offset = (y * (self.width * 4) + (x * 4)) as usize;
        self.data[offset + 0] = r;
        self.data[offset + 1] = g;
        self.data[offset + 2] = b;
    }

    pub fn write_ppm(&self, file_path: &str) {
        let mut f = File::create(file_path).unwrap();
        writeln!(&mut f, "P3").unwrap();
        writeln!(&mut f, "{} {}", self.width, self.height).unwrap();
        writeln!(&mut f, "255").unwrap();
        for pixel in self.data.chunks(4) {
            writeln!(&mut f, "{: >3} {: >3} {: >3}", pixel[0], pixel[1], pixel[2]).unwrap()
        }
        println!("Wrote PPM file: {}", file_path);
    }
}
