#!/bin/bash

cd gen || exit

for i in 25 50 250 500; do
	for j in $(seq 1 100); do
		cargo run -- $i >../data/random/$i/"$j".col
	done
done
