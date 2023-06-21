FROM julia:latest

RUN apt-get install -y python3

COPY . /app

WORKDIR /app

USER root
# install python3 (it is debian based)
RUN apt-get update && apt-get install python3

CMD python3 run.py