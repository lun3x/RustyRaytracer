use crate::objects::*;
use crate::scene::*;
use crate::visualiser::*;

const RED: ColourFloat = ColourFloat::new(255., 0., 0.);
const GREEN: ColourFloat = ColourFloat::new(0., 255., 0.);
const BLUE: ColourFloat = ColourFloat::new(0., 0., 255.);
const WHITE: ColourFloat = ColourFloat::new(255., 255., 255.);
const PURPLE: ColourFloat = ColourFloat::new(255., 0., 255.);

pub fn get_scene2() -> Scene {
    // Vertices
    let p0 = Point {
        x: -10.0,
        y: -10.0,
        z: 10.0,
    };
    let p1 = Point {
        x: -10.0,
        y: -10.0,
        z: -10.0,
    };
    let p2 = Point {
        x: 10.0,
        y: -10.0,
        z: 10.0,
    };
    let p3 = Point {
        x: 10.0,
        y: -10.0,
        z: -10.0,
    };
    let p4 = Point {
        x: 10.0,
        y: 10.0,
        z: 10.0,
    };
    let p5 = Point {
        x: 10.0,
        y: 10.0,
        z: -10.0,
    };
    let p6 = Point {
        x: -10.0,
        y: 10.0,
        z: 10.0,
    };
    let p7 = Point {
        x: -10.0,
        y: 10.0,
        z: -10.0,
    };

    // Walls
    let bottom1 = Object::Triangle(Triangle::new(p2, p1, p0, [GREEN; 3], Material::Diffuse));
    let bottom2 = Object::Triangle(Triangle::new(p2, p3, p1, [BLUE; 3], Material::Diffuse));
    let left1 = Object::Triangle(Triangle::new(p0, p1, p6, [RED; 3], Material::Diffuse));
    let left2 = Object::Triangle(Triangle::new(p7, p6, p1, [RED; 3], Material::Diffuse));
    let back1 = Object::Triangle(Triangle::new(p7, p1, p3, [GREEN; 3], Material::Specular));
    let back2 = Object::Triangle(Triangle::new(p3, p5, p7, [GREEN; 3], Material::Specular));
    let right1 = Object::Triangle(Triangle::new(p5, p3, p4, [WHITE; 3], Material::Diffuse));
    let right2 = Object::Triangle(Triangle::new(p3, p2, p4, [WHITE; 3], Material::Diffuse));
    let top1 = Object::Triangle(Triangle::new(p6, p7, p4, [PURPLE; 3], Material::Diffuse));
    let top2 = Object::Triangle(Triangle::new(p7, p5, p4, [PURPLE; 3], Material::Diffuse));
    let front1 = Object::Triangle(Triangle::new(
        p6,
        p2,
        p0,
        [RED, GREEN, BLUE],
        Material::Diffuse,
    ));
    let front2 = Object::Triangle(Triangle::new(
        p6,
        p4,
        p2,
        [RED, GREEN, BLUE],
        Material::Specular,
    ));

    let sphere0 = Object::Sphere(Sphere {
        centre: p0,
        radius: 1.0,
        colour: RED,
        material: Material::Diffuse,
    });
    let sphere1 = Object::Sphere(Sphere {
        centre: p1,
        radius: 1.0,
        colour: GREEN,
        material: Material::Diffuse,
    });
    let sphere2 = Object::Sphere(Sphere {
        centre: p2,
        radius: 1.0,
        colour: RED,
        material: Material::Diffuse,
    });
    let sphere3 = Object::Sphere(Sphere {
        centre: p3,
        radius: 1.0,
        colour: GREEN,
        material: Material::Diffuse,
    });
    let sphere4 = Object::Sphere(Sphere {
        centre: p4,
        radius: 1.0,
        colour: RED,
        material: Material::Diffuse,
    });
    let sphere5 = Object::Sphere(Sphere {
        centre: p5,
        radius: 1.0,
        colour: GREEN,
        material: Material::Diffuse,
    });
    let sphere6 = Object::Sphere(Sphere {
        centre: p6,
        radius: 1.0,
        colour: RED,
        material: Material::Diffuse,
    });
    let sphere7 = Object::Sphere(Sphere {
        centre: p7,
        radius: 1.0,
        colour: GREEN,
        material: Material::Diffuse,
    });

    // Ornaments
    let p_centre = Point {
        x: 0.0,
        y: 0.0,
        z: -5.0,
    };
    let sphere_centre = Object::Sphere(Sphere {
        centre: p_centre,
        radius: 1.0,
        colour: BLUE,
        material: Material::Specular,
    });
    let p_mid_centre = Point {
        x: -5.0,
        y: -5.0,
        z: -1.0,
    };
    let sphere_mid_centre = Object::Sphere(Sphere {
        centre: p_mid_centre,
        radius: 2.0,
        colour: PURPLE,
        material: Material::Specular,
    });
    let p_back_centre = Point {
        x: 7.0,
        y: -7.0,
        z: -7.0,
    };
    let sphere_back_centre = Object::Sphere(Sphere {
        centre: p_back_centre,
        radius: 2.0,
        colour: RED,
        material: Material::Specular,
    });

    let objects = vec![
        sphere0,
        sphere1,
        sphere2,
        sphere3,
        sphere4,
        sphere5,
        sphere6,
        sphere7,
        bottom1,
        bottom2,
        left1,
        left2,
        back1,
        back2,
        right1,
        right2,
        top1,
        top2,
        sphere_centre,
        sphere_mid_centre,
        sphere_back_centre,
        front1,
        front2,
    ];
    Scene { objects }
}

// pub fn get_scene() -> Scene {
//     let p1 = Point {
//         x: -4.0,
//         y: 2.0,
//         z: -13.0,
//     };
//     let p2 = Point {
//         x: -2.0,
//         y: -2.0,
//         z: -7.0,
//     };
//     let p3 = Point {
//         x: 2.0,
//         y: 0.0,
//         z: -11.0,
//     };
//     let sphere1 = Object::Sphere(Sphere {
//         centre: p1,
//         radius: 1.0,
//         colour: RED,
//         material: Material::Diffuse,
//     });
//     let sphere2 = Object::Sphere(Sphere {
//         centre: p2,
//         radius: 1.0,
//         colour: GREEN,
//         material: Material::Diffuse,
//     });
//     let sphere3 = Object::Sphere(Sphere {
//         centre: p3,
//         radius: 1.0,
//         colour: BLUE,
//         material: Material::Diffuse,
//     });
//     let v0 = Point {
//         x: p1.x,
//         y: p1.y,
//         z: p1.z + 2.0,
//     };
//     let v1 = Point {
//         x: p2.x,
//         y: p2.y,
//         z: p2.z + 2.0,
//     };
//     let v2 = Point {
//         x: p3.x,
//         y: p3.y,
//         z: p3.z + 2.0,
//     };
//     let triangle1 = Object::Triangle(Triangle::new(
//         p1,
//         p2,
//         p3,
//         [GREEN, BLUE, RED],
//         Material::Diffuse,
//     ));
//     Scene {
//         objects: vec![sphere1, sphere2, sphere3, triangle1],
//     }
// }
