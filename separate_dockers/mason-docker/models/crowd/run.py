import time
import subprocess
import os
import sys

# MODEL_FOLDER = './models/game_of_life'
MAPS_FOLDER = './maps'
INPUT_NAMES = ['board_1_500.csv', 'board_2_500.csv', 'board_3_100.csv']
JAR_PATH = './source/crowdSim.jar'

REPETITIONS = 1
NUM_STEPS = -1  # Probably should be in a separate file since its same for all simulations

directory = os.getcwd()

for input_name in INPUT_NAMES:
    start = time.time()
    for rep in range(REPETITIONS):
        subprocess.run(
            ['java', '-jar', JAR_PATH, os.path.join(MAPS_FOLDER,  input_name), str(NUM_STEPS)])
    end = time.time()
    print((end - start) / REPETITIONS, flush=True)
