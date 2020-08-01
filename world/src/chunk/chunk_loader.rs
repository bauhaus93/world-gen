use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

use super::{BuildStats, Chunk, ChunkBuilder, ChunkError, Worker};
use crate::architect::Architect;
use core::Point2i;

const INPUT_QUEUE_MAX: usize = 200;

pub struct ChunkLoader {
    stop: Arc<AtomicBool>,
    architect: Arc<Architect>,
    input_queue: Arc<Mutex<VecDeque<(Point2i, u8)>>>,
    output_queue: Arc<Mutex<VecDeque<ChunkBuilder>>>,
    build_stats: Arc<Mutex<BuildStats>>,
    handeled_positions: BTreeSet<Point2i>,
    thread_handles: Vec<thread::JoinHandle<()>>,
}

impl ChunkLoader {
    pub fn new(architect: Arc<Architect>) -> Self {
        Self {
            stop: Arc::new(AtomicBool::new(false)),
            architect: architect,
            input_queue: Arc::new(Mutex::new(VecDeque::new())),
            output_queue: Arc::new(Mutex::new(VecDeque::new())),
            build_stats: Arc::new(Mutex::new(BuildStats::default())),
            handeled_positions: BTreeSet::new(),
            thread_handles: Vec::new(),
        }
    }
    pub fn start(&mut self, thread_count: usize) {
        if !self.thread_handles.is_empty() {
            warn!("Starting chunk loader threads, but threads already running");
        }
        self.stop.load(Ordering::Relaxed);
        let worker = Worker::new(
            self.architect.clone(),
            self.stop.clone(),
            self.input_queue.clone(),
            self.output_queue.clone(),
            self.build_stats.clone(),
        );
        for _i in 0..thread_count {
            let next_worker = worker.clone();
            let handle = thread::spawn(move || match next_worker.work() {
                Ok(_) => trace!("Worker finished successfully"),
                Err(e) => error!("Worker error: {}", e),
            });
            self.thread_handles.push(handle);
        }
        info!("Started chunk loader with {} threads", thread_count);
    }

    pub fn stop(&mut self) {
        info!("Stopping chunk loader threads");
        self.stop.store(true, Ordering::Relaxed);
        let mut stop_count = 0;
        while let Some(handle) = self.thread_handles.pop() {
            match handle.join() {
                Ok(_) => {
                    stop_count += 1;
                }
                Err(_) => warn!("Thread to join panicked"),
            }
        }
        info!("Stopped {} chunk loader threads", stop_count);
    }

    pub fn get(&mut self, max_taken: usize) -> Result<BTreeMap<Point2i, Chunk>, ChunkError> {
        let mut chunks = BTreeMap::new();
        match self.output_queue.lock() {
            Ok(mut guard) => {
                while chunks.len() < max_taken {
                    if let Some(cb) = (*guard).pop_front() {
                        let chunk = cb.finish()?;
                        let pos = chunk.get_pos();
                        self.handeled_positions.remove(&pos);
                        chunks.insert(pos, chunk);
                    } else {
                        break;
                    }
                }
            }
            Err(_poisoned) => {
                return Err(ChunkError::MutexPoison);
            }
        }
        Ok(chunks)
    }

    pub fn request(&mut self, chunk_pos: &[(Point2i, u8)]) -> Result<(), ChunkError> {
        match self.input_queue.lock() {
            Ok(mut guard) => {
                for (pos, lod) in chunk_pos {
                    if (*guard).len() >= INPUT_QUEUE_MAX {
                        break;
                    }
                    if self.handeled_positions.insert(*pos) {
                        (*guard).push_back((*pos, *lod));
                    }
                }
                Ok(())
            }
            Err(_) => Err(ChunkError::MutexPoison),
        }
    }

    pub fn get_avg_build_time(&self) -> f64 {
        match self.build_stats.lock() {
            Ok(mut guard) => (*guard).get_avg_time(),
            Err(_poisoned) => 0.,
        }
    }
}

impl Drop for ChunkLoader {
    fn drop(&mut self) {
        self.stop();
    }
}
