import time
import subprocess
import os

MODEL_FOLDER = os.path.join("static_field", "maps")
REPETITIONS = 2
INPUT_NAMES = ['board_1_500.csv', 'board_2_500.csv', 'board_3_100.csv']
# INPUT_NAMES = ['10x10board.csv', '100x100board.csv']
for input_name in INPUT_NAMES:
    start = time.time()
    for rep in range(REPETITIONS):
        subprocess.run(['python3', os.path.join('static_field', 'visual.py'),
                        os.path.join(MODEL_FOLDER, input_name)])

    end = time.time()
    print((end - start)/REPETITIONS, flush=True)
