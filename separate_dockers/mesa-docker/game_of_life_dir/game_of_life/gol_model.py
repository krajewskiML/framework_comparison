from mesa import Agent, Model
from mesa.time import SimultaneousActivation
from mesa.space import MultiGrid
from mesa.datacollection import DataCollector
import random


class GameAgent(Agent):
    def __init__(self, unique_id, model):
        super().__init__(unique_id, model)
        self.alive = bool(random.getrandbits(1))
        self.next_alive = None
        self.model.grid.torus = False

    def step(self):
        neighbors = self.model.grid.get_neighbors(self.pos, moore=True, include_center=False)
        alive_count = 0
        for neighbor in neighbors:
            if neighbor.alive:
                alive_count += 1

        if self.alive:
            if alive_count < 2 or alive_count > 3:
                self.next_alive = False
        else:
            if alive_count == 3:
                self.next_alive = True

    def advance(self):
        self.alive = self.next_alive


class GameModel(Model):
    def __init__(self, width, height, game_map=None):
        self.num_agents = width * height
        self.grid = MultiGrid(width, height, True)
        self.schedule = SimultaneousActivation(self)
        self.running = True

        for i in range(width):
            for j in range(height):
                agent = GameAgent(i * height + j, self)
                self.grid.place_agent(agent, (i, j))
                self.schedule.add(agent)
        if game_map is not None:
            for i in range(width):
                for j in range(height):
                    self.schedule.agents[i * height + j].alive = bool(game_map[i][j])

    def step(self):
        self.schedule.step()
