use cgmath::prelude::*;

use crate::utils;
use crate::constants::*;

pub struct Scene {
    pub objects: Vec<Object>,
}

impl Scene {
    pub fn closest_intersection(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest_dist = f32::MAX;
        let mut closest_isect: Option<Intersection> = None;
        for object in self.objects.iter() {
            match object.intersection(ray) {
                Some(location) => {
                    if location.distance < closest_dist {
                        closest_dist = location.distance;
                        closest_isect = Some(Intersection::new(location, object));
                    }
                }
                None => (),
            }
        }
        closest_isect
    }
}

pub struct Ray {
    pub start: Point,
    pub dir: Vector,
}

pub fn rotate(dir: &Vector, rotation_matrix: &RotationMatrix) -> Vector {
    let dir4 = utils::to_4(dir);
    let rotated_dir = rotation_matrix * dir4;
    utils::to_3(&rotated_dir)
}

pub fn reflect(dir: &Vector, normal: &Vector) -> Vector {
    (dir - (2.0 * dir.dot(*normal) * normal)).normalize()
}

pub fn diffuse(normal: &Vector) -> Vector {
    loop {
        let p = utils::rand_vector_range(-1.0, 1.0);
        if p.magnitude2() < 1.0 {
            return (normal + p).normalize();
        }
    }
}

pub fn trace(ray: Ray, scene: &Scene, depth: u32) -> ColourFloat {
    match scene.closest_intersection(&ray) {
        Some(i) => {
            let isect_position: Point = i.location.distance * ray.dir;
            use Material::*;
            match *i.object.get_material() {
                Specular => {
                    if depth > 0 {
                        let normal = i.object.get_normal(isect_position);
                        let reflected_ray = Ray {
                            start: isect_position + (normal * 0.005),
                            dir: reflect(&ray.dir, &normal),
                        };
                        0.9 * trace(reflected_ray, scene, depth - 1)
                    } else {
                        i.object.get_colour(i.location.texture_coords)
                    }
                }
                Diffuse => i.object.get_colour(i.location.texture_coords),
                Lambertian => {
                    if depth > 0 {
                        let normal = i.object.get_normal(isect_position);
                        let diffuse_ray = Ray {
                            start: isect_position + (normal * 0.005),
                            dir: diffuse(&normal),
                        };
                        0.5 * trace(diffuse_ray, scene, depth - 1)
                    } else {
                        BLACK
                    }
                }
            }
        }
        None => BACKGROUND_TOP.lerp(BACKGROUND_BOTTOM, 0.5*(ray.dir.y+1.0)),
    }
}

pub struct Camera {
    pub origin: Point,
    pub horizontal_dir: Vector,
    pub vertical_dir: Vector,
    pub lower_left_corner: Vector,
    pub yaw: Degrees,
    pub rotation_matrix: RotationMatrix,
}

impl Camera {
    pub fn new(origin: Point, focal_length: f32, aspect_ratio: f32, yaw: Degrees) -> Self {
        let rotation_matrix = RotationMatrix::from_angle_y(yaw);
        // let origin = utils::to_3(&(rotation_matrix * utils::to_4(&origin)));

        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        
        let horizontal_dir = Vector::new(viewport_width, 0.0, 0.0);
        let vertical_dir = Vector::new(0.0, viewport_height, 0.0);
        let lower_left_corner = origin - horizontal_dir/2.0 - vertical_dir/2.0 - Vector::new(0.0, 0.0, focal_length);

        println!("Create camera at {:?} rotated by {:?}", origin, yaw);

        Camera {
            origin,
            horizontal_dir,
            vertical_dir,
            lower_left_corner,
            yaw,
            rotation_matrix,
        }
    }

    pub fn create_camera_ray(&self, u: f32, v: f32) -> Ray {
        let ray_dir = self.lower_left_corner + u*self.horizontal_dir + v*self.vertical_dir - self.origin;

        Ray {
            start: self.origin,
            dir: ray_dir.normalize(),
        }
    }

    pub fn rotate(&mut self, yaw: Degrees) {
        println!(
            "Rotate camera by {:?} from {:?} to {:?}",
            yaw,
            self.yaw,
            self.yaw + yaw
        );
        self.yaw = (self.yaw + yaw) % cgmath::Deg(360.0);
        self.rotation_matrix = RotationMatrix::from_angle_y(self.yaw);
        let location_rotation = RotationMatrix::from_angle_y(yaw);
        self.origin = utils::to_3(&(location_rotation * utils::to_4(&self.origin)));
    }

    pub fn dolly(&mut self, distance: f32) {
        println!("Dolly camera in {}m from ({:?}, {:?})", distance, self.origin.x, self.origin.z);
        self.origin.x -= distance * self.yaw.sin();
        self.origin.z -= distance * self.yaw.cos();
    }
}

#[derive(Clone, Copy)]
pub enum Material {
    Specular,
    Diffuse,
    Lambertian,
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
    pub fn new(
        v0: Point,
        v1: Point,
        v2: Point,
        colours: [ColourFloat; 3],
        material: Material,
    ) -> Self {
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
    fn get_colour(&self, texture_coords: TextureCoords) -> ColourFloat;
}

impl Coloured for Triangle {
    fn get_colour(&self, texture_coords: TextureCoords) -> ColourFloat {
        use TextureCoords::*;
        match texture_coords {
            Barycentric(coords) => {
                coords.u * self.colours[0] + coords.v * self.colours[1] + coords.w * self.colours[2]
            }
            None => panic!("Incorrect texture coord type specified for triangle."),
        }
    }
}

impl Coloured for Sphere {
    fn get_colour(&self, texture_coords: TextureCoords) -> ColourFloat {
        use TextureCoords::*;
        match texture_coords {
            Barycentric(_) => panic!("Incorrect texture coord type specified for sphere."),
            None => self.colour,
        }
    }
}

impl Coloured for Object {
    fn get_colour(&self, texture_coords: TextureCoords) -> ColourFloat {
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

pub struct BarycentricCoords {
    pub u: f32,
    pub v: f32,
    pub w: f32,
}

impl BarycentricCoords {
    pub fn new(u: f32, v: f32) -> Self {
        Self {
            u,
            v,
            w: 1.0 - u - v,
        }
    }
}

pub enum TextureCoords {
    Barycentric(BarycentricCoords),
    None,
}

pub struct IntersectionLocation {
    pub distance: f32,
    pub texture_coords: TextureCoords,
}

pub struct Intersection<'a> {
    pub location: IntersectionLocation,
    pub object: &'a Object,
}

impl<'a> Intersection<'a> {
    pub fn new(location: IntersectionLocation, object: &'a Object) -> Self {
        Intersection { location, object }
    }
}

pub trait Intersectable {
    fn intersection(&self, ray: &Ray) -> Option<IntersectionLocation>;
}

impl Intersectable for Object {
    fn intersection(&self, ray: &Ray) -> Option<IntersectionLocation> {
        match *self {
            Object::Sphere(ref s) => s.intersection(ray),
            Object::Triangle(ref t) => t.intersection(ray),
        }
    }
}

impl Intersectable for Sphere {
    fn intersection(&self, ray: &Ray) -> Option<IntersectionLocation> {
        let to_centre: Vector = self.centre - ray.start;
        // Right angle triangle side lengths, with to_centre as the hypotenuse
        let adjacent: f32 = ray.dir.dot(to_centre);
        let opposite_squared: f32 = to_centre.dot(to_centre) - (adjacent * adjacent);
        let radius_squared: f32 = self.radius * self.radius;

        // No intersect if opposite is greater than radius^2
        if utils::is_greater_than(opposite_squared, radius_squared) {
            return None;
        }

        // Length of chord (section of ray that is intersecting)
        let half_chord: f32 = (radius_squared - opposite_squared).sqrt();
        // Intersection distances
        let isect0 = adjacent - half_chord;
        let isect1 = adjacent + half_chord;

        // Don't return intersections behind camera
        if isect0.is_sign_negative() && isect1.is_sign_negative() {
            return None;
        }

        // Avoid shadow acne
        let distance = if isect0 < isect1 { isect0 } else { isect1 };
        if distance < 0.001 {
            return None;
        }

        let loc = IntersectionLocation {
            distance,
            texture_coords: TextureCoords::None,
        };

        Some(loc)
    }
}

impl Intersectable for Triangle {
    fn intersection(&self, ray: &Ray) -> Option<IntersectionLocation> {
        // Moller-Trumbore intersection algorithm

        let v0v1: Vector = self.v1 - self.v0;
        let v0v2: Vector = self.v2 - self.v0;

        let pvec: Vector = ray.dir.cross(v0v2);
        let determinant: f32 = v0v1.dot(pvec);

        // cull backfacing triangles
        if utils::is_negative(determinant) {
            return None;
        }
        // avoid parallel rays
        if utils::is_zero(determinant) {
            return None;
        }

        // compute 'u' barycentric coord
        let inv_det: f32 = 1.0 / determinant;
        let tvec: Vector = ray.start - self.v0;
        let u: f32 = tvec.dot(pvec) * inv_det;
        if utils::is_negative(u) || utils::is_greater_than(u, 1.0) {
            return None;
        }

        // compute 'v' barycentric coord
        let qvec: Vector = tvec.cross(v0v1);
        let v: f32 = ray.dir.dot(qvec) * inv_det;
        if utils::is_negative(v) || utils::is_greater_than(u + v, 1.0) {
            return None;
        }

        let distance: f32 = v0v2.dot(qvec) * inv_det;

        let loc = IntersectionLocation {
            distance,
            texture_coords: TextureCoords::Barycentric(BarycentricCoords::new(u, v)),
        };

        Some(loc)
    }
}
