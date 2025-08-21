#!/bin/bash
set -euxo pipefail

find ./tests/video/output -type f -name "*.png" -delete

cargo build

./target/debug/casci ./tests/video/input/test.mkv ./tests/video/output/small --small
./target/debug/casci ./tests/video/input/test.mkv ./tests/video/output/default --default
./target/debug/casci ./tests/video/input/test.mkv ./tests/video/output/large --large
