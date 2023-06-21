import sys

from mpi4py import MPI
from repast4py import core, space, schedule, random, parameters
from repast4py.space import DiscretePoint
from repast4py import context as ctx
from repast4py.logging import TabularLogger

import os


class Cell(core.Agent):
    def __init__(self, x: int, y: int, state: int, local_id: int, rank: int):
        super().__init__(id=local_id, type=0, rank=rank)
        self.neighbours = [DiscretePoint(x - 1, y + 1, 0), DiscretePoint(x, y + 1, 0),
                           DiscretePoint(x + 1, y + 1, 0), DiscretePoint(x - 1, y, 0),
                           DiscretePoint(x + 1, y, 0), DiscretePoint(x - 1, y - 1, 0),
                           DiscretePoint(x, y - 1, 0), DiscretePoint(x + 1, y - 1, 0)]
        self.pt = DiscretePoint(x, y, 0)
        self.state = state
        self.next_state = state

    def set_state(self, new_state: int):
        self.state = new_state
        self.next_state = new_state

    def change_state(self):
        self.state = self.next_state

    def update(self, grid: space.SharedGrid):
        live_neighbours = 0
        for point in self.neighbours:
            agent = grid.get_agent(point)
            if agent is not None:
                live_neighbours += agent.state

        if self.state == 0 and live_neighbours == 3:
            self.next_state = 1

        elif self.state == 1:
            if live_neighbours <= 1 or live_neighbours >= 4:
                self.next_state = 0


class GameOfLife:
    def __init__(self, comm: MPI.Intracomm, width: int, height: int, stop: int, points: list):
        self.start_points = points

        self.runner = schedule.init_schedule_runner(comm)
        self.runner.schedule_repeating_event(1, 1, self.step)
        self.runner.schedule_repeating_event(1.1, 1, self.log_agents)
        self.runner.schedule_stop(stop)
        schedule.runner().schedule_end_event(self.at_end)

        self.width = width
        self.height = height

        box = space.BoundingBox(0, self.width, 0, self.height, 0, 0)
        self.grid = space.SharedGrid(name='world', bounds=box, borders=space.BorderType.Sticky,
                                     occupancy=space.OccupancyType.Single, buffer_size=1, comm=comm)

        self.context = ctx.SharedContext(comm)
        self.context.add_projection(self.grid)

        self.agent_logger = TabularLogger(comm, "output/agent_log.csv", ['x', 'y', 'state'])

        self.setup(comm)
        self.log_agents()

    def setup(self, comm: MPI.Intracomm):
        rank = comm.Get_rank()

        for x in range(self.width):
            for y in range(self.height):
                cell = Cell(x, y, 0, self.width * x + y, rank)
                self.context.add(cell)
                self.grid.move(cell, cell.pt)

        for x in range(self.width):
            for y in range(self.height):
                pt = DiscretePoint(x, y)
                cell = self.grid.get_agent(pt)
                if self.start_points[y][x] == '0':
                    cell.set_state(0)
                else:
                    cell.set_state(1)

    def step(self):
        for cell in self.context.agents():
            cell.change_state()

        for cell in self.context.agents():
            cell.update(self.grid)

    def log_agents(self):
        for cell in self.context.agents():
            self.agent_logger.log_row(cell.pt.x, cell.pt.y, cell.state)

        self.agent_logger.write()

    def at_end(self):
        self.agent_logger.close()

    def run(self):
        self.runner.execute()


if __name__ == '__main__':
    os.system('rm -rf ./output')
    with open(sys.argv[1], 'r') as f:
        file = f.read()
        lines = file.split('\n')
        height = len(lines)
        starting_points = []
        for line in lines:
            line_values = line.split(',')
            starting_points.append(line_values)
            width = len(line_values)

    model = GameOfLife(MPI.COMM_WORLD, width, height, int(sys.argv[2]),
                       starting_points)
    model.run()
