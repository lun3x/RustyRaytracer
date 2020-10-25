use crate::intersect::*;
use crate::objects::*;
use crate::visualiser::*;

pub struct Scene {
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn closest_intersection(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest_dist = f32::MAX;
        let mut closest_isect: Option<Intersection> = None;
        for object in self.objects.iter() {
            match object.intersection(ray) {
                Some(location) => {
                    if location.distance < closest_dist {
                        closest_dist = location.distance;
                        closest_isect = Some(Intersection::new(location, object));
                    }
                }
                _ => (),
            }
        }
        closest_isect
    }
}
