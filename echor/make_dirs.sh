#!/bin/bash

# Directory to check
DIR="tests/expected"

# Check if directory exists
if [ ! -d "$DIR" ]; then
  # Create directory if it doesn't exist
  mkdir -p "$DIR"
  echo "Directory $DIR created."
fi

echo "Hello there" > $DIR/hello1.txt
echo "Hello"  "there" > $DIR/hello2.txt
echo -n "Hello  there" > $DIR/hello1.n.txt
echo -n "Hello"  "there" > $DIR/hello2.n.txt