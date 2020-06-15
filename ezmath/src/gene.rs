macro_rules! impl_deref
{
    ($name:ident, $na_name:ident) =>
    {
        pub struct $name<T>(pub(crate) nalgebra::$na_name<T>)
            where T: std::fmt::Debug + std::fmt::Display + Clone + Copy + PartialEq + PartialOrd  + num_traits::Zero + num_traits::One + simba::scalar::RealField + 'static;

        impl<T> std::ops::Deref for $name<T>
            where T: std::fmt::Debug + std::fmt::Display + Clone + Copy + PartialEq + PartialOrd  + num_traits::Zero + num_traits::One + simba::scalar::RealField + 'static
        {
            type Target = nalgebra::$na_name<T>;
        
            fn deref(&self) -> &Self::Target
            {
                &self.0
            }
        }
        
        impl<T> std::ops::DerefMut for $name<T>
            where T: std::fmt::Debug + std::fmt::Display + Clone + Copy + PartialEq + PartialOrd  + num_traits::Zero + num_traits::One + simba::scalar::RealField + 'static
        {
            fn deref_mut(&mut self) -> &mut Self::Target
            {
                &mut self.0
            }
        }
    };
}

impl_deref!(Vec2, Vector2);
impl_deref!(Vec3, Vector3);
impl_deref!(Vec4, Vector4);

impl_deref!(Mat2x2, Matrix2);
impl_deref!(Mat3x3, Matrix3);
impl_deref!(Mat4x4, Matrix4);