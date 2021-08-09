#!/bin/bash

#cat workplace/cargo-profiler-callgrind.txt | while read line
cat $1 | while read line
do
	echo $line
done
