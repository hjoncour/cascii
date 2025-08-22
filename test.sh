#!/bin/bash
set -euo pipefail

cargo build

./target/debug/casci ./tests/video/input/test.mkv ./tests/video/output/small --small
./target/debug/casci ./tests/video/input/test.mkv ./tests/video/output/default --default
./target/debug/casci ./tests/video/input/test.mkv ./tests/video/output/large --large

find ./tests/video/output -type f -name "*.png" -delete

# Clean up expected directories
find ./tests/video/output -type f -name "details.md" -delete
find ./tests/video/output -type f -name ".DS_Store" -delete

tests_failed=0

run_comparison() {
    local type=$1
    local expected_dir=./tests/video/output/$type/expected
    local actual_dir=./tests/video/output/$type/frame_images

    printf "\n--- Comparisons for %s ---\n" "$type"
    
    local table_header="Check,Expected,Actual,Result"
    local table_body=""

    # Compare total size
    local size1=$(du -s "$expected_dir" | awk '{print $1}')
    local size2=$(du -s "$actual_dir" | awk '{print $1}')
    local result="PASSED"
    if [ "$size1" -ne "$size2" ]; then
        result="FAILED"
        tests_failed=$((tests_failed + 1))
    fi
    table_body+="Total Size (blocks),$size1,$size2,$result\n"

    # Compare file count
    local count1=$(find "$expected_dir" -type f -name "frame_*.txt" | wc -l | tr -d ' ')
    local count2=$(find "$actual_dir" -type f -name "frame_*.txt" | wc -l | tr -d ' ')
    result="PASSED"
    if [ "$count1" -ne "$count2" ]; then
        result="FAILED"
        tests_failed=$((tests_failed + 1))
    fi
    table_body+="File Count,$count1,$count2,$result\n"

    # Compare first frame dimensions
    local first_frame1=$expected_dir/frame_0001.txt
    local first_frame2=$actual_dir/frame_0001.txt
    
    local height1=$(wc -l < "$first_frame1" | tr -d ' ')
    local height2=$(wc -l < "$first_frame2" | tr -d ' ')
    result="PASSED"
    if [ "$height1" -ne "$height2" ]; then
        result="FAILED"
        tests_failed=$((tests_failed + 1))
    fi
    table_body+="Frame Height (lines),$height1,$height2,$result\n"

    local width1=$(head -n 1 "$first_frame1" | wc -c | tr -d ' ')
    local width2=$(head -n 1 "$first_frame2" | wc -c | tr -d ' ')
    result="PASSED"
    if [ "$width1" -ne "$width2" ]; then
        result="FAILED"
        tests_failed=$((tests_failed + 1))
    fi
    table_body+="Frame Width (chars),$width1,$width2,$result\n"

    (echo "$table_header"; echo -e "$table_body") | column -t -s ','
}

run_comparison small
run_comparison default
run_comparison large

if [ "$tests_failed" -ne 0 ]; then
    printf "\n%s test(s) failed.\n" "$tests_failed"
    exit 1
fi

printf "\nAll tests passed!\n"

