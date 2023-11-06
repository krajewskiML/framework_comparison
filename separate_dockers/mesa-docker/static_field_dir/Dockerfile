FROM python:3

COPY . /app

WORKDIR /app

# install dependencies
USER root
RUN pip install --no-cache-dir -r requirements.txt

# run the command
CMD python3 run.py