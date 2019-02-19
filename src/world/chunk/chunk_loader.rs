use std::sync::{ Arc, Mutex };
use std::collections::{ BTreeMap, BTreeSet, VecDeque };
use std::time;
use std::thread;
use std::sync::atomic::{ AtomicBool, Ordering };

use super::{ Chunk, chunk_builder::ChunkBuilder, architect::Architect, ChunkError };
use super::tree::Tree;

pub struct ChunkLoader {
    stop: Arc<AtomicBool>,
    architect: Arc<Architect>,
    input_queue: Arc<Mutex<VecDeque<[i32; 2]>>>,
    output_queue: Arc<Mutex<Vec<ChunkBuilder>>>,
    handeled_positions: BTreeSet<[i32; 2]>,
    thread_handles: Vec<thread::JoinHandle<()>>
}

impl ChunkLoader {
    pub fn start(&mut self, thread_count: usize) {
        if !self.thread_handles.is_empty() {
            warn!("Starting chunk loader threads, but threads already running");
        }
        self.stop.load(Ordering::Relaxed);
        for _i in 0..thread_count {
            let architect = self.architect.clone();
            let stop = self.stop.clone();
            let input = self.input_queue.clone();
            let output = self.output_queue.clone();
            let handle = thread::spawn(move || {
                worker(architect, stop, input, output);
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
        info!("{} chunk loader threads stopped", stop_count);
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

    pub fn request(&mut self, chunk_pos: &[[i32; 2]]) -> Result<(), ChunkError> {
        match self.input_queue.lock() {
            Ok(mut guard) => {
                for pos in chunk_pos {
                    if self.handeled_positions.insert(*pos) {
                        (*guard).push_back(*pos);
                    }
                }
                Ok(())
            },
            Err(_) => Err(ChunkError::MutexPoison)
        }
    }
}
impl Default for ChunkLoader {
    fn default() -> Self {
        Self {
            stop: Arc::new(AtomicBool::new(false)),
            architect: Arc::new(Architect::default()),
            input_queue: Arc::new(Mutex::new(VecDeque::new())),
            output_queue: Arc::new(Mutex::new(Vec::new())),
            handeled_positions: BTreeSet::new(),
            thread_handles: Vec::new()
        }
    }
}

impl Drop for ChunkLoader {
    fn drop(&mut self) {
        self.stop();
    }
}

fn worker(architect: Arc<Architect>, stop: Arc<AtomicBool>, input_queue: Arc<Mutex<VecDeque<[i32; 2]>>>, output_queue: Arc<Mutex<Vec<ChunkBuilder>>>) {
    let sleep_time = time::Duration::from_millis(500);
    'exit: while !stop.load(Ordering::Relaxed) {
        let pos_opt = match input_queue.lock() {
            Ok(mut guard) => (*guard).pop_back(),
            Err(_poisoned) => { break 'exit; }
        };
        if let Some(pos) = pos_opt {
            let mut builder = ChunkBuilder::new(pos);
            builder.create_surface_buffer(architect.as_ref());
            match output_queue.lock() {
                Ok(mut guard) => (*guard).push(builder),
                Err(_poisoned) => { break 'exit; }
            }
        } else {
            thread::sleep(sleep_time);
        }
    }
}
