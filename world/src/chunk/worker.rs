use std::collections::VecDeque;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use super::{BuildStats, ChunkBuilder, ChunkError};
use crate::architect::Architect;
use core::Point2i;

#[derive(Clone)]
pub struct Worker {
    architect: Arc<Architect>,
    stop: Arc<AtomicBool>,
    input_queue: Arc<Mutex<VecDeque<(Point2i, u8)>>>,
    output_queue: Arc<Mutex<VecDeque<ChunkBuilder>>>,
    build_stats: Arc<Mutex<BuildStats>>,
}

impl Worker {
    pub fn new(
        architect: Arc<Architect>,
        stop: Arc<AtomicBool>,
        input_queue: Arc<Mutex<VecDeque<(Point2i, u8)>>>,
        output_queue: Arc<Mutex<VecDeque<ChunkBuilder>>>,
        build_stats: Arc<Mutex<BuildStats>>,
    ) -> Worker {
        Worker {
            architect: architect,
            stop: stop,
            input_queue: input_queue,
            output_queue: output_queue,
            build_stats: build_stats,
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

    fn build_chunk(&self, chunk_pos: Point2i, lod: u8) -> Result<(), ChunkError> {
        let builder = ChunkBuilder::new(chunk_pos, lod, self.architect.as_ref())?;

        match self.output_queue.lock() {
            Ok(mut guard) => (*guard).push_back(builder),
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

    fn get_chunk_pos(&self) -> Result<Option<(Point2i, u8)>, ChunkError> {
        match self.input_queue.lock() {
            Ok(mut guard) => Ok((*guard).pop_back()),
            Err(_poisoned) => Err(ChunkError::MutexPoison),
        }
    }
}
