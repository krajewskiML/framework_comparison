#!/usr/bin/env python3
import time
import subprocess
import os

BIN_PATH = './target/release/ants'

REPETITIONS = 2
MAP_SIZES = ["100", "300", "500"]
STEPS = ["40000", "10000", "3000"]

directory = os.getcwd()
os.chmod(BIN_PATH,  0b111101101)

for map_size, steps in zip(MAP_SIZES, STEPS):
    start = time.time()
    for rep in range(REPETITIONS):
        subprocess.call(
            [BIN_PATH, "-s", map_size, "-t", steps]
        )
    end = time.time()
    print(f"map size: {map_size}, rep: {REPETITIONS}, iterations: {steps}", flush=True)
    print("Total time: ", (end - start), flush=True)
    print("---", flush=True)