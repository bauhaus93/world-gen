use std::sync::{ Arc, Mutex };
use std::collections::{ BTreeMap, BTreeSet, VecDeque };
use std::time;
use std::thread;
use std::sync::atomic::{ AtomicBool, Ordering };

use rand::{ Rng };

use crate::world::erosion::hydraulic_erosion::HydraulicErosion;
use super::{ Chunk, chunk_builder::ChunkBuilder, architect::Architect, ChunkError };
use super::chunk_size::CHUNK_SIZE;

pub struct ChunkLoader {
    stop: Arc<AtomicBool>,
    architect: Arc<Architect>,
    input_queue: Arc<Mutex<VecDeque<[i32; 2]>>>,
    output_queue: Arc<Mutex<Vec<ChunkBuilder>>>,
    build_stats: Arc<Mutex<BuildStats>>,
    handeled_positions: BTreeSet<[i32; 2]>,
    thread_handles: Vec<thread::JoinHandle<()>>
}

struct BuildStats {
    build_time_accumulated: u32,
    build_count: u32,
}

impl ChunkLoader {
    pub fn from_rng<R: Rng + ?Sized>(rng: &mut R) -> Self {
        Self {
            stop: Arc::new(AtomicBool::new(false)),
            architect: Arc::new(Architect::from_rng(rng)),
            input_queue: Arc::new(Mutex::new(VecDeque::new())),
            output_queue: Arc::new(Mutex::new(Vec::new())),
            build_stats: Arc::new(Mutex::new(BuildStats::default())),
            handeled_positions: BTreeSet::new(),
            thread_handles: Vec::new()
        }
    }
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
            let build_stats = self.build_stats.clone();
            let handle = thread::spawn(move || {
                worker(architect, stop, input, output, build_stats);
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

impl Default for BuildStats {
    fn default() -> Self {
        Self {
            build_time_accumulated: 0,
            build_count: 0,
        }
    }
}

impl BuildStats {
    pub fn add_time(&mut self, build_time: u32) {
        self.build_time_accumulated += build_time;
        self.build_count += 1;
    }
    pub fn get_avg_time(&mut self) -> f64 {
        if self.build_count > 0 {
            self.build_time_accumulated as f64 / self.build_count as f64
        } else {
            0.
        }
    }
}

fn worker(architect: Arc<Architect>,
          stop: Arc<AtomicBool>,
          input_queue: Arc<Mutex<VecDeque<[i32; 2]>>>,
          output_queue: Arc<Mutex<Vec<ChunkBuilder>>>,
          build_stats: Arc<Mutex<BuildStats>>) {
    let sleep_time = time::Duration::from_millis(500);
    'exit: while !stop.load(Ordering::Relaxed) {
        let pos_opt = match input_queue.lock() {
            Ok(mut guard) => (*guard).pop_back(),
            Err(_poisoned) => { break 'exit; }
        };
        if let Some(pos) = pos_opt {
            let mut build_start = time::Instant::now();
            let mut builder = ChunkBuilder::new(pos);
            let raw_height_map = architect.create_height_map(pos, CHUNK_SIZE, 1.);
            let mut erosion = HydraulicErosion::new(&raw_height_map, &mut rand::thread_rng());
            erosion.erode();
            let height_map = erosion.create_heightmap();
            builder.create_surface_buffer(&height_map);

            let build_time = build_start.elapsed().as_secs() as u32 * 1000 + build_start.elapsed().subsec_millis();
            match build_stats.lock() {
                Ok(mut guard) => (*guard).add_time(build_time),
                Err(_poisoned) => { break 'exit; }
            }
            match output_queue.lock() {
                Ok(mut guard) => (*guard).push(builder),
                Err(_poisoned) => { break 'exit; }
            }
        } else {
            thread::sleep(sleep_time);
        }
    }
}
