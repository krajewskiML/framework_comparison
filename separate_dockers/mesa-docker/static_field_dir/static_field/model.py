from mesa import Model
from mesa.time import BaseScheduler
from mesa.space import MultiGrid
from agent import FieldAgent


class FieldModel(Model):

    def __init__(self, height, width, map_):
        self.num_agents = width * height
        self.height = height
        self.width = width
        self.grid = MultiGrid(height, width, True)
        self.schedule = BaseScheduler(self)
        self.running = True
        self.no_of_agents = 0
        self.done = False
        for i in range(width):
            for j in range(height):
                agent = FieldAgent(i * height + j, self, map_[i][j])
                self.grid.place_agent(agent, (j, i))
                self.schedule.add(agent)
                if map_[i][j] == 3:
                    self.no_of_agents += 1

        for agent in self.schedule.agents:
            agent.clear()

        to_check = []
        for agent in self.schedule.agents:
            if agent.pointType == 2:
                agent.staticField = 0
                to_check.extend(agent.getNeighbors())

        while len(to_check) > 0:
            if to_check[0].calcStaticField() == True:
                to_check.extend(to_check[0].getNeighbors())
            to_check.pop(0)

    def step(self):
        for agent in self.schedule.agents:
            agent.isBlocked = False
        self.schedule.step()
        if self.no_of_agents == 0:
            self.running = False
            self.done = True
