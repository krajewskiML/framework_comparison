# Use a base Julia image from Docker Hub
FROM julia:latest

# Set the working directory inside the container
WORKDIR /app

RUN apt-get update && \
    apt-get install -y python3 python3-pip 


COPY . /app

# Install required Julia packages
RUN julia -e 'using Pkg; Pkg.add(["Agents", "Random", "CSV", "Tables"])'

CMD python3 run.py
