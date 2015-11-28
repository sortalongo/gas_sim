#! /usr/bin/env bash

if [ $# != 1 ]; then
  echo Usage: run.sh OUTFILE
fi

OUTFILE=$1

mkdir -p tmp
cargo run | tail -n +2 > tmp/$OUTFILE.dat
gnuplot -e "infile='tmp/$OUTFILE.dat'; outfile='tmp/$OUTFILE.gif'" animate.plg

