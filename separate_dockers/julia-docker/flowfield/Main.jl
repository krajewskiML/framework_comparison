include("FlowField.jl")
using .FlowField: init, dummystep, m_step!, run!
using CSV, Tables

input_csv = ARGS[1]
iterations = parse(Int64, ARGS[2])

init_board = CSV.File(input_csv, header = false) |> Tables.matrix

flow_field = init(init_board)

run!(flow_field, dummystep, m_step!, iterations)

exit()

# visualisation
#=
function color(a) 
    if a.cellType == wall return :maroon
    elseif a.cellType == exit return :olive
    elseif a.cellType == pedestrian return :steelblue4
    else return  return :floralwhite
    end
end 

figure, ax, abmobs = abmplot(flow_field; agent_step! = dummystep, model_step! = m_step!, ac = color, am = :rect)
figure
=#