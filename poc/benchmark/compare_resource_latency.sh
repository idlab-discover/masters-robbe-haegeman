#!/bin/bash

USE_CARGO=false
if [[ "$1" == "build" ]]; then
    USE_CARGO=true
fi

if $USE_CARGO; then
    # 1 kind
    cargo run --release -- -f ./output/results_resource_latency.jsonl -r 0 -k 0 -d 30
    sleep 30
    cargo run --release -- -f ./output/results_resource_latency.jsonl -r 1 -k 1 -d 30
    sleep 30
    cargo run --release -- -f ./output/results_resource_latency.jsonl -r 5 -k 1 -d 30
    sleep 3
    cargo run --release -- -f ./output/results_resource_latency.jsonl -r 10 -k 1 -d 30
    sleep 30
    cargo run --release -- -f ./output/results_resource_latency.jsonl -r 100 -k 1 -d 30
    sleep 30
    cargo run --release -- -f ./output/results_resource_latency.jsonl -r 1000 -k 1 -d 30
else
    # 1 kind
    benchmark -f ./output/results_resource_latency.jsonl -r 0 -k 0 -d 30
    sleep 30
    benchmark -f ./output/results_resource_latency.jsonl -r 1 -k 1 -d 30
    sleep 30
    benchmark -f ./output/results_resource_latency.jsonl -r 5 -k 1 -d 30
    sleep 3
    benchmark -f ./output/results_resource_latency.jsonl -r 10 -k 1 -d 30
    sleep 30
    benchmark -f ./output/results_resource_latency.jsonl -r 100 -k 1 -d 30
    sleep 30
    benchmark -f ./output/results_resource_latency.jsonl -r 1000 -k 1 -d 30
fi
