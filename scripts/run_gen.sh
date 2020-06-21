#!/bin/sh

log_level="debug"
log_string="core=$log_level,heightmap_generator=$log_level,world=$log_level"
RUST_LOG=$log_string cargo run --release --bin heightmap_generator
