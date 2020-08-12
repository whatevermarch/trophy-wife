////////////////////////////////////////
//  CRATES
////////////////////////////////////////


////////////////////////////////////////
//  MODULES
////////////////////////////////////////

// mod vk;
mod ray;
mod shape;
mod camera;

////////////////////////////////////////
//  ALIASES
////////////////////////////////////////

use glm::Vec3;
use rand::Rng;

use ray::Ray;

use shape::{ ShapeCollection, HitRecord, Hitable };
use shape::sphere::Sphere;

use camera::Camera;

////////////////////////////////////////
//  GLOBAL DECLARATIONS
////////////////////////////////////////

//  define constants
static OUT_FILE_NAME: &str = "out_image.png";
static MAX_FLOAT: f32 = 1e37f32;

fn random_in_unit_sphere() -> Vec3
{
    //  initialize random number generator
    let mut rng = rand::thread_rng();

    loop
    {
        let p = glm::vec3( rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>() ) * 2.0f32 
            - glm::vec3( 1.0f32, 1.0f32, 1.0f32 );
        if glm::length( p.clone() ) >= 1.0f32
        {
            return p;
        }
    }
}

//  calculate color at which the ray hit
fn color( r: &Ray, scene: &ShapeCollection, bounces: i16 ) -> Vec3
{
    //  construct blank hit record
    let mut hit_record = HitRecord::new(); 

    //  limit the bounces
    if bounces >= 0
    {
        //  check if this ray hit some spheres
        if scene.hit( r, 0.0001f32, MAX_FLOAT, &mut hit_record )
        {
            let target = hit_record.p + hit_record.n + random_in_unit_sphere();
            let new_ray = Ray::new( hit_record.p.clone(), target - hit_record.p );
            return color( &new_ray, scene, bounces - 1 ) * 0.5f32;
        }
    }

    //  if we reach this state, that means it hits nothing
    //  calculate t
    let dir: Vec3 = r.direction();
    let t = 0.5f32 * ( dir.y + 1.0f32 );

    //  return color
    glm::vec3( 1.0f32, 1.0f32, 1.0f32 ) * ( 1.0f32 - t ) +  glm::vec3( 0.5f32, 0.7f32, 1.0f32 ) * t
}

//  render image function
fn render_image( w: u16, h: u16, num_samples: u16 ) 
{
    //  determine image size
    let nx = w;
    let ny = h;

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
    let mut encoder = png::Encoder::new(w, nx as u32, ny as u32); // Width is 2 pixels and height is 1.
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    //  construct scene
    let mut shape_list = Vec::new();
    shape_list.push( Box::new( Sphere::new( glm::vec3( 0.0f32, 0.0f32, -1.0f32 ), 0.5f32 ) ) as Box<dyn Hitable> );
    shape_list.push( Box::new( Sphere::new( glm::vec3( 0.0f32, -100.5f32, -1.0f32 ), 100.0f32 ) ) as Box<dyn Hitable> );
    let scene = ShapeCollection::new( &shape_list );

    //  construct camera
    let cam = Camera::new();

    //  initialize random number generator
    let mut rng = rand::thread_rng();

    //  construct data array containing a RGBA sequence.
    let mut data_vec = Vec::new();
    for j in (0..ny).rev() 
    {
        for i in 0..nx 
        {
            //  evaluate color fetched from rays
            let mut c = glm::vec3( 0.0f32, 0.0f32, 0.0f32 );
            for _ in 0..num_samples 
            {
                let u = ( i as f32 + rng.gen::<f32>() ) / nx as f32;
                let v = ( j as f32 + rng.gen::<f32>() ) / ny as f32;
                let r = cam.get_ray( u, v );
                //let p = r.point_at_param( 2.0f32 );
                c = c + color( &r, &scene, 3 );
            }
            c = c / num_samples as f32;

            //  adjust gamma (w/ gamma = 2)
            c = glm::sqrt(c);

            //  encode color
            let ir = ( 255.99f32 * c.x ) as u8;
            let ig = ( 255.99f32 * c.y ) as u8;
            let ib = ( 255.99f32 * c.z ) as u8;

            //  record the result color
            data_vec.extend( [ ir, ig, ib, 255 ].iter().clone() );
        }
    }

    //  save file
    writer.write_image_data( &data_vec ).unwrap(); 
}

////////////////////////////////////////
//  MAIN FUNCTION
////////////////////////////////////////

fn main() 
{
    //  initialize vulkan core
    // use vk::core::VkCore;
    // let vk_core = VkCore::new();

    //  render image
    render_image( 200, 100, 50 );
}
