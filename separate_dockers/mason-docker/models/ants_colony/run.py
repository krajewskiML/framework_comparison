import time
import subprocess
import os
import sys

JAR_PATH = './source/golSimulation.jar'

REPETITIONS = 3
MAP_SIZES = [100, 300, 500]

directory = os.getcwd()

start = time.time()

steps = 10

for size in MAP_SIZES:
    start = time.time()
    for rep in range(REPETITIONS):
        subprocess.run(
            ['java', '-jar', JAR_PATH, str(size), str(steps)])
    end = time.time()
    print((end - start) / REPETITIONS, flush=True)
