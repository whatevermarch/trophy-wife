////////////////////////////////////////
//  CRATES
////////////////////////////////////////

extern crate glm;
extern crate png;

////////////////////////////////////////
//  MODULES
////////////////////////////////////////

mod vk;

////////////////////////////////////////
//  ALIASES
////////////////////////////////////////

use glm::Vector3;

////////////////////////////////////////
//  GLOBAL DECLARATIONS
////////////////////////////////////////

type fvec3 = Vector3<f32>;

struct Ray {
    org: fvec3,
    dst: fvec3,
}

fn point_at_param( r: &Ray, t: f32 ) -> fvec3 {
    r.org + glm::vec3( t, t, t ) * r.dst
}

//  render image function
fn render_image() {

    //  determine image size
    let nx = 200;
    let ny = 100;

    //  local aliases for reading and opening files
    use std::path::Path;
    use std::fs::File;
    use std::io::BufWriter;

    //  for encoder.set()
    use png::HasParameters;

    //  create image file
    let path = Path::new("out_image.png");
    // let display = path.display();
    // println!("{}", display);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    //  setup image
    let mut encoder = png::Encoder::new(w, nx, ny); // Width is 2 pixels and height is 1.
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    //  construct data array containing a RGBA sequence.
    let mut data_vec = Vec::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let r: f32 = i as f32 / nx as f32;
            let g: f32 = j as f32 / ny as f32;
            let b: f32 = 0.2;
            let ir: u8 = ( 255.99 * r ) as u8;
            let ig: u8 = ( 255.99 * g ) as u8;
            let ib: u8 = ( 255.99 * b ) as u8;

            data_vec.extend( [ ir, ig, ib, 255 ].iter().clone() )
        }
    }

    //  save file
    writer.write_image_data( &data_vec ).unwrap(); 
}

////////////////////////////////////////
//  MAIN FUNCTION
////////////////////////////////////////

fn main() {

    //  initialize vulkan core
    use vk::core::VkCore;
    let vk_core = VkCore::new();

    //  render image
    render_image();

}
 