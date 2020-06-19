#![allow(dead_code)]

/// number of blocks in one dimension of a chunk
/// currently, chunks are 32x32x32
pub const CHUNK_SIZE: usize = 32;
/// square of CHUNK_SIZE = 32 * 32
pub const CHUNK_LAYER: usize = CHUNK_SIZE * CHUNK_SIZE;
/// cube of CHUNK_SIZE = 32 * 32 * 32
/// total number of full blocks in a chunk
pub const CHUNK_VOLUME: usize = CHUNK_LAYER * CHUNK_SIZE;