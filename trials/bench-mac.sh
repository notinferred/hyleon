#!/bin/bash
# Runs the T-02 comparison on a Mac: every variant at every size, three times, recording peak memory.
# Usage: bench-mac.sh <D executable> <T executable> <results folder>
#
# Memory is the app's process plus the WebKit processes that start during its run. WebKit runs pages
# in separate helper processes, so leave other WebKit apps (Safari, Mail) alone while this runs.
set -euo pipefail

d_exe=$1
t_exe=$2
out=$3
mkdir -p "$out"
test -x "$d_exe" && test -x "$t_exe"

webkit() { pgrep -f 'com.apple.WebKit' | sort || true; }

# run <name> <command...>: runs one bench and writes <name>.memory.json beside its results.
run() {
  local name=$1
  shift
  local before
  before=$(webkit)
  "$@" > /dev/null 2>&1 &
  local pid=$! peak=0 waited=0
  while kill -0 "$pid" 2>/dev/null && ((waited < 800)); do
    local helpers rss
    helpers=$(comm -13 <(echo "$before") <(webkit) | paste -sd, -)
    rss=$(ps -o rss= -p "$pid${helpers:+,$helpers}" 2>/dev/null | awk '{s += $1} END {print s + 0}')
    ((rss > peak)) && peak=$rss
    sleep 0.5
    waited=$((waited + 1))
  done
  kill "$pid" 2>/dev/null || true
  echo "{\"peakMemoryMB\": $((peak / 1024))}" > "$out/$name.memory.json"
  echo "$name: peak memory $((peak / 1024)) MB"
}

for i in 1 2 3; do
  for size in panel building; do
    run "D-mac-$size-$i" "$d_exe" --size "$size" --bench "$out/D-mac-$size-$i.json"
    run "T-native-mac-$size-$i" "$t_exe" --size "$size" --core native --bench "$out/T-native-mac-$size-$i.json"
    run "T-wasm-mac-$size-$i" "$t_exe" --size "$size" --core wasm --bench "$out/T-wasm-mac-$size-$i.json"
  done
done
