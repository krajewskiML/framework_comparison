from mesa.visualization.modules import CanvasGrid
from mesa.visualization.ModularVisualization import ModularServer
from mesa.visualization.UserParam import UserSettableParameter

from model import AntWorld
from agent import Pheromones, Ant, Food, Home, Wall
import math


def log_norm(value, lower, upper):
    value = min(value, upper)
    value = max(value, lower)
    lower_log = math.log(lower)
    upper_log = math.log(upper)
    value_log = math.log(value)
    return (value_log - lower_log) / (upper_log - lower_log)


def diffusion_portrayal(agent):
    if agent is None:
        return
    portrayal = {}
    if isinstance(agent, Ant):
        portrayal["Shape"] = "circle"
        portrayal["Color"] = "#000000" if agent.species == 0 else "#FF0000"
        portrayal["Filled"] = "true"
        portrayal["r"] = 0.5
        portrayal["Layer"] = 4
    elif isinstance(agent, Food):
        portrayal["Shape"] = "rect"
        portrayal["Filled"] = "true"
        portrayal["w"] = 1
        portrayal["h"] = 1
        portrayal["Filled"] = "true"
        portrayal["Layer"] = 2
        portrayal["Color"] = "#ffc0cb"
    elif isinstance(agent, Home):
        portrayal["Shape"] = "rect"
        portrayal["Filled"] = "true"
        portrayal["w"] = 1
        portrayal["h"] = 1
        portrayal["Layer"] = 2
        portrayal["Color"] = "#964B00BB"
        portrayal["text"] = agent.amount
        portrayal["text_color"] = "#000000"
    elif isinstance(agent, Pheromones):
        portrayal["Shape"] = "rect"
        portrayal["Filled"] = "true"
        portrayal["Layer"] = 1
        portrayal["w"] = 1
        portrayal["h"] = 1
        val1 = int(log_norm(agent.food_pheromone, agent.model.lowerbound, agent.model.initdrop) * 255)
        val2 = int(log_norm(agent.food_pheromone2, agent.model.lowerbound, agent.model.initdrop) * 255)
        portrayal["Color"] = "#%02xFF%02x" % (255-val1, 255-val2)
    elif isinstance(agent, Wall):
        portrayal["Shape"] = "rect"
        portrayal["Filled"] = "true"
        portrayal["Layer"] = 0
        portrayal["w"] = 1
        portrayal["h"] = 1
        portrayal["Color"] = '#aaaaaa'
    return portrayal


# canvas_element = CanvasGrid(diffusion_portrayal, 50, 50, 600, 600)
#
# model_params = {
#     "height": 50,
#     "width": 50,
#     "evaporate": 0.1,
#     "diffusion": 0.8,
#     "initdrop": 100,
#     "lowerbound": 0.01,
#     "drop_rate": 0.5
# }
#
# server = ModularServer(AntWorld, [canvas_element], "Ants", model_params)
