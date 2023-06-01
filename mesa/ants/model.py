from mesa import Model
from mesa.time import SimultaneousActivation
from mesa.space import MultiGrid
from agent import Pheromones, Ant, Food, Home, Wall
import random


class AntWorld(Model):
    def __init__(self, height=50, width=50, evaporate=0.1, diffusion=1, initdrop=100, lowerbound=0.01, drop_rate=0.05):
        super().__init__()
        self.evaporate = evaporate
        self.diffusion = diffusion
        self.initdrop = initdrop
        self.lowerbound = lowerbound
        self.drop_rate = drop_rate
        self.schedule = SimultaneousActivation(self)
        self.grid = MultiGrid(height, width, torus=False)
        self.running = True

        homeloc = (10, 10)
        homeloc2 = (40, 40)
        food_locs = ((11, 11), (35, 16), (36, 46))
        wall_locs = ((30, 12), (30, 13), (30, 14), (30, 15), (30, 16), (30, 17), (30, 18), (30, 19), (30, 20), (30, 21), (30, 22), (31,22), (32,22), (33,22), (34,22),
                     (35,22), (36,22), (37,22), (38,22), (39,22), (40,22))
        # wall_locs = ()

        self.home = Home(self.next_id(), homeloc, self, species_=0)
        self.home2 = Home(self.next_id(), homeloc2, self, species_=1)
        self.grid.place_agent(self.home, homeloc)
        self.grid.place_agent(self.home2, homeloc2)
        self.schedule.add(self.home)
        self.schedule.add(self.home2)

        for i in range(100):
            ant = Ant(self.next_id(), self.home, self)
            self.grid.place_agent(ant, self.home.pos)
            self.schedule.add(ant)

        for i in range(100):
            ant = Ant(self.next_id(), self.home2, self, species_=1)
            self.grid.place_agent(ant, self.home2.pos)
            self.schedule.add(ant)

        # uncomment if initial food needs to be hardcoded
        # for loc in food_locs:
        #     food = Food(self.next_id(), self)
        #     food.add(100)
        #     self.grid.place_agent(food, loc)
        #     self.schedule.add(food)

        for loc in wall_locs:
            wall = Wall(self.next_id(), loc, self)
            self.grid.place_agent(wall, loc)
            self.schedule.add(wall)

        for i in range(5):
            self.generate_food()

        for (contents, x, y) in self.grid.coord_iter():
            if (x, y) not in wall_locs and (x, y) != homeloc and (x, y) != homeloc2:
                pheromones = Pheromones(self.next_id(), (x, y), self)
                self.grid.place_agent(pheromones, (x, y))
                self.schedule.add(pheromones)

        # static field
        queue = [self.home]
        while queue:
            agent = queue.pop(0)
            if agent.visited:
                continue
            agent.visited = True
            for neighbor in self.grid.get_neighbors(agent.pos, moore=True, include_center=False):
                if isinstance(neighbor, Pheromones):
                    queue.append(neighbor)
            min_val = min([n.static_value for n in self.grid.get_neighbors(agent.pos, moore=True, include_center=False) if isinstance(n, Pheromones) or isinstance(n, Home)])
            if isinstance(agent, Pheromones):
                agent.static_value = min_val + 1

        for agent in self.schedule.agents:
            if isinstance(agent, Pheromones):
                agent.visited = False

        queue = [self.home2]
        while queue:
            agent = queue.pop(0)
            if agent.visited:
                continue
            agent.visited = True
            for neighbor in self.grid.get_neighbors(agent.pos, moore=True, include_center=False):
                if isinstance(neighbor, Pheromones):
                    queue.append(neighbor)
            min_val = min(
                [n.static_value2 for n in self.grid.get_neighbors(agent.pos, moore=True, include_center=False) if
                 isinstance(n, Pheromones) or isinstance(n, Home)])
            if isinstance(agent, Pheromones):
                agent.static_value2 = min_val + 1

        # static field visualisation
        # for x in range(self.grid.width):
        #     for y in range(self.grid.height):
        #         agent = self.grid.get_cell_list_contents([(y, x)])
        #         for a in agent:
        #             if isinstance(a, Pheromones) or isinstance(a, Home) or isinstance(a, Wall):
        #                 print(a.static_value, end=' ')
        #                 continue
        #     print()

    def step(self):
        self.schedule.step()
        if self.schedule.time % 200 == 0:
            self.generate_food()
        if self.home.amount == 1000:
            self.running = False
        if self.home2.amount == 1000:
            self.running = False

    def generate_food(self):
        pos = random.choice([(x, y) for (contents, x, y) in self.grid.coord_iter()])
        radius_ = random.randint(3, 4)
        is_valid = self.check_food_pos(pos, radius_)
        while not is_valid:
            pos = random.choice([(x, y) for (contents, x, y) in self.grid.coord_iter()])
            is_valid = self.check_food_pos(pos, radius_)
        neighbour_hoods = self.grid.get_neighborhood(pos, moore=False, radius=radius_, include_center=True)
        for loc in neighbour_hoods:
            food = Food(self.next_id(), self)
            food.add(2)
            self.grid.place_agent(food, loc)
            self.schedule.add(food)

    def check_food_pos(self, pos, radius_):
        for n in self.grid.get_neighbors(pos, moore=False, radius=radius_, include_center=True):
            if isinstance(n, Wall) or isinstance(n, Home):
                return False
        return True
