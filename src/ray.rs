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

////////////////////////////////////////
//  GLOBAL FUNCTIONS
//////////////////////////////////////// 


////////////////////////////////////////
//  STRUCTS DECLARATION
////////////////////////////////////////

#[derive(Clone)]
pub struct Ray {
    org: Vec3,
    dst: Vec3,
}

////////////////////////////////////////
//  STRUCTS IMPLEMENTATION
////////////////////////////////////////

impl Ray {

    //  instattiate new ray object
    pub fn new( o: Vec3, d: Vec3 ) -> Ray {
        Ray{ org: o, dst: d }
    }

    //  element getter
    #[inline(always)]
    pub fn origin( &self ) -> Vec3 { self.org.clone() }
    #[inline(always)]
    pub fn destination( &self ) -> Vec3 { self.dst.clone() }

    //  calculate position along the ray related to time
    pub fn point_at_param( &self, t: f32 ) -> Vec3 {
        glm::vec3( self.org.x + t * self.dst.x,
                    self.org.y + t * self.dst.y,
                    self.org.z + t * self.dst.z )
    }

    //  calculate unit vector of ray's direction
    pub fn direction( &self ) -> Vec3 {
        self.dst / glm::length( self.dst )
    }
}
