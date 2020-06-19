use ezgame::legion::*;

use crate::resources::game::LoadedChunks;
use crate::components::game::*;

/// system that updates the ChunkLoaded resource
pub fn system() -> Box<dyn Schedulable>
{
    SystemBuilder::new("loaded_chunks_system")
        // resources
        .write_resource::<LoadedChunks>()
        // components
        .with_query(<Read<Chunk>>::query().filter(tag::<ChunkLoadTag>()))
        // tags
        .write_component::<ChunkLoadTag>()
        .write_component::<ChunkGenerateTag>()
        // system
        .build(|cmd, world, loaded, query|
        {
            for (ent, chunk) in query.iter_entities(world)
            {
                // store into loaded chunks
                loaded.chunks.insert(chunk.position(), ent);

                // remove and set tag
                cmd.remove_tag::<ChunkLoadTag>(ent);
                cmd.add_tag(ent, ChunkGenerateTag);
            }
        })
}