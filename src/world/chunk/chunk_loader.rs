use std::sync::{ Arc, Mutex };
use std::collections::{ BTreeMap, VecDeque };
use std::time;
use std::thread;
use std::sync::atomic::{ AtomicBool, Ordering };

use crate::world::noise::Noise;
use super::{ Chunk, ChunkBuilder, ChunkError };

pub struct ChunkLoader {
    height_noise: Arc<Noise>,
    stop: Arc<AtomicBool>,
    input_queue: Arc<Mutex<VecDeque<[i32; 2]>>>,
    output_queue: Arc<Mutex<Vec<ChunkBuilder>>>,
    thread_handles: Vec<thread::JoinHandle<()>>
}

impl ChunkLoader {
    pub fn new(height_noise: Box<Noise>) -> ChunkLoader {
        let height_noise: Arc<Noise> = Arc::from(height_noise);
        let stop = Arc::new(AtomicBool::new(false));
        let input_queue = Arc::new(Mutex::new(VecDeque::new()));
        let output_queue = Arc::new(Mutex::new(Vec::new()));
        let mut thread_handles = Vec::new();
        for _i in 0..8 {
            let hn: Arc<Noise> = height_noise.clone();
            let stop = stop.clone();
            let input = input_queue.clone();
            let output = output_queue.clone();
            let handle = thread::spawn(move || {
                worker(hn, stop, input, output);
            });
            thread_handles.push(handle);
        }
        Self {
            height_noise: height_noise,
            stop: stop,
            input_queue: input_queue,
            output_queue: output_queue,
            thread_handles: thread_handles
        }
    }

    pub fn get(&mut self) -> Result<BTreeMap<[i32; 2], Chunk>, ChunkError> {
        let mut chunks = BTreeMap::new();
        match self.output_queue.lock() {
            Ok(mut guard) => {
                while let Some(cb) = (*guard).pop() {
                    let chunk = cb.finish()?;
                    chunks.insert(chunk.get_pos(), chunk);
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
                    (*guard).push_back(*pos);
                }
                Ok(())
            },
            Err(_) => Err(ChunkError::MutexPoison)
        }
    }
}

fn worker(height_noise: Arc<Noise>, stop: Arc<AtomicBool>, input_queue: Arc<Mutex<VecDeque<[i32; 2]>>>, output_queue: Arc<Mutex<Vec<ChunkBuilder>>>) {
    let sleep_time = time::Duration::from_millis(500);
    'exit: while !stop.load(Ordering::Relaxed) {
        let pos_opt = match input_queue.lock() {
            Ok(mut guard) => (*guard).pop_back(),
            Err(_poisoned) => { break 'exit; }
        };
        if let Some(pos) = pos_opt {
            let mut builder = ChunkBuilder::new(pos);
            builder.create_surface_buffer(height_noise.as_ref());
            match output_queue.lock() {
                Ok(mut guard) => (*guard).push(builder),
                Err(_poisoned) => { break 'exit; }
            }
        } else {
            thread::sleep(sleep_time);
        }
    }
}
