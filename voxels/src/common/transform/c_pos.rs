use ezmath::*;

/// translation component, to be used (optionally) with
/// the LocalToWorld component
#[derive(Debug, Clone, Default)]
pub struct CTranslation(pub float3);