use crate::intersect::*;
use crate::objects::*;
use crate::scene::*;
use crate::utils;
use crate::visualiser::*;
use cgmath::prelude::*;

const BACKGROUND: ColourFloat = ColourFloat::new(0.0, 0.0, 0.0);

pub struct Ray {
    pub start: Point,
    pub dir: Vector,
}

impl Ray {
    pub fn reflect(&self, normal: &Vector) -> Vector {
        self.dir - 2.0 * self.dir.dot(*normal) * normal
    }

    pub fn rotate(&self, rotation_matrix: RotationMatrix) -> Self {
        let dir4 = utils::to_4(self.dir);
        let rotated_dir = rotation_matrix * dir4;

        Ray {
            start: self.start,
            dir: utils::to_3(rotated_dir),
        }
    }
}

pub fn trace(ray: Ray, scene: &Scene, depth: u32) -> ColourFloat {
    match scene.closest_intersection(&ray) {
        Some(i) => {
            let isect_position: Point = i.location.distance * ray.dir;
            use Material::*;
            match *i.object.get_material() {
                Specular => {
                    if depth > 0 {
                        let normal = i.object.get_normal(isect_position);
                        let reflected_ray = Ray {
                            start: isect_position + (normal * 0.005),
                            dir: ray.reflect(&normal),
                        };
                        0.9 * trace(reflected_ray, scene, depth - 1)
                    } else {
                        i.object.get_colour(i.location.texture_coords)
                    }
                }
                Diffuse => i.object.get_colour(i.location.texture_coords),
            }
        }
        _ => BACKGROUND,
    }
}
