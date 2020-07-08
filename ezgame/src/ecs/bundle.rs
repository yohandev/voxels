use super::*;

/// collection of systems that can be added
/// all at once
pub trait SystemBundle
{
    fn insert(sys: &mut Systems);
}

macro_rules! impl_bundle
{
    ($($sys_id:ident),*) =>
    {
        impl<$($sys_id: System),*> SystemBundle for ($($sys_id),*,)
        {
            fn insert(sys: &mut Systems)
            {
                $(
                    $sys_id::register(sys);
                )*
            }
        }
    };
}

impl_bundle!(A);
impl_bundle!(A, B);
impl_bundle!(A, B, C);
impl_bundle!(A, B, C, D);
impl_bundle!(A, B, C, D, E);
impl_bundle!(A, B, C, D, E, F);
impl_bundle!(A, B, C, D, E, F, G);
impl_bundle!(A, B, C, D, E, F, G, H);
impl_bundle!(A, B, C, D, E, F, G, H, I);
impl_bundle!(A, B, C, D, E, F, G, H, I, J);
impl_bundle!(A, B, C, D, E, F, G, H, I, J, K);
impl_bundle!(A, B, C, D, E, F, G, H, I, J, K, L);
impl_bundle!(A, B, C, D, E, F, G, H, I, J, K, L, M);
impl_bundle!(A, B, C, D, E, F, G, H, I, J, K, L, M, N);
impl_bundle!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O);
impl_bundle!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P);
impl_bundle!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q);
impl_bundle!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R);
impl_bundle!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S);
impl_bundle!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T);
impl_bundle!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U);
impl_bundle!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V);
impl_bundle!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W);
impl_bundle!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X);
impl_bundle!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y);
impl_bundle!(A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z);