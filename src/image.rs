use std::{fs::File, io::{self, BufWriter}};

use image::ImageBuffer;
use lib_rs::color::Color;



pub struct Image{
    dimension:(u32,u32),
    pixels:Vec<Color>
}
impl Image{
    pub fn new(width:u32,height:u32,pixels:Vec<Color>)->Self{
        Self { pixels:pixels,dimension:(width,height) }
    }
    pub fn write_to_file(self){
        let mut buffer:Vec<u8> = Vec::with_capacity(self.pixels.len()*4);
        for c in self.pixels{
            buffer.extend_from_slice(&c.as_rgba8_bytes() );
        }
        let mut writer = BufWriter::new(File::open("output.png").unwrap());
        image::write_buffer_with_format(&mut writer, &buffer, self.dimension.0, self.dimension.1, image::ColorType::Rgba8, image::ImageFormat::Png).unwrap();
    }
}