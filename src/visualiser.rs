use crate::rays::*;
use crate::utils;
use cgmath::prelude::*;
use image::{Rgb, RgbImage};

pub type Colour = [u8; 3];
pub type ColourFloat = cgmath::Vector3<f32>;
pub type Point = cgmath::Vector3<f32>;
pub type Vector = cgmath::Vector3<f32>;
pub type RotationMatrix = cgmath::Matrix4<f32>;
pub type Degrees = cgmath::Deg<f32>;

pub struct Camera {
    pub location: Point,
    pub focal_length: f32,
    pub yaw: Degrees,
    pub rotation_matrix: RotationMatrix,
}

impl Camera {
    pub fn new(location: Point, focal_length: f32, yaw: cgmath::Deg<f32>) -> Self {
        let rotation_matrix = RotationMatrix::from_angle_y(yaw);
        let location = utils::four_to_three(rotation_matrix * utils::three_to_four(location));
        Camera {
            location,
            focal_length,
            yaw,
            rotation_matrix,
        }
    }

    pub fn rotate(&self, yaw: f32) -> Self {
        let yaw = cgmath::Deg(yaw);
        Self::new(self.location, self.focal_length, self.yaw + yaw)
    }
}

pub struct Visualiser {
    pub screen: RgbImage,
    pub aspect_ratio: f32,
    pub camera: Camera,
}

impl Visualiser {
    pub fn new(height: u32, width: u32, camera: Camera) -> Self {
        Visualiser {
            screen: RgbImage::new(width, height),
            aspect_ratio: width as f32 / height as f32,
            camera,
        }
    }

    pub fn rotate(&mut self, yaw: f32) {
        self.camera = self.camera.rotate(yaw);
    }

    pub fn create_camera_ray(&self, x: u32, y: u32) -> Ray {
        let x_screen = ((x as f32 + 0.5) / self.screen.width() as f32) * 2.0 - 1.0;
        let y_screen = 1.0 - ((y as f32 + 0.5) / self.screen.height() as f32) * 2.0;

        Ray {
            start: self.camera.location,
            dir: Vector {
                x: x_screen,
                y: y_screen,
                z: -self.camera.focal_length,
            }
            .normalize(),
        }
        .rotate(self.camera.rotation_matrix)
    }

    pub fn put_pixel(&mut self, x: u32, y: u32, colour: Colour) {
        self.screen.put_pixel(x, y, Rgb(colour));
    }

    pub fn save(&self) {
        let out_file = "render.png";
        match self.screen.save(out_file) {
            Ok(_) => println!("Saved {} successfully", out_file),
            Err(_) => panic!("Problem saving {}!", out_file),
        }
    }
}
