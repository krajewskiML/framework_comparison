include("GameOfLife.jl")
using .GameOfLife, Agents, Random, CSV, Tables

input_csv = ARGS[1]
iterations = parse(Int64, ARGS[2])


model = GameOfLife.init(matrix=CSV.File(input_csv, header=false) |> Tables.matrix);

run!(model, dummystep, GameOfLife.m_step!, iterations)

exit()