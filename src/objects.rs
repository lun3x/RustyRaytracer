use crate::intersect::*;
use crate::visualiser::*;
use cgmath::prelude::*;

pub enum Material {
    Specular,
    Diffuse,
}

pub enum Object {
    Triangle(Triangle),
    Sphere(Sphere),
}

pub struct Triangle {
    pub v0: Point,
    pub v1: Point,
    pub v2: Point,
    pub colours: [Colour; 3],
    pub material: Material,
    normal: Vector,
}

impl Triangle {
    pub fn new(v0: Point, v1: Point, v2: Point, colours: [Colour; 3], material: Material) -> Self {
        Triangle {
            v0,
            v1,
            v2,
            colours,
            material,
            normal: compute_normal(v0, v1, v2),
        }
    }
}

fn compute_normal(v0: Point, v1: Point, v2: Point) -> Vector {
    let e1 = Vector::new(v1.x - v0.x, v1.y - v0.y, v1.z - v0.z);
    let e2 = Vector::new(v2.x - v0.x, v2.y - v0.y, v2.z - v0.z);
    e2.cross(e1).normalize()
}

pub struct Sphere {
    pub centre: Point,
    pub radius: f32,
    pub colour: Colour,
    pub material: Material,
}

pub trait Coloured {
    fn get_colour(&self, texture_coords: BarycentricCoords) -> Colour;
}

fn as_float(colour: Colour) -> ColourFloat {
    ColourFloat::new(colour[0] as f32, colour[1] as f32, colour[2] as f32)
}

fn as_int(colour: ColourFloat) -> Colour {
    [colour[0] as u8, colour[1] as u8, colour[2] as u8]
}

impl Coloured for Triangle {
    fn get_colour(&self, texture_coords: BarycentricCoords) -> Colour {
        as_int(
            texture_coords.u * as_float(self.colours[0])
                + texture_coords.v * as_float(self.colours[1])
                + texture_coords.w * as_float(self.colours[2]),
        )
    }
}

impl Coloured for Sphere {
    fn get_colour(&self, _texture_coords: BarycentricCoords) -> Colour {
        self.colour
    }
}

impl Coloured for Object {
    fn get_colour(&self, texture_coords: BarycentricCoords) -> Colour {
        use Object::*;
        match *self {
            Triangle(ref t) => t.get_colour(texture_coords),
            Sphere(ref s) => s.get_colour(texture_coords),
        }
    }
}

impl Object {
    pub fn get_material(&self) -> &Material {
        use Object::*;
        match *self {
            Triangle(ref t) => t.get_material(),
            Sphere(ref s) => s.get_material(),
        }
    }

    pub fn get_normal(&self, location: Point) -> Vector {
        use Object::*;
        match *self {
            Triangle(ref t) => t.get_normal(location),
            Sphere(ref s) => s.get_normal(location),
        }
    }
}

impl Sphere {
    pub fn get_material(&self) -> &Material {
        &self.material
    }

    pub fn get_normal(&self, location: Point) -> Vector {
        (location - self.centre) / self.radius
    }
}

impl Triangle {
    pub fn get_material(&self) -> &Material {
        &self.material
    }

    pub fn get_normal(&self, _location: Point) -> Vector {
        self.normal
    }
}
