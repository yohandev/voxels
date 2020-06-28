use ezmath::*;

/// local to world component
#[derive(Debug, Clone, Default)]
pub struct CLocalToWorld(pub float4x4);

/// translation component, to be used (optionally) with
/// the LocalToWorld component
#[derive(Debug, Clone, Default)]
pub struct CTranslation(pub float3);

/// rotation component, to be used(optionally) with the
/// Rotation component
#[derive(Debug, Clone, Default)]
pub struct CRotation(pub float3);

impl CLocalToWorld
{
    /// get the forward of this local to world matrix
    pub fn forward(&self) -> float3
    {
        let row = self.0.row(2);

        float3::new(row[0], row[1], row[2])
    }
}