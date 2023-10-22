FROM rust:1.70 as builder
LABEL authors="maciejkrajewskistudia"

RUN cargo new --bin game_of_life
WORKDIR ./game_of_life_dir

COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN  cargo build --release

USER root
RUN apt-get update && apt-get install -y python3

COPY ./run.py ./run.py
COPY ./maps ./maps

# run the command
CMD python3 run.py