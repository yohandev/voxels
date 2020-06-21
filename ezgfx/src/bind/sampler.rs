/// a bind-able shader resource that, as the
/// name implies, samples textures. this is
/// kind of like a colour picker in photo
/// editing software.
#[derive(Debug)]
pub struct Sampler
{
    smp: wgpu::Sampler
}

/// input struct for a sampler, which have so
/// many arguments that can't be abstracted
/// away
pub struct SamplerDesc
{
    /// filter op when a texture is magnified
    /// (tex is X size, but is rendered at Y
    /// size where Y > X)
    pub mag: Filter,
    /// filter op when a texture is minified
    /// (tex is X size, but is rendered at Y
    /// size where Y < X)
    pub min: Filter,
    /// edge operation for (u, v)
    pub edge: (Edge, Edge),
    /// what kind of sampler will this be in
    /// GLSL code? `sampler` or `samplerShadow`
    pub mode: Mode
}

/// filter op in a sampler nearest
#[derive(Debug, Copy, Clone)]
pub enum Filter
{
    /// tries to mix nearby pixels, giving a smooth
    /// effect
    Linear,
    /// takes whatever nearest pixel, giving a blocky
    /// pixel-art look.
    Nearest,
}

/// edge op in a sampler. when the sampler attempts
/// to sample something at u/v > 1.0 or u/v < 0.0,
/// the edge op comes into play.
#[derive(Debug, Copy, Clone)]
pub enum Edge
{
    /// repeats the texture as it is.
    Repeat,
    /// clamps the border pixel, ie u = 1.87 becomes
    /// 1.0
    Clamp,
    /// same as repeat, except texture is mirrored
    /// each time
    Mirror
}

/// mode of the sampler, which affects the compare
/// function
#[derive(Debug, Copy, Clone)]
pub enum Mode
{
    /// treat this sampler a normal GLSL `sampler`.
    /// comparison function: always
    Normal,
    /// treat this sampler as a GLSL `samplerShadow`.
    /// comparion function: less-equal
    Depth,
}

impl crate::Bind for Sampler
{
    fn binding_type(&self) -> wgpu::BindingType
    {
        wgpu::BindingType::Sampler
        {
            comparison: false
        }
    }

    fn resource(&self) -> wgpu::BindingResource
    {
        wgpu::BindingResource::Sampler(&self.smp)
    }
}

impl Sampler
{
    /// create a new sampler. this should not be
    /// called directly.
    pub(crate) fn new(ctx: &crate::Renderer, desc: &SamplerDesc) -> Self
    {
        Self
        {
            smp: ctx.device.create_sampler
            (
                &wgpu::SamplerDescriptor
                {
                    address_mode_u: desc.edge.0.to_wgpu(),
                    address_mode_v: desc.edge.1.to_wgpu(),
                    address_mode_w: desc.edge.0.to_wgpu(),
                    mag_filter: desc.mag.to_wgpu(),
                    min_filter: desc.min.to_wgpu(),
                    mipmap_filter: desc.min.to_wgpu(),
                    lod_min_clamp: -100.0,
                    lod_max_clamp: 100.0,
                    compare: desc.mode.to_wgpu(),
                }
            )
        }
    }
}

impl Filter
{
    /// translate this enum to the wgpu one
    fn to_wgpu(self) -> wgpu::FilterMode
    {
        match self
        {
            Filter::Linear => wgpu::FilterMode::Linear,
            Filter::Nearest => wgpu::FilterMode::Nearest
        }
    }
}

impl Edge
{
    /// translate this enum to the wgpu one
    fn to_wgpu(self) -> wgpu::AddressMode
    {
        match self
        {
            Edge::Repeat => wgpu::AddressMode::Repeat,
            Edge::Clamp => wgpu::AddressMode::ClampToEdge,
            Edge::Mirror => wgpu::AddressMode::MirrorRepeat,
        }
    }
}

impl Mode
{
    /// translate this enum to the wgpu one
    fn to_wgpu(self) -> wgpu::CompareFunction
    {
        match self
        {
            Mode::Normal => wgpu::CompareFunction::Always,
            Mode::Depth => wgpu::CompareFunction::LessEqual,
        }
    }   
}