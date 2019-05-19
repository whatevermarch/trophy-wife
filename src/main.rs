////////////////////////////////////////
//  CRATES
////////////////////////////////////////

extern crate glm;
extern crate png;

////////////////////////////////////////
//  MODULES
////////////////////////////////////////

mod vk;
mod ray;

////////////////////////////////////////
//  ALIASES
////////////////////////////////////////

use glm::Vec3;
use ray::Ray;

////////////////////////////////////////
//  GLOBAL DECLARATIONS
////////////////////////////////////////

//  check if the ray hit the sphere
fn hit_sphere( center: &Vec3, radius: f32, r: &Ray ) -> bool {

    //  calculate discriminant
    let oc = r.origin() - center.clone();
    let a = glm::dot( r.destination(), r.destination() );
    let b = 2.0f32 * glm::dot( oc, r.destination() );
    let c = glm::dot( oc, oc ) - radius * radius;
    let discriminant = b * b - 4.0f32 * a * c;

    //  if discriminant is above zero (2 roots), it definitly hits the sphere.
    discriminant > 0.0f32
}

//  calculate color at which the ray hit
fn color( r: &Ray ) -> Vec3 {

    //  check if this ray hit the sphere
    if hit_sphere( &glm::vec3( 0.0f32, 0.0f32, -1.0f32 ), 0.5, &r ) {
        return glm::vec3( 1.0f32, 0.0f32, 0.0f32 )
    }

    //  calculate t
    let dir: Vec3 = r.direction();
    let t = 0.5f32 * ( dir.y + 1.0f32 );

    //  return color
    glm::vec3( 1.0f32, 1.0f32, 1.0f32 ) * ( 1.0f32 - t ) +  glm::vec3( 0.5f32, 0.7f32, 1.0f32 ) * t
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

    //  determine image boundaries
    let ll_pos = glm::vec3( -2.0f32, -1.0f32, -1.0f32 );
    let horizontal = glm::vec3( 4.0f32, 0.0f32, 0.0f32 );
    let vertical = glm::vec3( 0.0f32, 2.0f32, 0.0f32 );
    let origin = glm::vec3( 0.0f32, 0.0f32, 0.0f32 );

    //  construct data array containing a RGBA sequence.
    let mut data_vec = Vec::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new( origin.clone(), ll_pos + horizontal * u + vertical * v );
            let c = color( &r );
            let ir = ( 255.99f32 * c.x ) as u8;
            let ig = ( 255.99f32 * c.y ) as u8;
            let ib = ( 255.99f32 * c.z ) as u8;

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
    // use vk::core::VkCore;
    // let vk_core = VkCore::new();

    //  render image
    render_image();
}
