#! /usr/bin/env bash

if [ $# != 1 ]; then
  echo Usage: run.sh OUTFILE
  exit 1
fi

OUTFILE=$1

mkdir -p tmp
cargo run -q >tmp/$OUTFILE.dat 2>tmp/$OUTFILE.log
gnuplot -e "infile='tmp/$OUTFILE.dat'; outfile='tmp/$OUTFILE.gif'" animate.plg 2>/dev/null

echo "Output in tmp/$OUTFILE.(gif|dat|log)"

