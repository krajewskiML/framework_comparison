FROM python:3

# Install OpenJDK-11
RUN apt-get update && \
    apt-get install -y openjdk-11-jre-headless && \
    apt-get clean;

# Install Netlogo
RUN wget http://ccl.northwestern.edu/netlogo/6.0.4/NetLogo-6.0.4-64.tgz && \
    tar -xzf NetLogo-6.0.4-64.tgz;

# Install Julia
RUN wget https://julialang-s3.julialang.org/bin/linux/x64/1.7/julia-1.7.0-linux-x86_64.tar.gz && \
    tar tar -xvzf julia-1.7.0-linux-x86_64.tar.gz && \
    sudo cp -r julia-1.7.0 /opt/ && \
    sudo ln -s /opt/julia-1.7.0/bin/julia /usr/local/bin/julia
COPY . /app

# Install Python dependencies
RUN pip install -r /app/mesa/requirements.txt

WORKDIR /app
CMD ["python", "app.py"]
