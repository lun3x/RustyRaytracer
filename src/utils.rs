use crate::raytracing::*;
use rand::Rng;

const EPSILON: f32 = 0.000005;

// All functions optimistically return true
pub fn is_eq(num1: f32, num2: f32) -> bool {
    (num1 - num2).abs() < EPSILON
}

pub fn is_zero(num1: f32) -> bool {
    num1.abs() < EPSILON
}

pub fn is_negative(num1: f32) -> bool {
    num1 < -EPSILON
}

pub fn is_positive(num1: f32) -> bool {
    num1 > EPSILON
}

pub fn is_less_than(num1: f32, num2: f32) -> bool {
    num1 < num2 - EPSILON
}

pub fn is_greater_than(num1: f32, num2: f32) -> bool {
    num1 > num2 + EPSILON
}

pub fn to_3(vec4: &cgmath::Vector4<f32>) -> cgmath::Vector3<f32> {
    cgmath::Vector3::new(vec4.x, vec4.y, vec4.z)
}

pub fn to_4(vec3: &cgmath::Vector3<f32>) -> cgmath::Vector4<f32> {
    cgmath::Vector4::new(vec3.x, vec3.y, vec3.z, 1.0)
}

pub fn rand_f32() -> f32 {
    rand::thread_rng().gen::<f32>()
}

pub fn rand_f32_range(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min..max)
}

pub fn rand_vector() -> Vector {
    let mut rng = rand::thread_rng();
    Vector::new(rng.gen::<f32>(), rng.gen::<f32>(), rng.gen::<f32>())
}

pub fn rand_vector_range(min: f32, max: f32) -> Vector {
    let mut rng = rand::thread_rng();
    Vector::new(
        rng.gen_range(min..max),
        rng.gen_range(min..max),
        rng.gen_range(min..max),
    )
}
