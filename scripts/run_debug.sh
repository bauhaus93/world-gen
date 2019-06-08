#!/bin/sh

log_level="trace"
log_string="main=$log_level,application=$log_level,utility=$log_level,world_gen=$log_level,graphics=$log_level"
RUST_LOG=$log_string cargo run