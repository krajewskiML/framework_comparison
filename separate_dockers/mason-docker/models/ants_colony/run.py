import time
import subprocess
import os
import sys

JAR_PATH = './source/antsSimulation.jar'

REPETITIONS = 1
NUM_STEPS = 100  # Probably should be in a separate file since its same for all simulations

directory = os.getcwd()

start = time.time()
for rep in range(REPETITIONS):
    subprocess.run(
        ['java', '-jar', JAR_PATH, str(NUM_STEPS)])
end = time.time()
print((end - start) / REPETITIONS, flush=True)
