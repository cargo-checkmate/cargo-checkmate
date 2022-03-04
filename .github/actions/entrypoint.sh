#!/bin/bash
while [ $# -gt 0 ]
do
  echo "Arg: $1"
  shift
done
exit 1
