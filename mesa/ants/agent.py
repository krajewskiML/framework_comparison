from mesa import Agent
import random


def encode_angle(pos1, pos2):
    x1, y1 = pos1
    x2, y2 = pos2
    dx = x2 - x1
    dy = y2 - y1
    if dx == 0:
        if dy == -1:
            return 4
        elif dy == 1:
            return 0
        else:
            return None
    elif dx == 1:
        if dy == -1:
            return 5
        elif dy == 0:
            return 6
        elif dy == 1:
            return 7
        else:
            return None
    elif dx == -1:
        if dy == -1:
            return 3
        elif dy == 0:
            return 2
        elif dy == 1:
            return 1
        else:
            return None
    else:
        return None


def get_possible_next_moves(angle, curr_pos, neighbourhood_):
    if angle is None:
        return neighbourhood_
    wanted_angles = [angle, (angle + 1) % 8, (angle - 1) % 8]
    possible_next_moves = list(filter(lambda pos: encode_angle(curr_pos, pos) in wanted_angles, neighbourhood_))
    return possible_next_moves


class Pheromones(Agent):
    def __init__(self, unique_id, pos, model):
        super().__init__(unique_id, model)
        self.pos = pos
        self.food_pheromone = 0.0
        self._next_food_pheromone = 0.0
        self.food_pheromone2 = 0.0
        self._next_food_pheromone2 = 0.0
        self.static_value = 1000000
        self.static_value2 = 1000000
        self.visited = False

    def get_pheromones_total(self, type_=0):
        if type_ == 0:
            food_pheromone_total = self.food_pheromone
        else:
            food_pheromone_total = self.food_pheromone2
        neighbours = [n for n in self.model.grid.get_neighbors(self.pos, True) if isinstance(n, Pheromones)]
        for n in neighbours:
            if type_ == 0:
                food_pheromone_total += n.food_pheromone
            else:
                food_pheromone_total += n.food_pheromone2
        food_pheromone_total = food_pheromone_total / (len(neighbours) + 1)
        return food_pheromone_total

    def step(self):
        food_avg = self.get_pheromones_total()
        food_avg2 = self.get_pheromones_total(1)
        self._next_food_pheromone = (1 - self.model.evaporate)*(self.food_pheromone + (self.model.diffusion * (food_avg-self.food_pheromone)))
        self._next_food_pheromone2 = (1 - self.model.evaporate)*(self.food_pheromone2 + (self.model.diffusion * (food_avg2-self.food_pheromone2)))

        if self._next_food_pheromone < self.model.lowerbound:
            self._next_food_pheromone = 0

        if self._next_food_pheromone2 < self.model.lowerbound:
            self._next_food_pheromone2 = 0

    def advance(self):
        self.food_pheromone = self._next_food_pheromone
        self.food_pheromone2 = self._next_food_pheromone2

    def add(self, food_pheromone_, type_=0):
        if type_ == 0:
            self.food_pheromone += food_pheromone_
        else:
            self.food_pheromone2 += food_pheromone_

    def get_pos(self):
        return self.pos


class Home(Agent):
    def __init__(self, unique_id, pos, model, species_=0):
        super().__init__(unique_id, model)
        self.pos = pos
        self.amount = 0
        self.static_value = 0 if species_ == 0 else 1000000
        self.static_value2 = 0 if species_ == 1 else 1000000
        self.visited = False
        self.species = species_

    def add(self, amount):
        self.amount += amount

    def get_pos(self):
        return self.pos


class Food(Agent):
    def __init__(self, unique_id, model):
        super().__init__(unique_id, model)
        self.amount = 0

    def add(self, amount):
        self.amount += amount

    def eaten(self):
        if self.any_food():
            self.amount -= 1
        if not self.any_food():
            self.model.grid.remove_agent(self)

    def any_food(self):
        return self.amount > 0

    def get_pos(self):
        return self.pos


class Ant(Agent):
    def __init__(self, unique_id, home, model, moore=True, species_=0):
        super().__init__(unique_id, model)
        self.pos = home.pos
        self.state = "FORAGING"
        self.drop = 0
        self.home_drop = 1
        self.home = home
        self.moore = moore
        self.angle = random.randint(0, 7)
        self.next_angle = 0
        self.next_pos = (0, 0)
        self.species = species_

    def get_item(self, wanted_class):
        for agent in self.model.grid.get_cell_list_contents([self.pos]):
            if isinstance(agent, wanted_class):
                return agent

    def step(self):
        if self.state == "FORAGING":
            food = self.get_item(Food)
            if food is not None and food.any_food():
                food.eaten()
                self.state = "HOMING"
                self.drop = self.model.initdrop
            else:
                self.gradient_move()
        else:
            home = self.get_item(Home)
            if home is not None:
                if home.species == self.species:
                    home = self.get_item(Home)
                    home.add(1)
                    self.state = "FORAGING"
                    self.drop = 0
            else:
                self.drop_food_pheromone()
                self.home_move()

    def drop_food_pheromone(self):
        env = self.get_item(Pheromones)
        env.add(self.drop, self.species)
        self.drop *= self.model.drop_rate

    def random_move(self):
        next_moves = [n.get_pos() for n in self.model.grid.get_neighbors(self.pos, self.moore) if isinstance(n, Pheromones)]
        poss_moves = get_possible_next_moves(self.angle, self.pos, next_moves)
        if len(poss_moves) == 0:
            poss_moves = next_moves
        self.next_pos = random.choice(poss_moves)

    def home_move(self):
        neighbors = [n for n in self.model.grid.get_neighbors(self.pos, self.moore) if isinstance(n, Pheromones) or isinstance(n, Home)]
        if self.species == 0:
            min_dist = min([neigh.static_value for neigh in neighbors])
            min_dist_neighs = [neigh for neigh in neighbors if neigh.static_value == min_dist]
        else:
            min_dist = min([neigh.static_value2 for neigh in neighbors])
            min_dist_neighs = [neigh for neigh in neighbors if neigh.static_value2 == min_dist]
        self.next_pos = random.choice(min_dist_neighs).get_pos()

    def gradient_move(self):
        where = (-1, -1)
        max = self.model.lowerbound
        neighbors = [n for n in self.model.grid.get_neighbors(self.pos, moore=self.moore) if isinstance(n, Pheromones) or isinstance(n, Food)]
        for n in neighbors:
            if isinstance(n, Food) and n.any_food():
                where = n.get_pos()
                break
            if isinstance(n, Pheromones):
                if self.species == 0:
                    if n.food_pheromone > max:
                        max = n.food_pheromone
                        where = n.get_pos()
                else:
                    if n.food_pheromone2 > max:
                        max = n.food_pheromone2
                        where = n.get_pos()
                    # and n.food_pheromone > max:
                    # max = n.food_pheromone
                    # where = n.get_pos()
        if where != (-1, -1):
            self.next_pos = where
        else:
            self.random_move()

    def advance(self) -> None:
        self.next_angle = encode_angle(self.pos, self.next_pos)
        self.model.grid.move_agent(self, self.next_pos)
        self.pos = self.next_pos
        self.angle = self.next_angle


class Wall(Agent):
    def __init__(self, unique_id, pos, model):
        super().__init__(unique_id, model)
        self.pos = pos
        self.static_value = 1000000

    def get_pos(self):
        return self.pos

