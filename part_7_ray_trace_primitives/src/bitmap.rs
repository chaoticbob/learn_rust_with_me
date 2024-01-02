#![allow(dead_code)]

use std::fs::File;
use std::io::Write;
use std::vec::Vec;

use stb_image_write_rust::ImageWriter::ImageWriter;

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

    pub fn get_pixels(&self) -> &Vec<u8> {
        &self.data
    }

    pub fn get_scanline(&mut self, y: u32) -> *mut u8 {
        if (y < self.height) {
            let offset: isize = (y as isize) * (self.width as isize) * 4;
            unsafe {
                let ptr = self.data.as_mut_ptr();
                return ptr.offset(offset);
            }
        }
        else {
            panic!("y is out of bounds");
        }
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, r: u8, g: u8, b: u8) {
        let offset = (y * (self.width * 4) + (x * 4)) as usize;
        self.data[offset + 0] = r;
        self.data[offset + 1] = g;
        self.data[offset + 2] = b;
        self.data[offset + 3] = 255;
    }

    pub fn swap_red_and_blue(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let offset = (y * (self.width * 4) + (x * 4)) as usize;
                let r = self.data[offset + 0];
                let b = self.data[offset + 2];
                self.data[offset + 0] = b;
                self.data[offset + 2] = r;
            }
        }
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

    pub fn write_ppm_bgr(&self, file_path: &str) {
        let mut f = File::create(file_path).unwrap();
        writeln!(&mut f, "P3").unwrap();
        writeln!(&mut f, "{} {}", self.width, self.height).unwrap();
        writeln!(&mut f, "255").unwrap();
        for pixel in self.data.chunks(4) {
            writeln!(&mut f, "{: >3} {: >3} {: >3}", pixel[2], pixel[1], pixel[0]).unwrap()
        }
        println!("Wrote PPM file: {}", file_path);
    }

    pub fn write_png(&self, file_path: &str) {
        let mut writer = ImageWriter::new(file_path);
        writer.write_png(self.width as i32, self.height as i32, 4, self.data.as_ptr());
        println!("Wrote PNG file: {}", file_path);
    }
}
