////////////////////////////////////////
//  CRATES
////////////////////////////////////////


////////////////////////////////////////
//  MODULES
////////////////////////////////////////

pub mod sphere;

////////////////////////////////////////
//  ALIASES
////////////////////////////////////////

use glm::Vec3;

use crate::ray::Ray;

////////////////////////////////////////
//  GLOBAL FUNCTIONS
////////////////////////////////////////


////////////////////////////////////////
//  STRUCTS DECLARATION
////////////////////////////////////////

pub struct ShapeCollection<'a> {
    list: &'a Vec<Box<dyn Hitable>>,
}

#[derive(Clone)]
pub struct HitRecord {
    pub t: f32,
    pub p: Vec3,
    pub n: Vec3,
}

pub trait Hitable {
    fn hit( &self, ray: &Ray, 
            t_min: f32, t_max: f32, 
            hit_rec: &mut HitRecord ) -> bool;
}

////////////////////////////////////////
//  STRUCTS IMPLEMENTATION
////////////////////////////////////////

impl<'a> ShapeCollection<'a>
{
    //  default constructor
    pub fn new( shape_list: &Vec<Box<dyn Hitable>> ) -> ShapeCollection 
    {
        ShapeCollection{ list: shape_list }
    }
}

impl<'a> Hitable for ShapeCollection<'a>
{
    //  check if the ray hit something in the collection
    fn hit( &self, ray: &Ray, 
        t_min: f32, t_max: f32, 
        hit_rec: &mut HitRecord ) -> bool 
    {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for shape in self.list.iter()
        {
            if (*shape).hit( ray, t_min, closest_so_far, hit_rec )
            {
                hit_anything = true;
                closest_so_far = hit_rec.t;
            }
        }

        hit_anything
    }
}

impl HitRecord
{
    //  default constructor
    pub fn new() -> HitRecord 
    {
        HitRecord
        { 
            t: 0.0f32, 
            p: glm::vec3( 0.0f32, 0.0f32, 0.0f32 ), 
            n: glm::vec3( 0.0f32, 0.0f32, 0.0f32 ) 
        }
    }
}
