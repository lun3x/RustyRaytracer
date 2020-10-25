use cgmath::prelude::*;
use crate::visualiser::*;

pub enum Object {
    Triangle(Triangle),
    Sphere(Sphere),
}

pub struct Triangle {
    pub v0: Point,
    pub v1: Point,
    pub v2: Point,
    pub colour: Colour,
    normal: Vector,
}

impl Triangle {
    pub fn new(v0: Point, v1: Point, v2: Point, colour: Colour) -> Self {
        Triangle {
            v0,
            v1,
            v2,
            colour,
            normal: compute_normal(v0, v1, v2),
        }
    }
}

fn compute_normal(v0: Point, v1: Point, v2: Point) -> Vector {
    let e1 = Vector::new(v1.x-v0.x, v1.y-v0.y, v1.z-v0.z);
    let e2 = Vector::new(v2.x-v0.x, v2.y-v0.y, v2.z-v0.z);
    e2.cross(e1).normalize()
}

pub struct Sphere {
    pub centre: Point,
    pub radius: f32,
    pub colour: Colour,
}

pub trait Coloured {
    fn get_colour(&self) -> Colour;
}

impl Coloured for Sphere {
    fn get_colour(&self) -> Colour {
        self.colour
    }
}

impl Coloured for Triangle {
    fn get_colour(&self) -> Colour {
        self.colour
    }
}

impl Coloured for Object {
    fn get_colour(&self) -> Colour {
        use Object::*;
        match *self {
            Triangle(ref t) => t.get_colour(),
            Sphere(ref s) => s.get_colour(),
        }
    }
}



