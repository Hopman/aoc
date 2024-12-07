#!/bin/bash

if [ -z $1 ] ; then
    echo "Need a number for the day!"
    echo "Exiting..."
    exit 1
fi

cp src/dayx.rs "src/day$1.rs"

echo "Please provide example input:"
read example
echo "$example" > "input/day$1_example.txt"

echo "Please provide puzzle input:"
read puzzle
echo "$puzzle" > "input/day$1.txt"

echo "Done..."
