use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use super::{Architect, BuildStats, ChunkBuilder, ChunkError};
use core::ObjectManager;

#[derive(Clone)]
pub struct Worker {
    architect: Arc<Architect>,
    object_manager: Arc<ObjectManager>,
    stop: Arc<AtomicBool>,
    input_queue: Arc<Mutex<VecDeque<([i32; 2], u8)>>>,
    output_queue: Arc<Mutex<Vec<ChunkBuilder>>>,
    build_stats: Arc<Mutex<BuildStats>>,
    random_state: [u8; 16],
}

impl Worker {
    pub fn new(
        architect: Arc<Architect>,
        object_manager: Arc<ObjectManager>,
        stop: Arc<AtomicBool>,
        input_queue: Arc<Mutex<VecDeque<([i32; 2], u8)>>>,
        output_queue: Arc<Mutex<Vec<ChunkBuilder>>>,
        build_stats: Arc<Mutex<BuildStats>>,
        random_state: [u8; 16],
    ) -> Worker {
        Worker {
            architect: architect,
            object_manager: object_manager,
            stop: stop,
            input_queue: input_queue,
            output_queue: output_queue,
            build_stats: build_stats,
            random_state: random_state,
        }
    }

    pub fn work(&self) -> Result<(), ChunkError> {
        while !self.stop.load(Ordering::Relaxed) {
            self.work_cycle()?;
        }
        Ok(())
    }

    fn work_cycle(&self) -> Result<(), ChunkError> {
        let sleep_time = Duration::from_millis(500);
        if let Some((pos, lod)) = self.get_chunk_pos()? {
            let build_start = Instant::now();
            self.build_chunk(pos, lod)?;
            if lod == 0 {
                // only want stats for high quality chunks
                self.handle_build_stats(&build_start)?;
            }
        } else {
            thread::sleep(sleep_time);
        }
        Ok(())
    }

    fn build_chunk(&self, chunk_pos: [i32; 2], lod: u8) -> Result<(), ChunkError> {
        let builder = ChunkBuilder::new(
            chunk_pos,
            lod,
            &self.architect,
            &self.object_manager,
            &self.random_state,
        )?;

        match self.output_queue.lock() {
            Ok(mut guard) => (*guard).push(builder),
            Err(_poisoned) => {
                return Err(ChunkError::MutexPoison);
            }
        }
        Ok(())
    }

    fn handle_build_stats(&self, build_start: &Instant) -> Result<(), ChunkError> {
        let build_time =
            build_start.elapsed().as_secs() as u32 * 1000 + build_start.elapsed().subsec_millis();
        match self.build_stats.lock() {
            Ok(mut guard) => (*guard).add_time(build_time),
            Err(_poisoned) => {
                return Err(ChunkError::MutexPoison);
            }
        }
        Ok(())
    }

    fn get_chunk_pos(&self) -> Result<Option<([i32; 2], u8)>, ChunkError> {
        match self.input_queue.lock() {
            Ok(mut guard) => Ok((*guard).pop_back()),
            Err(_poisoned) => Err(ChunkError::MutexPoison),
        }
    }
}
