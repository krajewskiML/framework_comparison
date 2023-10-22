module FlowField

using Agents, Random, CSV, DataFrames

macro MAX()
    return :(100000)
end

@enum Type begin
    floor = 0
    wall = 1
    exit = 2
    pedestrian = 3
end

@agent Cell GridAgent{2} begin
    cellType::Type
    staticField::Int
    isBlocked::Bool
end

function calcStaticField(agent, model)
    neis = map(x -> (x.staticField, x), nearby_agents(agent, model))
    if length(neis) > 0
        minField, _ = findmin(first, neis)
        if agent.staticField > minField + 1
            agent.staticField = minField + 1
            return true
        end
    end
    return false
end

function calcBoardField(model)
    toCheck = []
    for agent in allagents(model)
        if agent.cellType == exit
            agent.staticField = 0
            for nei in nearby_agents(agent, model)
                push!(toCheck, nei)
            end
        end
    end

    while !isempty(toCheck)
        if calcStaticField(toCheck[1], model)
            for nei in nearby_agents(toCheck[1], model)
                push!(toCheck, nei)
            end
        end
        popfirst!(toCheck)
    end
end

function m_step!(model)
    for agent in allagents(model)
        agent.isBlocked = false
    end

    for agent in allagents(model)
        if agent.cellType == pedestrian && !agent.isBlocked
            freeNeis = collect(Iterators.filter(x -> (!x.isBlocked && (x.cellType == exit || x.cellType == floor)), nearby_agents(agent, model)))

            if length(freeNeis) > 0
                _, minId = findmin(first, map(x -> (x.staticField, x), freeNeis))
                minNei = freeNeis[minId]

                if minNei.cellType == exit
                    agent.cellType = floor
                    minNei.isBlocked = true
                else
                    agent.cellType = floor
                    minNei.cellType = pedestrian
                    minNei.isBlocked = true
                end
            end

        end
    end
end

function init(matrix)
    xdims, ydims = size(matrix)
    board = GridSpace((xdims, ydims); metric=:chebyshev, periodic=false)
    model = ABM(Cell, board)
    id = 1
    for i in 1:xdims
        for j in 1:ydims
            add_agent_pos!(Cell(id, (i, ydims - j + 1), Type(matrix[j, i]), @MAX, false), model)
            id += 1
        end
    end
    calcBoardField(model)
    return model
end

end