import glob
import os
import re
import subprocess
import sys
import time

#  python full_bench.py rust
#  python full_bench.py rust_release
kind = sys.argv[-1]

if kind == "rust":
    subprocess.run(["cargo", "build"])
    cmd = [os.path.abspath("target/debug/cg_astar.exe")]
elif kind == "rust_release":
    subprocess.run(["cargo", "build", "--release"])
    cmd = [os.path.abspath("target/release/cg_astar.exe")]
else:
    print("Invalid argument")
    sys.exit(1)

TIME_LIMIT = 1.0
SCORE_RE = re.compile(r"All runs best Score:\s*(-?\d+)")

test_files = sorted(glob.glob("tests/*.txt"))

total_score = 0
for i, test_file in enumerate(test_files):
    name = os.path.basename(test_file)
    start_time = time.time()

    result = subprocess.run(cmd, stdin=open(test_file, "r"), capture_output=True, text=True)
    end_time = time.time()

    match = SCORE_RE.search(result.stderr)
    time_limit_exceeded = end_time - start_time >= TIME_LIMIT

    if match is None:
        print(f"Test {i + 1} ({name}): 'FAIL' - no score found")
        continue

    score = int(match.group(1))
    total_score += score

    if time_limit_exceeded:
        print(f"Test {i + 1} ({name}): 'TIMEOUT' ({end_time - start_time:0.3f}s) - {score} pts")
    else:
        print(f"Test {i + 1} ({name}): 'OK' ({end_time - start_time:0.3f}s) - {score} pts")

print(f"\n{len(test_files)} tests passed - Total score: {total_score} pts")
