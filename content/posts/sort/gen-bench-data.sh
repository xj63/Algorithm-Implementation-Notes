#!/bin/bash

commands=(
  gen-gaussian-with-noise
  gen-high-sample-sin-with-noise
  gen-low-sample-sin-with-noise
  gen-random
  gen-stroll
  gen-trend-increasing
)

cmd_pids=()

mkdir -p ./bench-data

for cmd in "${commands[@]}"; do
  output_file="./bench-data/${cmd#gen-}.json"
  cargo run --bin="$cmd" >"$output_file" 2>/dev/null || exit 1 &
  cmd_pids+=($!)
done

for pid in "${cmd_pids[@]}"; do
  wait "$pid" || exit 101
done
