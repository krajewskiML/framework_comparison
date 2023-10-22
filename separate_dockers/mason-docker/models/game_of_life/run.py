import time
import subprocess
import os
import sys

# MODEL_FOLDER = './models/game_of_life'
MAPS_FOLDER = './maps'
INPUT_NAMES = ['10x10board.csv', '100x100board.csv', '1000x1000board.csv']
JAR_PATH = './source/golSimulation.jar'

REPETITIONS = 1
NUM_STEPS = 100  # Probably should be in a separate file since its same for all simulations

directory = os.getcwd()

for input_name in INPUT_NAMES:
    start = time.time()
    for rep in range(REPETITIONS):
        subprocess.run(
            ['java', '-jar', JAR_PATH, os.path.join(MAPS_FOLDER,  input_name), str(NUM_STEPS)])
    end = time.time()
    print((end - start) / REPETITIONS, flush=True)
