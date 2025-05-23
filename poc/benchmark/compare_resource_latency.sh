#!/bin/bash

# 1 kind
cargo run --release -- -f ./results_resource_latency.jsonl -r 0 -k 0 -d 30
sleep 30
cargo run --release -- -f ./results_resource_latency.jsonl -r 1 -k 1 -d 30
sleep 30
cargo run --release -- -f ./results_resource_latency.jsonl -r 5 -k 1 -d 30
sleep 30
cargo run --release -- -f ./results_resource_latency.jsonl -r 10 -k 1 -d 30
sleep 30
cargo run --release -- -f ./results_resource_latency.jsonl -r 100 -k 1 -d 30
sleep 30
cargo run --release -- -f ./results_resource_latency.jsonl -r 1000 -k 1 -d 30
