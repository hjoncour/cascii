#!/bin/bash
set -euxo pipefail

cargo build

./target/debug/casci ./tests/video/input/test.mkv ./tests/video/output/small --small
./target/debug/casci ./tests/video/input/test.mkv ./tests/video/output/default --default
./target/debug/casci ./tests/video/input/test.mkv ./tests/video/output/large --large
