#[macro_export]
macro_rules! buffer_data
{
    (
        $(#[$struct_meta:meta])*
        $sv:vis struct $struct_name:ident
        {
            $($fv:vis $field_name:ident: $field_type:ty),* $(,)?
        }
    ) =>
    {
        #[repr(C)]
        #[derive(Debug, Copy, Clone)]
        $(#[$struct_meta])*
        $sv struct $struct_name
        {
            $($fv $field_name: $field_type),*
        }

        impl $crate::BufferData for $struct_name {}

        unsafe impl $crate::bytemuck::Zeroable for $struct_name {}
        unsafe impl $crate::bytemuck::Pod      for $struct_name {}
    };
}