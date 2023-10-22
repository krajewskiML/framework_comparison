module GameOfLife

using Agents, Random

@enum State begin
    alive = true
    dead = false
end

@agent Cell GridAgent{2} begin
    curr_state::State
    next_state::State
end

function init(; matrix, rdead=[2, 3], ralive=[3])
    xdims, ydims = size(matrix)
    properties = Dict(
        :rdead => rdead, 
        :ralive => ralive)
    board = GridSpace((xdims, ydims); metric=:chebyshev, periodic=false)
    model = ABM(Cell, board; properties)
    id = 1
    for i in 1:xdims
        for j in 1:ydims
            add_agent_pos!(Cell(id, (i, j), State(!iszero(matrix[j, i])), State(!iszero(matrix[j, i]))), model)
            id += 1
        end
    end
    return model
end

function m_step!(model)
    for agent in allagents(model)
        alive_nei = 0
        for nei in nearby_agents(agent, model)
            if nei.curr_state == alive
                alive_nei += 1
            end
        end

        if (agent.curr_state == alive && alive_nei in model.rdead) || (agent.curr_state == dead && alive_nei in model.ralive)
            agent.next_state = alive
        else
            agent.next_state = dead
        end
    end

    for agent in allagents(model)
        agent.curr_state = agent.next_state
    end
end

end