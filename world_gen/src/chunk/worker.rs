use std::sync::{ Arc, Mutex };
use std::collections::VecDeque;
use std::time;
use std::thread;
use std::sync::atomic::{ AtomicBool, Ordering };

use crate::{ ObjectManager, Object };
use super::{ ChunkBuilder, Architect, ChunkError, BuildStats, CHUNK_SIZE };

#[derive(Clone)]
pub struct Worker {
    architect: Arc<Architect>,
    object_manager: Arc<ObjectManager>,
    stop: Arc<AtomicBool>,
    input_queue: Arc<Mutex<VecDeque<([i32; 2], u8)>>>,
    output_queue: Arc<Mutex<Vec<ChunkBuilder>>>,
    build_stats: Arc<Mutex<BuildStats>>
}

impl Worker {
    pub fn new(
        architect: Arc<Architect>,
        object_manager: Arc<ObjectManager>,
        stop: Arc<AtomicBool>,
        input_queue: Arc<Mutex<VecDeque<([i32; 2], u8)>>>,
        output_queue: Arc<Mutex<Vec<ChunkBuilder>>>,
        build_stats: Arc<Mutex<BuildStats>>
    ) -> Worker {
        Worker {
            architect: architect,
            object_manager: object_manager,
            stop: stop,
            input_queue: input_queue,
            output_queue: output_queue,
            build_stats: build_stats
        }
    }

    pub fn work(&self) -> Result<(), ChunkError> {
        let sleep_time = time::Duration::from_millis(500);
        'exit: while !self.stop.load(Ordering::Relaxed) {
            let pos_opt = match self.input_queue.lock() {
                Ok(mut guard) => (*guard).pop_back(),
                Err(_poisoned) => { return Err(ChunkError::MutexPoison); }
            };
            if let Some((pos, lod)) = pos_opt {
                let build_start = time::Instant::now();
                let mut builder = ChunkBuilder::new(pos, lod);
                
                let raw_height_map = match lod {
                    0 => self.architect.create_height_map(pos, CHUNK_SIZE, 1),
                    _ => self.architect.create_height_map(pos, CHUNK_SIZE / 8, 8),
                };
                builder.create_surface_buffer(&raw_height_map);

                let build_time = build_start.elapsed().as_secs() as u32 * 1000 + build_start.elapsed().subsec_millis();
                match self.build_stats.lock() {
                    Ok(mut guard) => (*guard).add_time(build_time),
                    Err(_poisoned) => { return Err(ChunkError::MutexPoison); }
                }
                match self.output_queue.lock() {
                    Ok(mut guard) => (*guard).push(builder),
                    Err(_poisoned) => { return Err(ChunkError::MutexPoison); }
                }
            } else {
                thread::sleep(sleep_time);
            }
        }
        Ok(())
    }
}
