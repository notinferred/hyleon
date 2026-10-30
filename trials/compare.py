# Prints the T-02 comparison table from bench results: python3 -I compare.py <results files...>
# Results of the same variant, platform and size (repeat runs) are combined by their median.
import json
import re
import statistics
import sys
from collections import defaultdict

ROWS = [
    ("Open (ms)", lambda r: r.get("open", {}).get("ms")),
    ("Drag p50", lambda r: r.get("drag", {}).get("latency", {}).get("p50")),
    ("Drag p95", lambda r: r.get("drag", {}).get("latency", {}).get("p95")),
    ("Drag dropped frames", lambda r: r.get("drag", {}).get("frames", {}).get("dropped")),
    ("Pan p50", lambda r: r.get("pan", {}).get("latency", {}).get("p50")),
    ("Pan p95", lambda r: r.get("pan", {}).get("latency", {}).get("p95")),
    ("Zoom p50", lambda r: r.get("zoom", {}).get("latency", {}).get("p50")),
    ("Edit + checks p50", lambda r: r.get("edit", {}).get("latency", {}).get("p50")),
    ("Edit + checks p95", lambda r: r.get("edit", {}).get("latency", {}).get("p95")),
    ("Next page p50", lambda r: r.get("pages", {}).get("latency", {}).get("p50")),
    ("All pages at once (ms)", lambda r: r.get("overview", {}).get("ms")),
    ("List filter p50", lambda r: r.get("list", {}).get("latency", {}).get("p50")),
    ("Reload p50", lambda r: r.get("reload", {}).get("latency", {}).get("p50")),
    ("Peak memory (MB)", lambda r: r.get("peakMemoryMB")),
    ("Errors", lambda r: len(r.get("errors", []))),
]

groups = defaultdict(list)
for path in sys.argv[1:]:
    if path.endswith(".memory.json"):
        continue
    with open(path) as f:
        result = json.load(f)
    memory = re.sub(r"\.json$", ".memory.json", path)
    try:
        with open(memory) as f:
            result.update(json.load(f))
    except FileNotFoundError:
        pass
    name = re.sub(r"-\d+\.json$|\.json$", "", path.split("/")[-1])
    groups[name].append(result)

columns = sorted(groups)
print("| Scenario | " + " | ".join(f"{c} ({len(groups[c])} run{'s' if len(groups[c]) > 1 else ''})" for c in columns) + " |")
print("|---" * (len(columns) + 1) + "|")
for label, get in ROWS:
    cells = []
    for c in columns:
        values = [v for v in (get(r) for r in groups[c]) if isinstance(v, (int, float))]
        cells.append(f"{statistics.median(values):g}" if values else "-")
    print(f"| {label} | " + " | ".join(cells) + " |")
