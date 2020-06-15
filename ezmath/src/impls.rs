use std::fmt::Display;

use simba::scalar::*;
use num_traits::*;
use nalgebra::*;

use crate::gene::*;

impl<T> Display for Vec2<T>
    where T: std::fmt::Debug + std::fmt::Display + Clone + Copy + PartialEq + PartialOrd  + Zero + One + RealField + 'static
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        // nalgebra display is really large and annoying for debugging
        f.write_fmt(format_args!("<{}, {}>", self.x, self.y))
    }
}

impl<T> Display for Vec3<T>
    where T: std::fmt::Debug + std::fmt::Display + Clone + Copy + PartialEq + PartialOrd  + Zero + One + RealField + 'static
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        // nalgebra display is really large and annoying for debugging
        f.write_fmt(format_args!("<{}, {}, {}>", self.x, self.y, self.z))
    }
}

impl<T> Display for Vec4<T>
    where T: std::fmt::Debug + std::fmt::Display + Clone + Copy + PartialEq + PartialOrd  + Zero + One + RealField + 'static
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        // nalgebra display is really large and annoying for debugging
        f.write_fmt(format_args!("<{}, {}, {}, {}>", self.x, self.y, self.z, self.w))
    }
}

impl<T> Display for Mat2x2<T>
    where T: std::fmt::Debug + std::fmt::Display + Clone + Copy + PartialEq + PartialOrd  + Zero + One + RealField + 'static
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        // just use the nalgebra one. matrices are large display either way
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl<T> Display for Mat3x3<T>
    where T: std::fmt::Debug + std::fmt::Display + Clone + Copy + PartialEq + PartialOrd  + Zero + One + RealField + 'static
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        // just use the nalgebra one. matrices are large display either way
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl<T> Display for Mat4x4<T>
    where T: std::fmt::Debug + std::fmt::Display + Clone + Copy + PartialEq + PartialOrd  + Zero + One + RealField + 'static
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        // just use the nalgebra one. matrices are large display either way
        f.write_fmt(format_args!("{}", self.0))
    }
}

impl<T> Vec2<T>
    where T: std::fmt::Debug + std::fmt::Display + Clone + Copy + PartialEq + PartialOrd + Zero + One + RealField + 'static
{
    /// create a new vector
    pub fn new(x: T, y: T) -> Self
    {
        Vec2(Vector2::new(x, y))
    }

    /// create a new vector initialized with 1s in all dimensions
    pub fn one() -> Self
    {
        Self::new(T::one(), T::one())
    }

    /// create a new vector initialized with 0s in all dimensions
    pub fn zero() -> Self
    {
        Self::new(T::zero(), T::zero())
    }
}

impl<T> Mat4x4<T>
    where T: std::fmt::Debug + std::fmt::Display + Clone + Copy + PartialEq + PartialOrd + Zero + One + RealField + 'static
{
    /// create a new matrix initialized with its entity
    pub fn identity() -> Self
    {
        Self(Matrix4::identity())
    }

    /// create a new matrix from a row slice
    pub fn new(slice: &[T]) -> Self
    {
        Self(Matrix4::from_row_slice(slice))
    }

    /// create a new matrix from a translation
    pub fn from_translation(n: Vec3<T>) -> Self
    {
        Self(Translation3::new(n.x, n.y, n.z).to_homogeneous())
    }

    /// create a new matrix from euler angles rotation
    pub fn from_euler_angles(n: Vec3<T>) -> Self
    {
        Self(Rotation3::from_euler_angles(n.x, n.y, n.z).to_homogeneous())
    }

    /// create a new matrix from a perspective
    pub fn from_perspective(aspect: T, fov: T, near: T, far: T) -> Self
    {
        Self(Perspective3::new(aspect, fov, near, far).into_inner())
    }
}