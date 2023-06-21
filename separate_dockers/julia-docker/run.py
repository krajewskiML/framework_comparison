import time
import subprocess
import os

MODEL_FOLDER = "maps"
REPETITIONS = 5
INPUT_NAMES = ['10x10board.csv', '100x100board.csv', '1000x1000board.csv']
for input_name in INPUT_NAMES:
    start = time.time()
    for rep in range(REPETITIONS):
        subprocess.run(['julia', os.path.join('Agentsjl', 'gameoflife', 'src', 'Main.jl'),
                        os.path.join(MODEL_FOLDER, input_name), "100"])

    end = time.time()
    print((end - start)/REPETITIONS, flush=True)