from mesa.visualization.modules import CanvasGrid
from mesa.visualization.ModularVisualization import ModularServer
from model import FieldModel
import pandas as pd
import sys

def agent_portrayal(agent):
    if agent.pointType == 0:
        color = "white"
    elif agent.pointType == 1:
        color = "black"
    elif agent.pointType == 2:
        color = "red"
    elif agent.pointType == 3:
        color = "blue"
    portrayal = {"Shape": "circle",
                 "Filled": "true",
                 "Layer": 0,
                 "Color": color,
                 "r": 0.5}
    return portrayal


def main(map_='./board_3_100.csv', steps=100, web_visualization=False):
    game_map_df = pd.read_csv(map_, header=None)
    game_map = game_map_df.values
    width = game_map.shape[0]
    height = game_map.shape[1]

    if web_visualization:
        grid = CanvasGrid(agent_portrayal, width, height, width*5, height*5)
        server = ModularServer(FieldModel,
                               [grid],
                               "Field flow",
                               {"width": width, "height": height, 'map': game_map})
        server.port = 8521
        server.launch()
    else:
        model = FieldModel(width, height, game_map)
        if steps is None or steps <= 0:
            steps = 100000
        for i in range(steps):
            model.step()
            if model.done:
                print("done in {} steps".format(i))
                break


if __name__ == "__main__":
    if len(sys.argv) > 2:
        main(sys.argv[1], int(sys.argv[2]), False)
    elif len(sys.argv) > 1:
        main(sys.argv[1], 100, False)
    else:
        raise ValueError("Not enough arguments, did you forget to put map file or steps?")
