use crate::objects::*;
use crate::intersect::*;
use crate::visualiser::*;

pub struct Scene {
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn closest_intersection(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest_dist = f32::MAX;
        let mut closest_isec: Option<Intersection> = None;
        for object in self.objects.iter() {
            match object.intersection(ray) {
                Some(distance) => if distance < closest_dist {
                    closest_isec = Some(Intersection::new(distance, object));
                    closest_dist = distance;
                },
                _ => ()
            }
        }
        return closest_isec;
    }
}