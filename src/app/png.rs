use std::fs::File;
use std::path::Path;
use std::io::BufWriter;

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