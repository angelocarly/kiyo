use std::fs::File;
use std::path::Path;
use std::io::BufWriter;
use std::ops::{AddAssign, DivAssign};

#[derive( Clone, Copy )]
pub struct Color( pub u32, pub u32, pub u32, pub u32 );

pub struct ColorSink {
    width: u32,
    height: u32,
    data: Box<[Color]>
}

impl AddAssign for Color {
    fn add_assign(&mut self, other: Self) {
        self.0 += other.0;
        self.1 += other.1;
        self.2 += other.2;
    }
}

impl DivAssign for Color {
    fn div_assign(&mut self, other: Self) {
        self.0 /= other.0;
        self.1 /= other.1;
        self.2 /= other.2;
    }
}

impl ColorSink {
    pub fn get_width(&self) -> u32 {
        return self.width;
    }

    pub fn get_height(&self) -> u32 {
        return self.height;
    }
}

impl ColorSink {
    pub fn new(width: u32, height: u32) -> Self {
        if width == 0 || height == 0 {
            panic!("Width and height must be greater than 0.");
        }

        let data = vec![Color(0, 0, 0, 1); (width * height) as usize].into_boxed_slice();
        Self { width, height, data }
    }

    pub fn get_pixel(&mut self, x: u32, y: u32 ) -> Color {
        if x >= self.width || y >= self.height {
            panic!("Pixel out of bounds.");
        }

        self.data[(y * self.width + x) as usize]
    }

    pub fn set_pixel(&mut self, x: u32, y: u32, color: Color) {
        if x >= self.width || y >= self.height {
            panic!("Pixel out of bounds.");
        }

        self.data[(y * self.width + x) as usize] = color;
    }

    pub fn set_block(&mut self, i: u32, data: Box<[Color]>) {
        if i >= self.width * self.height {
            panic!("Block out of bounds.");
        }

        let start = i;
        let end = i + data.len() as u32;
        self.data[start as usize..end as usize].clone_from_slice(&data);
    }

    pub fn get_data(&self) -> Box<[Color]> {
        self.data.clone()
    }
}

pub fn write_png_image( in_data: &[u8], width: u32, height: u32, path: &str ) {
    let path = Path::new(path);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height ); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);

    let mut writer = encoder.write_header().unwrap();
    writer.write_image_data( &in_data ).unwrap(); // Save
}

pub fn read_png_image( path: &str ) -> ColorSink {
    let path = Path::new(path);
    let file = File::open(path).unwrap();
    let decoder = png::Decoder::new(file);
    let mut reader = decoder.read_info().unwrap();

    if( reader.info().color_type != png::ColorType::Rgb ) {
        panic!("Image must be RGB");
    }

    let width = reader.info().width;
    let height = reader.info().height;

    let mut data = vec![ 0 as u8; (width * height * 4) as usize ].into_boxed_slice();
    reader.next_frame( &mut data ).unwrap();

    let mut color_data = vec![ Color(0, 0, 0, 1); (width * height) as usize ].into_boxed_slice();
    for i  in 0..(width * height) as usize {
        color_data[i].0 = data[i * 4 + 0] as u32;
        color_data[i].1 = data[i * 4 + 1] as u32;
        color_data[i].2 = data[i * 4 + 2] as u32;
    }

    ColorSink {
        width,
        height,
        data: color_data
    }
}