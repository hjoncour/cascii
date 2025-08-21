#!/bin/bash
set -euxo pipefail

find ./tests/video/output -type f -name "*.png" -delete

cargo build

./target/debug/casci ./tests/video/input/test.mkv ./tests/video/output/small --small
./target/debug/casci ./tests/video/input/test.mkv ./tests/video/output/default --default
./target/debug/casci ./tests/video/input/test.mkv ./tests/video/output/large --large

compare_dirs() {
    local dir1=$1
    local dir2=$2
    local size1=$(du -sb "$dir1" | awk '{print $1}')
    local size2=$(du -sb "$dir2" | awk '{print $1}')

    if [ "$size1" -eq "$size2" ]; then
        echo "Size check passed for $dir1 and $dir2"
    else
        echo "Size check failed for $dir1 and $dir2: $size1 vs $size2"
        exit 1
    fi
}

compare_dirs ./tests/video/output/small/expected ./tests/video/output/small/frame_images
compare_dirs ./tests/video/output/default/expected ./tests/video/output/default/frame_images
compare_dirs ./tests/video/output/large/expected ./tests/video/output/large/frame_images

