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
use glm::builtin::sqrt;

use crate::ray::Ray;
use sphere::Sphere;

////////////////////////////////////////
//  GLOBAL FUNCTIONS
////////////////////////////////////////


////////////////////////////////////////
//  STRUCTS DECLARATION
////////////////////////////////////////

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

impl Hitable for Sphere {

    //  check if the ray hit the sphere
    fn hit( &self, ray: &Ray, 
            t_min: f32, t_max: f32, 
            hit_rec: &mut HitRecord ) -> bool {

        //  calculate discriminant
        let oc = ray.origin() - self.center();
        let a = glm::dot( ray.destination(), ray.destination() );
        let b = 2.0f32 * glm::dot( oc, ray.destination() );
        let c = glm::dot( oc, oc ) - self.radius() * self.radius();
        let discriminant = b * b - 4.0f32 * a * c;

        //  if discriminant is above zero (2 roots), it definitly hits the sphere.
        if discriminant > 0.0f32 {
            //  calculate first hit on sphere
            let t1 = ( -b - sqrt( discriminant ) ) / ( 2.0f32 * a );
            if t1 <= t_max && t1 >= t_min {
                hit_rec.t = t1;
                hit_rec.p = ray.point_at_param( hit_rec.t );
                hit_rec.n = ( hit_rec.p - self.center() ) / self.radius();
                return true;
            }
            //  calculate second hit on sphere
            let t2 = ( -b + sqrt( discriminant ) ) / ( 2.0f32 * a );
            if t2 <= t_max && t2 >= t_min {
                hit_rec.t = t2;
                hit_rec.p = ray.point_at_param( hit_rec.t );
                hit_rec.n = ( hit_rec.p - self.center() ) / self.radius();
                return true;
            }
        }

        //  else, it definitly hits no sphere.
        false
    }
}
