#![allow(non_camel_case_types)]
#![allow(dead_code)]

use nalgebra::*;

// -- matrices --
pub type float4x4 = Matrix<f32, U4, U4, ArrayStorage<f32, U4, U4>>;
pub type int4x4 = Matrix<i32, U4, U4, ArrayStorage<i32, U4, U4>>;

// -- vectors --
pub type float4 = Vector4<f32>;
pub type float3 = Vector3<f32>;
pub type float2 = Vector2<f32>;

pub type int4 = Vector4<i32>;
pub type int3 = Vector3<i32>;
pub type int2 = Vector2<i32>;

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

pub fn refract(i: &float3, n: &float3, ior: f32) -> float3
{
    let mut cosi = clamp(i.dot(n), -1.0, 1.0);
    let mut etai = 1.0;
    let mut etat = ior;
    let mut n = *n;
    if cosi < 0.0
    {
        cosi = -cosi;
    }
    else
    {
        std::mem::swap(&mut etai, &mut etat);
        n = -n;
    }
    let eta = etai / etat;
    let k = 1.0 - eta * eta * (1.0 - cosi * cosi);
    if k < 0.0
    {
        float3::new(0.0, 0.0, 0.0)
    }
    else
    {
        eta * i + (eta * cosi - k.sqrt()) * n
    }
   // refract_swap(i, n, ior, 1.0)
}

fn refract_swap(i: &float3, n: &float3, eta_t: f32, eta_i: f32) -> float3
{
    let cosi = clamp(i.dot(n), -1.0, 1.0);

    if cosi < 0.0
    {
        refract_swap(i, &-n, eta_i, eta_t)
    }
    else
    {
        let eta = eta_i / eta_t;
        let k = 1.0 - (eta * eta * (1.0 - (cosi * cosi)));

        if k < 0.0
        {
            float3::new(1.0, 0.0, 0.0)
        }
        else
        {
            (i * eta) + (n * (eta * cosi - k.sqrt()))
        }
    }
}