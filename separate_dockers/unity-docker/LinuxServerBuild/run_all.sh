#!/bin/bash
./LinuxServerBuild.x86_64 --mapFilePath=./MockMaps/10x10board.csv --iterationCount=10000 --repetitionsCount=10000
./LinuxServerBuild.x86_64 --mapFilePath=./MockMaps/100x100board.csv --iterationCount=1000 --repetitionsCount=1000
./LinuxServerBuild.x86_64 --mapFilePath=./MockMaps/1000x1000board.csv --iterationCount=1000 --repetitionsCount=10
