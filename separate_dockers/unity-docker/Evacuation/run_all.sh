#!/bin/bash
./Evacuation_LinuxServerBuild.x86_64 --mapFilePath=./EvacuationMaps/board_1_500.csv --repetitionsCount=1
./Evacuation_LinuxServerBuild.x86_64 --mapFilePath=./EvacuationMaps/board_2_500.csv --repetitionsCount=10
./Evacuation_LinuxServerBuild.x86_64 --mapFilePath=./EvacuationMaps/board_3_100.csv --repetitionsCount=100
