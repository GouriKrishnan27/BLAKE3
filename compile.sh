#!/bin/bash
# ProgramBench-style build: must produce ./executable at the repo root.
set -e
cd "$(dirname "$0")"
# The CLI is the standalone b3sum crate (not a member of the root workspace).
(cd b3sum && cargo build --release)
cp b3sum/target/release/b3sum executable
