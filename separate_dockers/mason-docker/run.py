import time
import subprocess
import os
import sys

MODEL_FOLDER = 'models/game_of_life'
REPETITIONS = 1
INPUT_NAMES = ['10x10board.csv', '100x100board.csv', '1000x1000board.csv']

directory = os.getcwd()


for input_name in INPUT_NAMES:
    start = time.time()
    for rep in range(REPETITIONS):
        subprocess.run(['java', '-jar', 'antsSimulation.jar',
                        os.path.join(MODEL_FOLDER, 'maps', input_name)])

    end = time.time()
    print((end - start)/REPETITIONS, flush=True)