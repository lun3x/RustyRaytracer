use crate::objects::*;
use crate::scene::*;
use crate::visualiser::*;
use cgmath::prelude::*;

pub struct Ray {
    pub start: Point,
    pub dir: Vector,
}

impl Ray {
    pub fn reflect(&self, normal: &Vector) -> Vector {
        self.dir - (2.0 * self.dir.dot(*normal) * normal)
    }
}

pub fn trace(ray: Ray, scene: &Scene) -> Colour {
    match scene.closest_intersection(&ray) {
        Some(i) => {
            let isect_position: Point = i.location.distance * ray.dir;
            use Material::*;
            match *i.object.get_material() {
                Specular => {
                    let reflected_ray = Ray {
                        start: isect_position,
                        dir: ray.reflect(&i.object.get_normal(isect_position)),
                    };
                    trace(reflected_ray, scene)
                }
                Diffuse => i.object.get_colour(i.location.texture_coords),
            }
        }
        _ => [0, 0, 0],
    }
}
