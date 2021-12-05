pub const SCREEN_HEIGHT: usize = 200;
pub const SCREEN_WIDTH: usize = SCREEN_HEIGHT * 16 / 9;
pub const ASPECT_RATIO: f32 = SCREEN_WIDTH as f32 / SCREEN_HEIGHT as f32;

pub const ANTIALIAS_SAMPLES: u32 = 1;
pub const REFLECT_DEPTH: u32 = 5;

pub type Colour = [u8; 3];
pub type ColourFloat = cgmath::Vector3<f32>;
pub type Point = cgmath::Vector3<f32>;
pub type Vector = cgmath::Vector3<f32>;
pub type RotationMatrix = cgmath::Matrix4<f32>;
pub type Degrees = cgmath::Deg<f32>;

pub const BACKGROUND_BOTTOM: ColourFloat = ColourFloat::new(1.0, 1.0, 1.0);
pub const BACKGROUND_TOP: ColourFloat = ColourFloat::new(0.5, 0.7, 1.0);
pub const RED: ColourFloat     = ColourFloat::new(1.0, 0.0, 0.0);
pub const GREEN: ColourFloat   = ColourFloat::new(0.0, 1.0, 0.0);
pub const BLUE: ColourFloat    = ColourFloat::new(0.0, 0.0, 1.0);
pub const WHITE: ColourFloat   = ColourFloat::new(1.0, 1.0, 1.0);
pub const PURPLE: ColourFloat  = ColourFloat::new(1.0, 0.0, 1.0);
pub const BLACK: ColourFloat   = ColourFloat::new(0.0, 0.0, 0.0);