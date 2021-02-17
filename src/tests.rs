use super::*;
#[test]
fn test_reflect_straight() {
    // Straight on
    let start = Point::new(0.0, 0.0, 0.0);
    let dir = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, 1.0);
    let ray = rays::Ray { start, dir };
    assert_eq!(ray.reflect(&normal), Vector::new(0.0, 0.0, 1.0));

    // 45 degree incident ray
    let start = Point::new(0.0, 0.0, 0.0);
    let dir = Vector::new(1.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, 1.0);
    let ray = rays::Ray { start, dir };
    assert_eq!(ray.reflect(&normal), Vector::new(1.0, 0.0, 1.0));

    // 45 degree normal
    let start = Point::new(0.0, 0.0, 0.0);
    let dir = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(1.0, 0.0, 1.0);
    let ray = rays::Ray { start, dir };
    assert_eq!(ray.reflect(&normal), Vector::new(1.0, 0.0, 0.0));
}