#!/bin/bash

CURRENT_FOLDER=random
SIZE=500

for folder in ./data/*; do
	if [[ "$folder" != ./data/$CURRENT_FOLDER ]]; then
		continue
	fi
	for file in "$folder"/$SIZE/*; do
		./target/release/gcp-heuristics -p "$file" -a genetic --generations 80000 --population-size 100 --offspring-size 2 --mutation-probaility 0.01 --population-ratio 0.2
	done
done
