#!/bin/bash

# Get list of examples
examples=($(ls examples/*.rs | xargs -n 1 basename | sed 's/\.rs//'))

echo "Select an example to flash:"
for i in "${!examples[@]}"; do 
    echo "$((i+1)). ${examples[$i]}"
done

read -p "Enter number: " choice
index=$((choice-1))

if [ -n "${examples[$index]}" ]; then
    selected="${examples[$index]}"
    echo "Flashing $selected..."
    cargo run --example "$selected"
else
    echo "Invalid selection"
fi
