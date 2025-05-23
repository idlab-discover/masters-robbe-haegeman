#!/bin/bash

# 1 kind
cargo run --release -- -f ./results_kind_latency.jsonl -r 100 -k 1 -d 30
sleep 45
cargo run --release -- -f ./results_kind_latency.jsonl -r 100 -k 2 -d 30
sleep 45
cargo run --release -- -f ./results_kind_latency.jsonl -r 100 -k 3 -d 30
sleep 45
cargo run --release -- -f ./results_kind_latency.jsonl -r 100 -k 4 -d 30
sleep 45
cargo run --release -- -f ./results_kind_latency.jsonl -r 100 -k 5 -d 30
