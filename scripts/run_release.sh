#!/bin/sh

log_level="debug"
log_string="core=$log_level,world_gen=$log_level,world=$log_level"
RUST_LOG=$log_string cargo run --release
