////////////////////////////////////////
//  CRATES
////////////////////////////////////////


////////////////////////////////////////
//  MODULES
////////////////////////////////////////

mod vk;
mod ray;
mod shape;

////////////////////////////////////////
//  ALIASES
////////////////////////////////////////

use glm::Vec3;

use ray::Ray;

use shape::{ HitRecord, Hitable };
use shape::sphere::Sphere;

////////////////////////////////////////
//  GLOBAL DECLARATIONS
////////////////////////////////////////

//  output image file name
static OUT_FILE_NAME: &str = "out_image.png";

//  check if this ray hit something
fn hit_anything( r: &Ray, shape_list: &Vec<Box<Hitable>>, 
                t_min: f32, t_max: f32, 
                hit_rec: &mut HitRecord ) -> bool {
    
    //  loop all shapes and check if this ray hit something
    let mut hit_any = false;
    let mut closest_so_far = t_max;
    for shape in shape_list {
        if shape.hit( &r, t_min, closest_so_far, hit_rec ) {
            hit_any = true;

            //  get the closest hit
            closest_so_far = hit_rec.t;
        }
    }

    hit_any
}

//  calculate color at which the ray hit
fn color( r: &Ray, shape_list: &Vec<Box<Hitable>> ) -> Vec3 {

    //  construct blank hit record
    let mut hit_record = HitRecord{ 
        t: 0.0f32, 
        p: glm::vec3( 0.0f32, 0.0f32, 0.0f32 ), 
        n: glm::vec3( 0.0f32, 0.0f32, 0.0f32 ) 
    }; 

    //  check if this ray hit some spheres
    if hit_anything( &r, &shape_list, 0.0f32, 1e37f32, &mut hit_record ) {
        return ( hit_record.n + 1.0f32 ) * 0.5f32;
    }

    //  if we reach this state, that means it hits nothing
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
    let path = Path::new( OUT_FILE_NAME );
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

    //  construct shape list
    let mut shape_list = Vec::new();
    shape_list.push( Box::new( Sphere::new( glm::vec3( 0.0f32, 0.0f32, -1.0f32 ), 0.5f32 ) ) as Box<Hitable> );
    shape_list.push( Box::new( Sphere::new( glm::vec3( 0.0f32, -100.5f32, -1.0f32 ), 100.0f32 ) ) as Box<Hitable> );

    //  construct data array containing a RGBA sequence.
    let mut data_vec = Vec::new();
    for j in (0..ny).rev() {
        for i in 0..nx {
            let u = i as f32 / nx as f32;
            let v = j as f32 / ny as f32;
            let r = Ray::new( origin.clone(), ll_pos + horizontal * u + vertical * v );
            let c = color( &r, &shape_list );
            let ir = ( 255.99f32 * c.x ) as u8;
            let ig = ( 255.99f32 * c.y ) as u8;
            let ib = ( 255.99f32 * c.z ) as u8;

            data_vec.extend( [ ir, ig, ib, 255 ].iter().clone() );
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
