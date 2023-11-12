from mesa.visualization.modules import CanvasGrid
from mesa.visualization.ModularVisualization import ModularServer
from model import AntWorld
import sys
from visual import diffusion_portrayal


def main(map_size=100, steps=1000, web_visualization=False):
    evap_rate = 0.15
    diff_rate = 0.88
    init_drop = 100
    lower_bound = 0.01
    drop_rate = 0.5

    if web_visualization:
        canvas_element = CanvasGrid(diffusion_portrayal, map_size, map_size, 600, 600)
        model_params = {
            "height": map_size,
            "width": map_size,
            "evaporate": evap_rate,
            "diffusion": diff_rate,
            "initdrop": init_drop,
            "lowerbound": lower_bound,
            "drop_rate": drop_rate,
        }
        server = ModularServer(AntWorld, [canvas_element], "Ants", model_params)
        server.port = 8521
        server.launch()
    else:
        model = AntWorld(height=map_size,
                         width=map_size,
                         evaporate=evap_rate,
                         diffusion=diff_rate,
                         initdrop=init_drop,
                         lowerbound=lower_bound,
                         drop_rate=drop_rate)
        if steps is None or steps <= 0:
            steps = 1000
        for i in range(steps):
            model.step()


if __name__ == "__main__":
    if len(sys.argv) > 2:
        main(int(sys.argv[1]), int(sys.argv[2]), False)
    elif len(sys.argv) > 1:
        main(int(sys.argv[1]), 1000, False)
    else:
        raise ValueError("Not enough arguments, did you forget to put map size or number of steps?")