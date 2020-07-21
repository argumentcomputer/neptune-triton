#!/bin/bash
set -e

cd library

# Regenerate code with explicit futhark command.
futhark opencl --safe --library -o neptune-triton/lib/a poseidon.fut
futhark --version > neptune-triton/futhark-version.txt

# Grep will return exit code 1 if there is no match, and this will fail the job.
git status | grep 'nothing to commit, working tree clean'
