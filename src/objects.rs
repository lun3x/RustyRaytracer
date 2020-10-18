use crate::visualiser::*;

pub enum Object {
    Triangle(Triangle),
    Sphere(Sphere),
}

pub struct Triangle {
    pub v0: Point,
    pub v1: Point,
    pub v2: Point,
    pub normal: Vector,
}

pub struct Sphere {
    pub centre: Point,
    pub radius: f32,
}



