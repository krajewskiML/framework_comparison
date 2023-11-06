from mesa import Agent


class FieldAgent(Agent):
    """
    Agent for the field.
    0 - empty
    1 - wall
    2 - exit
    3 - agent
    """
    def __init__(self, unique_id, model, pointType_):
        super().__init__(unique_id, model)
        self.pos = (0, 0)
        self.pointType = pointType_
        self.staticField = 100000
        self.isBlocked = False

    def clear(self):
        self.staticField = 100000

    def calcStaticField(self):
        if self.pointType == 1:
            return False
        neighbors = self.model.grid.get_neighbors(self.pos, moore=True, include_center=False)
        neigh = [n.staticField for n in neighbors]
        if len(neigh) > 0:
            neighMin = min(neigh)
            if self.staticField > neighMin+1:
                self.staticField = neighMin+1
                return True
        return False

    def step(self):
        if self.pointType == 3 and not self.isBlocked:
            neighbors = self.model.grid.get_neighbors(self.pos, moore=True, include_center=False)
            neigh = [n for n in neighbors if not n.isBlocked and (n.pointType == 0 or n.pointType == 2)]
            if len(neigh) > 0:
                next_neigh = neigh[0]
                minValue = neigh[0].staticField
                for n in neigh:
                    if n.staticField < minValue:
                        minValue = n.staticField
                        next_neigh = n
                if next_neigh.pointType == 2:
                    self.pointType = 0
                    next_neigh.isBlocked = True
                    self.model.no_of_agents -= 1
                else:
                    self.pointType = 0
                    next_neigh.pointType = 3
                    next_neigh.isBlocked = True

    def getNeighbors(self):
        neighbors = self.model.grid.get_neighbors(self.pos, moore=True, include_center=False)
        return neighbors

