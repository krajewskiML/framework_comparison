import time
import subprocess
import os

MODEL_FOLDER = os.path.join("game_of_life", "maps")
REPETITIONS = 2
# INPUT_NAMES = ['10x10board.csv', '100x100board.csv', '1000x1000board.csv']
INPUT_NAMES = ['1000x1000board.csv']
# INPUT_NAMES = ['10x10board.csv', '100x100board.csv']
for input_name in INPUT_NAMES:
    start = time.time()
    for rep in range(REPETITIONS):
        subprocess.run(['python3', os.path.join('game_of_life', 'visual.py'),
                        os.path.join(MODEL_FOLDER, input_name), "100"])

    end = time.time()
    print((end - start)/REPETITIONS, flush=True)
