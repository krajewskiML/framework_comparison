import time
import subprocess
import os

REPETITIONS = 2
MAP_SIZES = ["100", "300", "500"]
STEPS = ["40000", "10000", "3000"]

for map_size, steps in zip(MAP_SIZES, STEPS):
    start = time.time()
    for rep in range(REPETITIONS):
        subprocess.run(['python3', os.path.join('mrowki', 'run.py'), map_size, steps])
    end = time.time()
    print((end - start)/REPETITIONS, flush=True)
