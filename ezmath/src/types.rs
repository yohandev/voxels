use crate::gene::*;

/// macro that defines a vector or matrix type
macro_rules! define_vec
{
    ( $name:ident, $gene:ty ) =>
    {
        /// a stack-allocated column-Vector
        #[allow(non_camel_case_types)]
        pub type $name = $gene;
    };
}

macro_rules! define_mat
{
    ( $name:ident, $gene:ty ) =>
    {
        /// a stack-allocated square matrix
        #[allow(non_camel_case_types)]
        pub type $name = $gene;
    };
}

define_vec!(bool2, Vec2<bool>);
define_vec!(bool3, Vec3<bool>);
define_vec!(bool4, Vec4<bool>);

define_vec!(char2, Vec2<char>);
define_vec!(char3, Vec3<char>);
define_vec!(char4, Vec4<char>);

define_vec!(sbyte2, Vec2<i8>);
define_vec!(sbyte3, Vec3<i8>);
define_vec!(sbyte4, Vec4<i8>);

define_vec!(short2, Vec2<i16>);
define_vec!(short3, Vec3<i16>);
define_vec!(short4, Vec4<i16>);

define_vec!(int2, Vec2<i32>);
define_vec!(int3, Vec3<i32>);
define_vec!(int4, Vec4<i32>);

define_vec!(long2, Vec2<i64>);
define_vec!(long3, Vec3<i64>);
define_vec!(long4, Vec4<i64>);

define_vec!(isize2, Vec2<isize>);
define_vec!(isize3, Vec3<isize>);
define_vec!(isize4, Vec4<isize>);

define_vec!(byte2, Vec2<u8>);
define_vec!(byte3, Vec3<u8>);
define_vec!(byte4, Vec4<u8>);

define_vec!(ushort2, Vec2<u16>);
define_vec!(ushort3, Vec3<u16>);
define_vec!(ushort4, Vec4<u16>);

define_vec!(uint2, Vec2<u32>);
define_vec!(uint3, Vec3<u32>);
define_vec!(uint4, Vec4<u32>);

define_vec!(ulong2, Vec2<u64>);
define_vec!(ulong3, Vec3<u64>);
define_vec!(ulong4, Vec4<u64>);

define_vec!(usize2, Vec2<usize>);
define_vec!(usize3, Vec3<usize>);
define_vec!(usize4, Vec4<usize>);

define_vec!(float2, Vec2<f32>);
define_vec!(float3, Vec3<f32>);
define_vec!(float4, Vec4<f32>);

define_vec!(double2, Vec2<f64>);
define_vec!(double3, Vec3<f64>);
define_vec!(double4, Vec4<f64>);

define_mat!(bool2x2, Mat2x2<bool>);
define_mat!(bool3x3, Mat3x3<bool>);
define_mat!(bool4x4, Mat4x4<bool>);

define_mat!(char2x2, Mat2x2<char>);
define_mat!(char3x3, Mat3x3<char>);
define_mat!(char4x4, Mat4x4<char>);

define_mat!(sbyte2x2, Mat2x2<i8>);
define_mat!(sbyte3x3, Mat3x3<i8>);
define_mat!(sbyte4x4, Mat4x4<i8>);

define_mat!(short2x2, Mat2x2<i16>);
define_mat!(short3x3, Mat3x3<i16>);
define_mat!(short4x4, Mat4x4<i16>);

define_mat!(int2x2, Mat2x2<i32>);
define_mat!(int3x3, Mat3x3<i32>);
define_mat!(int4x4, Mat4x4<i32>);

define_mat!(long2x2, Mat2x2<i64>);
define_mat!(long3x3, Mat3x3<i64>);
define_mat!(long4x4, Mat4x4<i64>);

define_mat!(isize2x2, Mat2x2<isize>);
define_mat!(isize3x3, Mat3x3<isize>);
define_mat!(isize4x4, Mat4x4<isize>);

define_mat!(byte2x2, Mat2x2<u8>);
define_mat!(byte3x3, Mat3x3<u8>);
define_mat!(byte4x4, Mat4x4<u8>);

define_mat!(ushort2x2, Mat2x2<u16>);
define_mat!(ushort3x3, Mat3x3<u16>);
define_mat!(ushort4x4, Mat4x4<u16>);

define_mat!(uint2x2, Mat2x2<u32>);
define_mat!(uint3x3, Mat3x3<u32>);
define_mat!(uint4x4, Mat4x4<u32>);

define_mat!(ulong2x2, Mat2x2<u64>);
define_mat!(ulong3x3, Mat3x3<u64>);
define_mat!(ulong4x4, Mat4x4<u64>);

define_mat!(usize2x2, Mat2x2<usize>);
define_mat!(usize3x3, Mat3x3<usize>);
define_mat!(usize4x4, Mat4x4<usize>);

define_mat!(float2x2, Mat2x2<f32>);
define_mat!(float3x3, Mat3x3<f32>);
define_mat!(float4x4, Mat4x4<f32>);

define_mat!(double2x2, Mat2x2<f64>);
define_mat!(double3x3, Mat3x3<f64>);
define_mat!(double4x4, Mat4x4<f64>);