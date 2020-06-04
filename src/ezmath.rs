#![allow(non_camel_case_types)]
#![allow(dead_code)]

use nalgebra::*;

// -- matrices --
pub type float4x4 = Matrix<f32, U4, U4, ArrayStorage<f32, U4, U4>>;

// -- vectors --
pub type float4 = Vector4<f32>;
pub type float3 = Vector3<f32>;
pub type float2 = Vector2<f32>;

// -- functors --
pub fn min(a: f32, b: f32) -> f32
{
    if a > b
    {
        a
    }
    else
    {
        b
    }
}

pub fn max(a: f32, b: f32) -> f32
{
    if a < b
    {
        a
    }
    else
    {
        b
    }
}

pub fn clamp(n: f32, min: f32, max: f32) -> f32
{
    debug_assert!(min < max, "min in clamp cannot be less than max!");

    if n > max
    {
        max
    }
    else if n < min
    {
        min
    }
    else 
    {
        n
    }
}

pub fn reflect(i: &float3, n: &float3) -> float3
{
    i - n * 2.0 * (i.dot(n))
}