#!/bin/bash

if [ -z $1 ] ; then
    echo "Need a number for the day!"
    echo "Exiting..."
    exit 1
fi

cp src/dayx.rs "src/day$1.rs"

sed -i "s/DAYX/day$1/g" "src/day$1.rs"

touch "input/day$1_example.txt"

touch "input/day$1.txt"

echo "Done..."
