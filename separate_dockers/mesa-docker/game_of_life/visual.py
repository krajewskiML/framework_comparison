from mesa.visualization.modules import CanvasGrid
from mesa.visualization.ModularVisualization import ModularServer
from gol_model import *
import numpy as np
import sys
import pandas as pd


def agent_portrayal(agent):
    if agent.alive:
        color = "green"
    else:
        color = "white"
    portrayal = {"Shape": "circle",
                 "Filled": "true",
                 "Layer": 0,
                 "Color": color,
                 "r": 0.5}
    return portrayal


def main(game_map_=None, steps_=None):
    game_map_ = np.random.randint(0, 2, (20, 20), dtype=np.int8) if game_map_ is None else game_map_
    width = game_map_.shape[0]
    height = game_map_.shape[1]
    if steps_ is None:
        grid = CanvasGrid(agent_portrayal, width, height, 500, 500)
        server = ModularServer(GameModel,
                               [grid],
                               "Game of Life", {"width": width, "height": height, "game_map": game_map_})
        server.port = 8521
        server.launch()
    else:
        model = GameModel(width, height, game_map_)
        for i in range(steps_):
            model.step()


if __name__ == "__main__":
    if len(sys.argv) > 2:
        df = pd.read_csv(sys.argv[1], header=None)
        game_map = df.values
        main(game_map, int(sys.argv[2]))
    elif len(sys.argv) > 1:
        df = pd.read_csv(sys.argv[1], header=None)
        game_map = df.values
        main(game_map)
    else:
        main()
