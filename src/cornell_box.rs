use crate::objects::*;
use crate::scene::*;
use crate::visualiser::*;

const RED: [u8; 3] = [255, 0, 0];
const GREEN: [u8; 3] = [0, 255, 0];
const BLUE: [u8; 3] = [0, 0, 255];

pub fn get_scene2() -> Scene {
    let p0 = Point {
        x: -10.0,
        y: -10.0,
        z: 0.0,
    };
    let p1 = Point {
        x: -10.0,
        y: -10.0,
        z: -10.0,
    };
    let p2 = Point {
        x: 10.0,
        y: -10.0,
        z: 0.0,
    };
    let p3 = Point {
        x: 10.0,
        y: -10.0,
        z: -10.0,
    };
    let p4 = Point {
        x: 10.0,
        y: 10.0,
        z: 0.0,
    };
    let p5 = Point {
        x: 10.0,
        y: 10.0,
        z: -10.0,
    };
    let p6 = Point {
        x: -10.0,
        y: 10.0,
        z: 0.0,
    };
    let p7 = Point {
        x: -10.0,
        y: 10.0,
        z: -10.0,
    };

    // let left1 = Object::Triangle(Triangle::new(
    //     p0,
    //     p1,
    //     p2,
    //     [GREEN; 3],
    //     Material::Diffuse,
    // ));
    // let left2 = Object::Triangle(Triangle::new(
    //     p0,
    //     p1,
    //     p2,
    //     [GREEN; 3],
    //     Material::Diffuse,
    // ));
    
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
    Scene {
        objects: vec![sphere0, sphere1, sphere2, sphere3, sphere4, sphere5, sphere6, sphere7],
    }
}

pub fn get_scene() -> Scene {
    let p1 = Point {
        x: -4.0,
        y: 2.0,
        z: -13.0,
    };
    let p2 = Point {
        x: -2.0,
        y: -2.0,
        z: -7.0,
    };
    let p3 = Point {
        x: 2.0,
        y: 0.0,
        z: -11.0,
    };
    let sphere1 = Object::Sphere(Sphere {
        centre: p1,
        radius: 1.0,
        colour: RED,
        material: Material::Diffuse,
    });
    let sphere2 = Object::Sphere(Sphere {
        centre: p2,
        radius: 1.0,
        colour: GREEN,
        material: Material::Diffuse,
    });
    let sphere3 = Object::Sphere(Sphere {
        centre: p3,
        radius: 1.0,
        colour: BLUE,
        material: Material::Diffuse,
    });
    let v0 = Point {
        x: p1.x,
        y: p1.y,
        z: p1.z + 2.0,
    };
    let v1 = Point {
        x: p2.x,
        y: p2.y,
        z: p2.z + 2.0,
    };
    let v2 = Point {
        x: p3.x,
        y: p3.y,
        z: p3.z + 2.0,
    };
    let triangle1 = Object::Triangle(Triangle::new(
        p1,
        p2,
        p3,
        [GREEN, BLUE, RED],
        Material::Diffuse,
    ));
    Scene {
        objects: vec![sphere1, sphere2, sphere3, triangle1],
    }
}
