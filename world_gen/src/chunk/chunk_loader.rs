use std::sync::{ Arc, Mutex };
use std::collections::{ BTreeMap, BTreeSet, VecDeque };
use std::thread;
use std::sync::atomic::{ AtomicBool, Ordering };

use rand::{ Rng };

use crate::{ ObjectManager };
use super::{ Chunk, ChunkBuilder, Architect, ChunkError, BuildStats, Worker };

pub struct ChunkLoader {
    stop: Arc<AtomicBool>,
    architect: Arc<Architect>,
    object_manager: Arc<ObjectManager>,
    input_queue: Arc<Mutex<VecDeque<([i32; 2], u8)>>>,
    output_queue: Arc<Mutex<Vec<ChunkBuilder>>>,
    build_stats: Arc<Mutex<BuildStats>>,
    handeled_positions: BTreeSet<[i32; 2]>,
    thread_handles: Vec<thread::JoinHandle<()>>,
    random_state: [u8; 16]
}


impl ChunkLoader {
    pub fn new<R: Rng + ?Sized>(rng: &mut R, object_manager: Arc<ObjectManager>) -> Self {
        let mut random_state = [0; 16];
        rng.fill_bytes(&mut random_state);
        Self {
            stop: Arc::new(AtomicBool::new(false)),
            architect: Arc::new(Architect::from_rng(rng)),
            object_manager: object_manager,
            input_queue: Arc::new(Mutex::new(VecDeque::new())),
            output_queue: Arc::new(Mutex::new(Vec::new())),
            build_stats: Arc::new(Mutex::new(BuildStats::default())),
            handeled_positions: BTreeSet::new(),
            thread_handles: Vec::new(),
            random_state: random_state
        }
    }
    pub fn start(&mut self, thread_count: usize) {
        if !self.thread_handles.is_empty() {
            warn!("Starting chunk loader threads, but threads already running");
        }
        self.stop.load(Ordering::Relaxed);
        let worker = Worker::new(
            self.architect.clone(),
            self.object_manager.clone(),
            self.stop.clone(),
            self.input_queue.clone(),
            self.output_queue.clone(),
            self.build_stats.clone(),
            self.random_state
        );
        for _i in 0..thread_count {
            let next_worker = worker.clone();
            let handle = thread::spawn(move || {
                match next_worker.work() {
                    Ok(_) => debug!("Worker finished successfully"),
                    Err(e) =>  error!("Worker error: {}", e)
                 }
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
                Ok(_) => { stop_count += 1; },
                Err(_) => warn!("Thread to join panicked")
            }
        }
        info!("Stopped {} chunk loader threads", stop_count);
    }

    pub fn get(&mut self) -> Result<BTreeMap<[i32; 2], Chunk>, ChunkError> {
        let mut chunks = BTreeMap::new();
        match self.output_queue.lock() {
            Ok(mut guard) => {
                while let Some(cb) = (*guard).pop() {
                    let chunk = cb.finish()?;
                    let pos = chunk.get_pos();
                    self.handeled_positions.remove(&pos);
                    chunks.insert(pos, chunk);
                }
            },
            Err(_poisoned) => { return Err(ChunkError::MutexPoison); }
        }
        Ok(chunks)
    }

    pub fn request(&mut self, chunk_pos: &[([i32; 2], u8)]) -> Result<(), ChunkError> {
        match self.input_queue.lock() {
            Ok(mut guard) => {
                for (pos, lod) in chunk_pos {
                    if self.handeled_positions.insert(*pos) {
                        (*guard).push_back((*pos, *lod));
                    }
                }
                Ok(())
            },
            Err(_) => Err(ChunkError::MutexPoison)
        }
    }

    pub fn get_avg_build_time(&self) -> f64 {
        match self.build_stats.lock() {
            Ok(mut guard) => {
              (*guard).get_avg_time()
            },
            Err(_poisoned) => 0.
        }
    }
}

impl Drop for ChunkLoader {
    fn drop(&mut self) {
        self.stop();
    }
}


