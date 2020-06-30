/// tag to mark a chunk as ungenerated
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TUngenerated;

/// tag to mark a chunk that had a block updated
/// in the past frame.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct TUpdated;