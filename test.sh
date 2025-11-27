#!/usr/bin/env bash

colors=("\e[31m" "\e[32m" "\e[33m" "\e[34m" "\e[35m")
RESET="\e[0m"

for i in {1..10}; do
	color=${colors[$(((i - 1) % ${#colors[@]}))]}
	echo -e "${color}This is message #$i${RESET}"
	sleep 1
done
