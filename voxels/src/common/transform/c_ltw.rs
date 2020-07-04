use ezmath::*;

/// local to world component
#[derive(Debug, Clone, Default)]
pub struct CLocalToWorld(pub float4x4);

impl CLocalToWorld
{
    /// get the forward of this local to world matrix
    pub fn forward(&self) -> float3
    {
        let row = self.0.column(0);

        float3::new(row[0], row[1], row[2])
    }

    /// get the right of this local to world matrix
    pub fn right(&self) -> float3
    {
        let row = self.0.column(2);

        float3::new(row[0], row[1], row[2])
    }

    /// get the up of this local to world matrix
    #[allow(dead_code)]
    pub fn up(&self) -> float3
    {
        let row = self.0.column(1);

        float3::new(row[0], row[1], row[2])
    }
}