FROM opensciencegrid/osgvo-ubuntu-20.04

ARG MODEL_FOLDER
ARG NETLOGO_VERSION=6.3.0
ARG NETLOGO_HOME=/opt/netlogo
ARG NETLOGO_NAME=NetLogo-$NETLOGO_VERSION

ENV LC_ALL=C.UTF-8 \
    LANG=C.UTF-8 \
    NETLOGO_TARBALL=NetLogo-$NETLOGO_VERSION-64.tgz

ENV NETLOGO_URL=https://ccl.northwestern.edu/netlogo/$NETLOGO_VERSION/$NETLOGO_TARBALL

WORKDIR /opt

RUN wget -q $NETLOGO_URL && tar xzf $NETLOGO_TARBALL && ln -sf "NetLogo $NETLOGO_VERSION" netlogo && rm -f $NETLOGO_TARBALL

USER root
RUN apt-get install -y python3 && pip3 install pandas

COPY . /opt

RUN mv /opt/models/$MODEL_FOLDER/model.nlogo /opt/models/NLModel.nlogo

CMD python3 run.py