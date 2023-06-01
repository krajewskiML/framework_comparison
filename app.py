# we will define different functions for different frameworks
import subprocess
import time
# java
def java(jar_name: str) -> float:
    start = time.time()
    subprocess.run(['java', '-jar', 'mason/' + jar_name])
    end = time.time()
    return end - start

def java_game_of_life(jar_name: str, args: str) -> float:
    start = time.time()
    subprocess.run(['java', '-jar', 'mason/game_of_life/' + jar_name, args])
    end = time.time()
    return end - start
# netlogo
def netlogo(nlogo_name: str) -> float:
    start = time.time()
    subprocess.run(['netlogo', 'netlogo/' + nlogo_name])
    end = time.time()
    return end - start
# python
def python(python_name: str) -> float:
    start = time.time()
    subprocess.run(['python', 'mesa/ants/' + python_name])
    end = time.time()
    return end - start
# rust
# c++
# julia
def julia(julia_name: str) -> float:
    start = time.time()
    subprocess.run(['julia', 'julia/test/' + julia_name])
    end = time.time()
    return end - start

def main():
    # java_time = java('antsSimulation.jar')
    # print(f'Java time: {java_time}')
    #
    # mesa_time = python('terminal_run.py')
    # print(f'Mesa time: {mesa_time}')

    # java = java_game_of_life('game_of_life.jar', 'assets/game_of_life/plansza1.csv')
    # print(f'Java time: {java}')

    julia_time = julia('myscript.jl')

if __name__ == '__main__':
    main()