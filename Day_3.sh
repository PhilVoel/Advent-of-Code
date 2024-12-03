#!/usr/bin/env bash
if [[ $1 != "--part2" ]]; then
	cat inputs/3.txt
else
	tr -d \\n < inputs/3.txt |
	perl -pe "s/don't\(\).+?(do\(\)|$)//g"
fi |
grep -o 'mul([0-9]\{1,3\},[0-9]\{1,3\})' |
sed -r 's/mul\((.*),(.*)\)/\1*\2/' |
while read exp; do
	echo $((exp));done |
awk '{s+=$1} END {print s}'
