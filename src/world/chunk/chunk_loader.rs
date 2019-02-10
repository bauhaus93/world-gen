use std::sync::Arc;
use std::collections::BTreeMap;
use futures::future::{ Future, lazy };
use futures::Async;

use tokio_threadpool::{ ThreadPool, Builder, SpawnHandle };

use crate::world::noise::Noise;
use super::{ Chunk, ChunkBuilder, ChunkError };

pub struct ChunkLoader {
    height_noise: Arc<Noise>,
    thread_pool: ThreadPool,
    builder_futures: BTreeMap<[i32; 2], SpawnHandle<ChunkBuilder, ChunkError>>
}

impl ChunkLoader {
    pub fn new(height_noise: Box<Noise>) -> ChunkLoader {
        let thread_pool = Builder::new()
            .pool_size(10)
            .build();
        Self {
            height_noise: Arc::from(height_noise),
            thread_pool: thread_pool,
            builder_futures: BTreeMap::new()
        }
    }

    pub fn get(&mut self) -> Result<BTreeMap<[i32; 2], Chunk>, ChunkError> {
        let mut chunks = BTreeMap::new();
        let mut finished_pos: Vec<[i32; 2]> = Vec::new();
        for (pos, builder_fut) in self.builder_futures.iter_mut() {
            let builder = builder_fut.wait()?;
            finished_pos.push(*pos);
            chunks.insert(*pos, builder.finish()?);
        }
        for pos in finished_pos {
            self.builder_futures.remove(&pos);
        }
        Ok(chunks)
    }

    pub fn request(&mut self, chunk_pos: [i32; 2]) {
        let hn = self.height_noise.clone();
        let handle = self.thread_pool.spawn_handle(lazy(move || {
            let mut builder = ChunkBuilder::new(chunk_pos);
            builder.build_surface(hn.as_ref());
            Ok::<_, ChunkError>(builder)
        }));
        self.builder_futures.insert(chunk_pos, handle);
    }
}
