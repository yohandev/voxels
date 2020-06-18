use ezmath::*;

/// local to world component
pub struct LocalToWorld(pub float4x4);

/// translation component, to be used (optionally) with
/// the LocalToWorld component
pub struct Translation(pub float3);

/// rotation component, to be used(optionally) with the
/// Rotation component
pub struct Rotation(pub float3);