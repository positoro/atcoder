#!/bin/bash
problem=$1

cargo compete new $problem; find ./$problem -type d -name .git -prune -o -type d -empty -exec touch {}/.gitkeep \;
