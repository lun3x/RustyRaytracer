use cgmath::prelude::*;
use crate::objects::*;
use crate::visualiser::*;

const EPSILON: f32 = 0.000005;

pub struct Intersection<'a> {
    pub distance: f32,
    pub object: &'a Object,
}

impl<'a> Intersection<'a> {
    pub fn new(distance: f32, object: &'a Object) -> Self {
        Intersection {
            distance,
            object
        }
    }
}

pub trait Intersectable {
    fn intersection(&self, ray: &Ray) -> Option<f32>;
}

impl Intersectable for Object {
    fn intersection(&self, ray: &Ray) -> Option<f32> {
        match *self {
            Object::Sphere(ref s) => s.intersection(ray),
            Object::Triangle(ref t) => t.intersection(ray),
        }
    }
}

impl Intersectable for Sphere {
    fn intersection(&self, ray: &Ray) -> Option<f32> {
        let to_centre: Vector = self.centre - ray.start;
        // Right angle triangle side lengths, with to_centre as the hypotenuse
        let adjacent: f32 = ray.dir.dot(to_centre);
        let opposite_squared: f32 = to_centre.dot(to_centre) - (adjacent * adjacent);
        let radius_squared: f32 = self.radius * self.radius;

        // No intersect if opposite is greater than radius^2
        if opposite_squared > radius_squared {
            return None;
        }

        // Length of chord (section of ray that is intersecting)
        let half_chord: f32 = (radius_squared - opposite_squared).sqrt();
        // Intersection distances
        let isect0 = adjacent - half_chord;
        let isect1 = adjacent + half_chord;
        
        // Dont return intersections behind camera
        if isect0.is_sign_negative() && isect1.is_sign_negative() {
            return None;
        }

        Some(if isect0 < isect1 {isect0} else {isect1})
    }
}

impl Intersectable for Triangle {
    fn intersection(&self, ray: &Ray) -> Option<f32> {
        // Moller-Trumbore intersection algorithm

        let v0v1: Vector = self.v1 - self.v0;
        let v0v2: Vector = self.v2 - self.v0;

        let pvec: Vector = ray.dir.cross(v0v2);
        let determinant: f32 = v0v1.dot(pvec);

        // cull backfacing triangles
        if determinant < EPSILON { 
            return None; 
        }
        // avoid parallel rays
        if determinant.abs() < EPSILON { 
            return None; 
        }

        // compute 'u' barycentric coord
        let inv_det: f32 = 1.0 / determinant;
        let tvec: Vector = ray.start - self.v0;
        let u: f32 = tvec.dot(pvec) * inv_det;
        if u < 0.0 || u > 1.0 { 
            return None; 
        }

        // compute 'v' barycentric coord
        let qvec: Vector = tvec.cross(v0v1);
        let v: f32 = ray.dir.dot(qvec) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }

        let t: f32 = v0v2.dot(qvec) * inv_det;
        Some(t)
    }
}