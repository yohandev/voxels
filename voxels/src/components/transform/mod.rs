use ezmath::*;

/// local to world component
#[derive(Debug, Clone, Default)]
pub struct LocalToWorld(pub float4x4);

/// translation component, to be used (optionally) with
/// the LocalToWorld component
#[derive(Debug, Clone, Default)]
pub struct Translation(pub float3);

/// rotation component, to be used(optionally) with the
/// Rotation component
#[derive(Debug, Clone, Default)]
pub struct Rotation(pub float3);