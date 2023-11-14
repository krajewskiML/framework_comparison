import time
import subprocess
import os
import sys

JAR_PATH = './source/antsSimulation.jar'

REPETITIONS = 1
MAP_SIZES = [100, 300, 500]

directory = os.getcwd()

start = time.time()

for size in MAP_SIZES:
    start = time.time()
    for rep in range(REPETITIONS):
        subprocess.run(
            ['java', '-jar', JAR_PATH, str(size)])
    end = time.time()
    print((end - start) / REPETITIONS, flush=True)
