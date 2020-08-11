////////////////////////////////////////
//  CRATES
////////////////////////////////////////


////////////////////////////////////////
//  MODULES
////////////////////////////////////////

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

#[derive(Clone)]
pub struct Camera
{
    origin: Vec3,
    ll_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
}

////////////////////////////////////////
//  STRUCTS IMPLEMENTATION
////////////////////////////////////////

impl Camera
{
    //  default constructor 
    pub fn new() -> Camera
    {
        Camera
        {
            origin: glm::vec3( 0.0f32, 0.0f32, 0.0f32 ),
            ll_corner: glm::vec3( -2.0f32, -1.0f32, -1.0f32 ),
            horizontal: glm::vec3( 4.0f32, 0.0f32, 0.0f32 ),
            vertical: glm::vec3( 0.0f32, 2.0f32, 0.0f32 )
        }
    }

    pub fn get_ray( &self, u: f32, v: f32 ) -> Ray
    {
        Ray::new( self.origin.clone(), self.ll_corner + self.horizontal * u + self.vertical * v - self.origin )
    }
}