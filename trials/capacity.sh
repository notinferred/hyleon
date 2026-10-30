#!/bin/bash
# The T-02 capacity test: the Tauri app, with its core native and inside the UI, opens ever larger
# projects until it breaks. (The Dioxus variant ran here too until D-043 retired it; see T-02.md.)
# Usage: capacity.sh <T executable> <results folder> ["<sizes>"]
#
# A variant drops out at its first size where it crashes or doesn't open within 30 s, its drag p95 is
# over 100 ms (felt as lag), or its edit p95 (every check rerun, findings redrawn) is over 1 s.
# Memory is the app plus the WebKit processes that start during its run, so leave Safari and Mail
# alone while this runs. Run it plugged in, with Low Power Mode off: it caps WebKit at 30 frames/s.
set -uo pipefail

t_exe=$1
out=$2
sizes=${3:-"1000 2000 5000 10000 20000 50000"}
mkdir -p "$out"
out=$(cd "$out" && pwd)

webkit() { pgrep -f 'com.apple.WebKit' | sort || true; }

# Each stack runs as a Mac app opened through macOS, so its window comes to the front as an app's
# does. Started from a terminal, a window can open behind it, and macOS barely draws hidden windows.
wrap() {
  local exe=$1 name=$2 app="$out/apps/$2.app"
  mkdir -p "$app/Contents/MacOS"
  cp "$exe" "$app/Contents/MacOS/$name"
  cat > "$app/Contents/Info.plist" << PLIST
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0"><dict>
<key>CFBundleExecutable</key><string>$name</string>
<key>CFBundleIdentifier</key><string>com.example.hyleon-$name</string>
<key>CFBundleName</key><string>$name</string>
<key>CFBundlePackageType</key><string>APPL</string>
<key>NSHighResolutionCapable</key><true/>
</dict></plist>
PLIST
  echo "$app"
}
t_app=$(wrap "$t_exe" trial-t)

# run <name> <app> <arguments...>: one run, stopped after 2 minutes; writes <name>.memory.json.
run() {
  local name=$1 app=$2
  shift 2
  local before helpers rss pid peak=0 ticks=0
  before=$(webkit)
  open -n "$app" --args "$@"
  sleep 0.5
  pid=$(pgrep -n -f "$app/Contents/MacOS/")
  while kill -0 "$pid" 2>/dev/null && ((ticks < 240)); do
    helpers=$(comm -13 <(echo "$before") <(webkit) | paste -sd, -)
    rss=$(ps -o rss= -p "$pid${helpers:+,$helpers}" 2>/dev/null | awk '{s += $1} END {print s + 0}')
    ((rss > peak)) && peak=$rss
    sleep 0.5
    ticks=$((ticks + 1))
  done
  [[ -n $pid ]] && kill "$pid" 2>/dev/null
  echo "{\"peakMemoryMB\": $((peak / 1024))}" > "$out/$name.memory.json"
}

# verdict <results.json>: "ok", or why the stack broke at this size.
verdict() {
  python3 -I -c '
import json, sys
try:
    r = json.load(open(sys.argv[1]))
except Exception:
    print("crashed or hung (no results)"); sys.exit()
drag = r.get("drag", {}).get("latency", {}).get("p95")
edit = r.get("edit", {}).get("latency", {}).get("p95")
if "open" not in r: print("did not open within 30 s")
elif drag is None or drag > 100: print(f"drag p95 {drag} ms")
elif edit is None or edit > 1000: print(f"edit p95 {edit} ms")
else: print("ok")
' "$1"
}

broken=" "
for parts in $sizes; do
  for variant in T-wasm T-native; do
    [[ $broken == *" $variant "* ]] && continue
    file="$out/$variant-$parts.json"
    case $variant in
      T-wasm) run "$variant-$parts" "$t_app" --size "$parts" --core wasm --capacity --bench "$file" ;;
      T-native) run "$variant-$parts" "$t_app" --size "$parts" --core native --capacity --bench "$file" ;;
    esac
    result=$(verdict "$file")
    echo "$variant at $parts parts: $result"
    [[ $result != ok ]] && broken="$broken$variant "
  done
  [[ $broken == *" T-wasm "*" T-native "* || $broken == *" T-native "*" T-wasm "* ]] && break
done
echo "done; broke:${broken}"
