import time
import subprocess

REPETITIONS = 1
INPUT_NAMES = ["10x10board.csv", "100x100board.csv", "1000x1000board.csv"]
ITERATIONS_PER_REPETITION = "100"

for input_name in INPUT_NAMES:
    start = time.time()
    for rep in range(REPETITIONS):
        subprocess.run(["julia", "Main.jl", input_name, ITERATIONS_PER_REPETITION])

    end = time.time()
    print((end - start) / REPETITIONS, flush=True)
