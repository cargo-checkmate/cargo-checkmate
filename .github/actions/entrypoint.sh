#!/bin/bash
while [ $# -gt 0 ]
do
  echo "Arg: $1"
  shift
done

find /github -type d

exit 1
