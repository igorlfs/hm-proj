#!/bin/bash

CURRENT_FOLDER=reg

for folder in ./data/*; do
	if [[ "$folder" != ./data/$CURRENT_FOLDER ]]; then
		continue
	fi
	for file in "$folder"/*; do
		for GIter in 5 15 25; do
			for CIter in 5 15 25; do
				for CSize in 3 6 9; do
					./target/release/gcp-heuristics -p "$file" -a grasp --grasp-iterations $GIter --color-iterations $CIter --color-list-size $CSize
				done
			done
		done
	done
done
