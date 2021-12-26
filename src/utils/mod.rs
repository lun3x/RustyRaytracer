use std::ops::{Sub, Mul};

use cgmath::InnerSpace;
use rand::Rng;
use crate::constants::*;

const EPSILON: f32 = 0.0005;

pub fn hamming_distance(num1: Vector, num2: Vector) -> f32 {
    (num1.x - num2.x).abs() +
    (num1.y - num2.y).abs() +
    (num1.z - num2.z).abs()
}

// All functions optimistically return true
pub fn is_eq(num1: f32, num2: f32) -> bool {
    (num1 - num2).abs() < EPSILON
}

pub fn is_eq_vec(vec1: Vector, vec2: Vector) -> bool {
    hamming_distance(vec1, vec2) < EPSILON
}

pub fn is_zero(num1: f32) -> bool {
    num1.abs() < EPSILON
}

pub fn is_negative(num1: f32) -> bool {
    num1 < -EPSILON
}

#[allow(dead_code)]
pub fn is_positive(num1: f32) -> bool {
    num1 > EPSILON
}

#[allow(dead_code)]
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

#[allow(dead_code)]
pub fn rand_f32_range(min: f32, max: f32) -> f32 {
    rand::thread_rng().gen_range(min..max)
}

#[allow(dead_code)]
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

pub fn clamp(val: f32, min: f32, max: f32) -> f32 {
    if val < min {
        return min;
    }
    if val > max {
        return max;
    }
    return val;
}

pub fn clamp_colour(colour: ColourFloat) -> ColourFloat {
    ColourFloat::new(
        clamp(colour[0], 0.0, 0.999),
        clamp(colour[1], 0.0, 0.999),
        clamp(colour[2], 0.0, 0.999))
}

pub fn as_int(colour: ColourFloat) -> Colour {
    [colour[0] as u8, colour[1] as u8, colour[2] as u8]
}

pub fn gamma_correct(colour: ColourFloat) -> ColourFloat {
    ColourFloat::new(colour[0].sqrt(), colour[1].sqrt(), colour[2].sqrt())
}

pub fn correct_for_output(colour: ColourFloat) -> Colour {
    as_int(256.0 * clamp_colour(gamma_correct(colour)))
}