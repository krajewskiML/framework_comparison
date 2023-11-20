#!/bin/bash
./Ants_LinuxServerBuild.x86_64 --mapFilePath=./Maps/updated_dungeon_map_100x100.csv --iterationCount=1000 --repetitionsCount=10
./Ants_LinuxServerBuild.x86_64 --mapFilePath=./Maps/medium_dungeon_map_300x300.csv --iterationCount=10000 --repetitionsCount=1
./Ants_LinuxServerBuild.x86_64 --mapFilePath=./Maps/large_dungeon_map_500x500.csv --iterationCount=10000 --repetitionsCount=1
