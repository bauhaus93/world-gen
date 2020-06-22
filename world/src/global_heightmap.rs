use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::{btree_map::Entry, BTreeMap};
use std::fs::File;
use std::io;
use std::io::{Read, BufReader};
use std::ops::{Index, IndexMut};
use std::path::Path;
use std::time::Instant;

use crate::chunk::chunk_size::CHUNK_SIZE;
use crate::noise::Noise;
use core::{Point2i, Point2f};

pub struct GlobalHeightmap {
    fallback: Fallback,
    map: BTreeMap<Point2i, Sector>,
    sector_size: Point2i,
}

struct Sector {
    last_access: Instant,
    size: Point2i,
    height_list: Vec<f32>,
}

enum Fallback {
    File(BufReader<File>, Point2i),
    Noise(Box<dyn Noise>),
}

impl GlobalHeightmap {
    pub fn from_file(filepath: &Path) -> Result<Self, io::Error> {
        let mut reader = BufReader::new(File::open(filepath)?);
        let size = Point2i::new(
            reader.read_i32::<LittleEndian>()?,
            reader.read_i32::<LittleEndian>()?,
        );
        let sector_size = Point2i::new(
            reader.read_i32::<LittleEndian>()?,
            reader.read_i32::<LittleEndian>()?,
        );

        Ok(Self {
            fallback: Fallback::File(reader, size),
            map: BTreeMap::new(),
            sector_size: sector_size,
        })
    }

    pub fn from_noise(noise: Box<dyn Noise>) -> Self {
        let size = Point2i::from(noise.get_cycle());
        Self {
            fallback: Fallback::Noise(noise),
            map: BTreeMap::new(),
            sector_size: Point2i::from_scalar(CHUNK_SIZE),
        }
    }

    pub fn get(&mut self, p: Point2i) -> f32 {
        let sector_pos = get_sector(p, self.sector_size);
        self.map
            .entry(sector_pos)
            .or_insert(self.load_sector(sector_pos))[Point2i::new(0, 0)]
    }

    fn load_sector(&mut self, sector_pos: Point2i) -> Sector {
        let sector = Sector::new(self.sector_size);

        sector
    }

    fn get_from_fallback(&mut self, sector_pos: Point2i) -> f32 {
        0.
    }
}

impl Sector {
    pub fn new(size: Point2i) -> Self {
        Self {
            last_access: Instant::now(),
            size: size,
            height_list: Vec::with_capacity((size[0] * size[1]) as usize),
        }
    }

    fn fill_from_reader(&mut self, reader: &mut impl Read, sector_pos: Point2i) {}

    fn fill_from_noise(&mut self, noise: &dyn Noise, sector_pos: Point2i) {
        for y in 0..self.size[1] {
            for x in 0..self.size[0] {
                let local_pos = Point2i::new(x, y);
                let global_pos = Point2f::from(local_pos + sector_pos * self.size);
                self[local_pos] = noise.get_noise(global_pos);
            }
        }
    }
}

impl Index<Point2i> for Sector {
    type Output = f32;

    fn index(&self, pos: Point2i) -> &Self::Output {
        &self.height_list[(pos[0] + self.size[0] * pos[1]) as usize]
    }
}

impl IndexMut<Point2i> for Sector {
    fn index_mut(&mut self, pos: Point2i) -> &mut Self::Output {
        &mut self.height_list[(pos[0] + self.size[0] * pos[1]) as usize]
    }
}

fn get_sector(global: Point2i, sector_size: Point2i) -> Point2i {
    global / sector_size
}

fn get_index(global: Point2i, sector_size: Point2i) -> usize {
    let x_rel = global[0] % sector_size[0];
    let y_rel = global[1] % sector_size[1];
    debug_assert!(x_rel >= 0);
    debug_assert!(y_rel >= 0);
    (y_rel * sector_size[0] + x_rel) as usize
}
