use super::*;
#[test]
fn test_reflect_straight() {
    // Straight on
    let start = Point::new(0.0, 0.0, 0.0);
    let dir = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, 1.0);
    let ray = Ray { start, dir };
    assert_eq!(reflect(&ray.dir, &normal), Vector::new(0.0, 0.0, 1.0));

    // 45 degree incident ray
    let start = Point::new(0.0, 0.0, 0.0);
    let dir = Vector::new(1.0, 0.0, -1.0);
    let normal = Vector::new(0.0, 0.0, 1.0);
    let ray = Ray { start, dir };
    assert_eq!(reflect(&ray.dir, &normal), Vector::new(1.0, 0.0, 1.0));

    // 45 degree normal
    let start = Point::new(0.0, 0.0, 0.0);
    let dir = Vector::new(0.0, 0.0, -1.0);
    let normal = Vector::new(1.0, 0.0, 1.0);
    let ray = Ray { start, dir };
    assert_eq!(reflect(&ray.dir, &normal), Vector::new(1.0, 0.0, 0.0));
}
