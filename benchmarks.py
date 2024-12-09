import os
import subprocess
import time
import re

PASSED_TESTS = re.compile(r"test result: ok\. ([0-9]) passed")
HERE = os.getcwd()
STAR = "⭐"
N = 100

def benchmark(dir_):
    total = 0.
    print(f"Benchmarking {dir_}...")
    subprocess.run("cargo build --release", shell=True, capture_output=True)
    for _ in range(N):
        start = time.time()
        subprocess.run("cargo run --release", shell=True, capture_output=True)
        elapsed = time.time() - start
        total += elapsed
    return total / N

def for_each_day(do_something):
    res = {}
    for dir_ in os.listdir("."):
        if not os.path.isdir(dir_) or dir_[:3] != "day":
            continue
        os.chdir(dir_)
        day = int(dir_[-2:])
        res[day] = do_something(dir_)
        os.chdir("..")
    return res

def test_solutions(dir_):
    print(f"Testing {dir_}...")
    output = str(subprocess.run("cargo test", shell=True, capture_output=True).stdout)
    return int(PASSED_TESTS.findall(output)[0])

def build_readme(benches, test_results):
    with open("README - template.md") as f:
        template = f.read()
    day_results = "\n".join(f"- Day {d} - {STAR * test_results[d]} in {benches[d]:.2f} s..." for d in sorted(benches.keys()))
    template = template.replace("{{__days__}}", day_results)
    with open("README.md", "w") as f:
        f.write(template)

if __name__ == "__main__":
    benches = for_each_day(benchmark)
    stars = for_each_day(test_solutions)
    build_readme(benches, stars)
