use cgmath::InnerSpace;

use super::raytracing::*;
use super::constants::*;
use super::utils;

#[test]
fn test_reflect_straight() {
    // Straight on
    let start = Point::new(0.0, 0.0, 0.0);
    let dir = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, 1.0);
    let ray = Ray { start, dir };
    let reflected_ray = reflect(&ray.dir, &normal);

    assert!(utils::is_eq_vec(reflected_ray, Vector::new(0.0, 0.0, 1.0)), "{:?} != {:?}", reflected_ray, Vector::new(1.0, 0.0, 0.0));

    // 45 degree incident ray
    let start = Point::new(0.0, 0.0, 0.0);
    let dir = Vector::new(1.0, 0.0, -1.0).normalize();
    let normal = Vector::new(0.0, 0.0, 1.0);
    let ray = Ray { start, dir };
    let reflected_ray = reflect(&ray.dir, &normal);

    assert!(utils::is_eq_vec(reflected_ray, Vector::new(1.0, 0.0, 1.0).normalize()), "{:?} != {:?}", reflected_ray, Vector::new(1.0, 0.0, 0.0));

    // 45 degree normal
    let start = Point::new(0.0, 0.0, 0.0);
    let dir = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(1.0, 0.0, -1.0).normalize();
    let ray = Ray { start, dir };
    let reflected_ray = reflect(&ray.dir, &normal);
    // assert!(utils::is_eq_vec(reflected_ray, Vector::new(1.0, 0.0, 0.0)), "{:?} != {:?}", reflected_ray, Vector::new(1.0, 0.0, 0.0));
}
