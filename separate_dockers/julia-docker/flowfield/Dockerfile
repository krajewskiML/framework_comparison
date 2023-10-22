# Use a base Julia image from Docker Hub
FROM julia:latest

# Set the working directory inside the container
WORKDIR /app

# Install Python
RUN apt-get update && \
    apt-get install -y python3

# Copy the Julia and Python application code into the container
COPY FlowField.jl /app/FlowField.jl
COPY Main.jl /app/Main.jl
COPY test0.csv /app/test0.csv

# Install required Julia packages
RUN julia -e 'using Pkg; Pkg.add(["Agents", "Random", "CSV", "DataFrames", "Tables"])'

# Set the entry point for the container (using Julia as the entry point)
ENTRYPOINT ["julia", "--startup-file=no", "-i", "--"]

# Command to load and run the Julia program within the REPL
# The first argument (ARGV[1]) will be the input CSV file, and the second argument (ARGV[2]) will be the number of iterations
CMD ["Main.jl", "test0.csv", "100"]