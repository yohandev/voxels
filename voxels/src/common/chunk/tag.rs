/// tag to mark a chunk as currently loading
#[derive(Debug, Clone, PartialEq)]
pub struct ChunkLoadTag;

/// tag for chunks that need generation. this shouldn't
/// be added directly.
#[derive(Debug, Clone, PartialEq)]
pub struct ChunkGenerateTag;

/// tag for chunks that need remeshing. this shouldn't
/// be added directly.
#[derive(Debug, Clone, PartialEq)]
pub struct ChunkRemeshTag;