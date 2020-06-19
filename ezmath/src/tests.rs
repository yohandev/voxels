use crate::*;

#[test]
fn test_vectors()
{
    // just testing out the types work
    // it's all just nalgebra behind the scenes,
    // which should work.
    let _ = float4x4::identity();
    let _ = float4x4::perspective(0.5, 45f32.to_radians(), 0.01, 100.0);
    let _ = float4x4::translation(float3::one());
    let _ = float3x3::rotation(30.0);
}