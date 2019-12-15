#!/bin/sh

log_level="debug"
log_string="core=$log_level,check_erosion=$log_level,world=$log_level"
RUST_LOG=$log_string cargo run --example check_erosion
