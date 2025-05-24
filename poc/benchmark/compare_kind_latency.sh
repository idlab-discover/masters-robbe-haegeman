#!/bin/bash

USE_CARGO=false
if [[ "$1" == "build" ]]; then
    USE_CARGO=true
fi

if $USE_CARGO; then
    cargo run --release -- -f ./results_kind_latency.jsonl -r 100 -k 1 -d 30
    sleep 45
    cargo run --release -- -f ./results_kind_latency.jsonl -r 100 -k 2 -d 30
    sleep 45
    cargo run --release -- -f ./results_kind_latency.jsonl -r 100 -k 3 -d 30
    sleep 45
    cargo run --release -- -f ./results_kind_latency.jsonl -r 100 -k 4 -d 30
    sleep 45
    cargo run --release -- -f ./results_kind_latency.jsonl -r 100 -k 5 -d 30

else
    benchmark -f ./results_kind_latency.jsonl -r 100 -k 1 -d 30
    sleep 45
    benchmark -f ./results_kind_latency.jsonl -r 100 -k 2 -d 30
    sleep 45
    benchmark -f ./results_kind_latency.jsonl -r 100 -k 3 -d 30
    sleep 45
    benchmark -f ./results_kind_latency.jsonl -r 100 -k 4 -d 30
    sleep 45
    benchmark -f ./results_kind_latency.jsonl -r 100 -k 5 -d 30
fi
