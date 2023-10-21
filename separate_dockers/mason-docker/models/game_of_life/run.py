import time
import subprocess
import os
import sys

# MODEL_FOLDER = './models/game_of_life'
MAPS_FOLDER = './maps'
INPUT_NAMES = ['10x10board.csv']
# INPUT_NAMES = ['10x10board.csv', '100x100board.csv', '1000x1000board.csv']
JAR_PATH = './source/golSimulation.jar'

REPETITIONS = 10000
NUM_STEPS = 10000  # Probably should be in a separate file since its same for all simulations

directory = os.getcwd()

for input_name in INPUT_NAMES:
    start = time.time()
    for rep in range(REPETITIONS):
        subprocess.run(
            ['java', '-jar', JAR_PATH, os.path.join(MAPS_FOLDER,  input_name), str(NUM_STEPS)])
    end = time.time()
    # print((end - start) / REPETITIONS, flush=True)
    print("input: {}, rep: {}, iterations: {}".format(input, REPETITIONS, NUM_STEPS))
    print("1 iteration time:",(end - start)/(REPETITIONS*NUM_STEPS))
    print("Total time: ", (end - start))
    print("---")
