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
    pub colour: Colour,
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



