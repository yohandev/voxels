/// represents a type that's suitable to be a scalar in a vector or matrix.
/// implemented for pretty much every primitive
pub trait Scalar: std::fmt::Debug + std::fmt::Display + Clone + Copy + PartialEq + PartialOrd + num_traits::Zero + num_traits::One + simba::scalar::RealField + 'static {}

impl<T> Scalar for T where T: std::fmt::Debug + std::fmt::Display + Clone + Copy + PartialEq + PartialOrd + num_traits::Zero + num_traits::One + simba::scalar::RealField + 'static {}

/// trait to add capabilities to nalgebra's Vector2
pub trait Vec2<T: Scalar>
{
    /// get a new vector with all its components initialized to one
    fn one() -> Self;
    /// get a new vector with all its components initialized to zero
    fn zero() -> Self;
}

/// trait to add capabilities to nalgebra's Vector3
pub trait Vec3<T: Scalar>
{
    /// get a new vector with all its components initialized to one
    fn one() -> Self;
    /// get a new vector with all its components initialized to zero
    fn zero() -> Self;
}

/// trait to add capabilities to nalgebra's Vector4
pub trait Vec4<T: Scalar>
{
    /// get a new vector with all its components initialized to one
    fn one() -> Self;
    /// get a new vector with all its components initialized to zero
    fn zero() -> Self;
}

/// trait to add capabilities to nalgebra's 2x2 matrix
pub trait Mat2x2<T: Scalar>
{
    /// constructs a new matrix from translation transformation
    fn translation(translation: T) -> Self;
    /// returns the inverse of the matrix
    fn inverse(&self) -> Self;
}

/// trait to add capabilities to nalgebra's 3x3 matrix
pub trait Mat3x3<T: Scalar>
{
    /// constructs a new matrix from translation transformation
    fn translation(translation: nalgebra::Vector2<T>) -> Self;
    /// constructs a new matrix from an euler angle rotation
    fn rotation(euler_angle: T) -> Self;
    /// returns the inverse of the matrix
    fn inverse(&self) -> Self;
}

/// trait to add capabilities to nalgebra's 4x4 matrix
pub trait Mat4x4<T: Scalar>
{
    /// constructs a new matrix from a perspective transformation
    fn perspective(aspect: T, fov: T, near: T, far: T) -> Self;
    /// constructs a new matrix from translation transformation
    fn translation(translation: nalgebra::Vector3<T>) -> Self;
    /// constructs a new matrix from euler angles rotation
    fn rotation(euler_angles: nalgebra::Vector3<T>) -> Self;
    /// returns the inverse of the matrix
    fn inverse(&self) -> Self;
}

impl<T: Scalar> Vec2<T> for nalgebra::Vector2<T>
{
    fn one() -> Self
    {
        Self::new(T::one(), T::one())
    }

    fn zero() -> Self
    {
        Self::new(T::zero(), T::zero())
    }
}

impl<T: Scalar> Vec3<T> for nalgebra::Vector3<T>
{
    fn one() -> Self
    {
        Self::new(T::one(), T::one(), T::one())
    }

    fn zero() -> Self
    {
        Self::new(T::zero(), T::zero(), T::zero())
    }
}

impl<T: Scalar> Vec4<T> for nalgebra::Vector4<T>
{
    fn one() -> Self
    {
        Self::new(T::one(), T::one(), T::one(), T::one())
    }

    fn zero() -> Self
    {
        Self::new(T::zero(), T::zero(), T::zero(), T::zero())
    }
}

impl<T: Scalar> Mat2x2<T> for nalgebra::Matrix2<T>
{
    fn translation(n: T) -> Self
    {
        nalgebra::Translation1::new(n).to_homogeneous()
    }

    fn inverse(&self) -> Self
    {
        self.try_inverse().unwrap_or(*self)
    }
}

impl<T: Scalar> Mat3x3<T> for nalgebra::Matrix3<T>
{
    fn translation(n: nalgebra::Vector2<T>) -> Self
    {
        nalgebra::Translation2::new(n.x, n.y).to_homogeneous()
    }

    fn rotation(n: T) -> Self
    {
        nalgebra::Rotation2::new(n).to_homogeneous()
    }

    fn inverse(&self) -> Self
    {
        self.try_inverse().unwrap_or(*self)
    }
}

impl<T: Scalar> Mat4x4<T> for nalgebra::Matrix4<T>
{
    fn perspective(aspect: T, fov: T, near: T, far: T) -> Self
    {
        nalgebra::Perspective3::new(aspect, fov, near, far).into_inner()
    }

    fn translation(n: nalgebra::Vector3<T>) -> Self
    {
        nalgebra::Translation3::new(n.x, n.y, n.z).to_homogeneous()
    }

    fn rotation(n: nalgebra::Vector3<T>) -> Self
    {
        nalgebra::Rotation3::from_euler_angles(n.x, n.y, n.z).to_homogeneous()
    }

    fn inverse(&self) -> Self
    {
        self.try_inverse().unwrap_or(*self)
    }
}
