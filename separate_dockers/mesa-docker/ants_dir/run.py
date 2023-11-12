import time
import subprocess
import os

REPETITIONS = 2
start = time.time()
for rep in range(REPETITIONS):
    subprocess.run(['python3', os.path.join('mrowki', 'run.py'), "100", "40000"])

end = time.time()
print((end - start)/REPETITIONS, flush=True)
