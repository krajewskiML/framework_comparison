FROM openjdk:11
LABEL authors="BartoszKruczek"

COPY . /app

WORKDIR /app

USER root
RUN apt-get install -y python3

# run the command
CMD python3 run.py
