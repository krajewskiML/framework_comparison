#!/usr/bin/env python3
import time
import subprocess
import os

MAPS_FOLDER = './maps'
INPUT_NAMES = ['board_1_500.csv', 'board_2_500.csv', 'board_3_100.csv']
BIN_PATH = './target/release/static_field'

REPETITIONS = 10

directory = os.getcwd()
os.chmod(BIN_PATH,  0b111101101)

for input_name in INPUT_NAMES:
    start = time.time()
    print([BIN_PATH, "-f", os.path.join(MAPS_FOLDER,  input_name)])
    for rep in range(REPETITIONS):
        subprocess.call(
            [BIN_PATH, "-f", os.path.join(MAPS_FOLDER,  input_name)]
        )
    end = time.time()
    # print((end - start) / REPETITIONS, flush=True)
    print(f"input: {input_name}, rep: {REPETITIONS}")
    # print("1 iteration time:",(end - start)/(REPETITIONS*NUM_STEPS))
    print("Total time: ", (end - start))
    print("---")