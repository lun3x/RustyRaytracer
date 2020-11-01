use crate::intersect::*;
use crate::visualiser::*;
use cgmath::prelude::*;

#[derive(Clone, Copy)]
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
    pub colours: [ColourFloat; 3],
    pub material: Material,
    normal: Vector,
}

impl Triangle {
    pub fn new(v0: Point, v1: Point, v2: Point, colours: [ColourFloat; 3], material: Material) -> Self {
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
    let v1v0 = v1 - v0;
    let v2v0 = v2 - v0;
    v2v0.cross(v1v0).normalize()
}

pub struct Sphere {
    pub centre: Point,
    pub radius: f32,
    pub colour: ColourFloat,
    pub material: Material,
}

pub trait Coloured {
    fn get_colour(&self, texture_coords: BarycentricCoords) -> ColourFloat;
}

pub fn as_float(colour: Colour) -> ColourFloat {
    ColourFloat::new(colour[0] as f32, colour[1] as f32, colour[2] as f32)
}

pub fn as_int(colour: ColourFloat) -> Colour {
    [colour[0] as u8, colour[1] as u8, colour[2] as u8]
}

impl Coloured for Triangle {
    fn get_colour(&self, texture_coords: BarycentricCoords) -> ColourFloat {
        texture_coords.u * self.colours[0]
        + texture_coords.v * self.colours[1]
        + texture_coords.w * self.colours[2]
    }
}

impl Coloured for Sphere {
    fn get_colour(&self, _texture_coords: BarycentricCoords) -> ColourFloat {
        self.colour
    }
}

impl Coloured for Object {
    fn get_colour(&self, texture_coords: BarycentricCoords) -> ColourFloat {
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
        ((location - self.centre) / self.radius).normalize()
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
