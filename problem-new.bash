#!/bin/bash
problem=$1

cargo compete new $problem; find ./$problem -type d -name .git -prune -o -type d -empty -exec touch {}/.gitkeep \;
cp my_modules/modules.rs ./$problem/src/bin
cp -r my_modules/modules ./$problem/src/bin/modules
