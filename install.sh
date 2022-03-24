#!/bin/bash

if [ -f string_convert ]; then
  cp string_convert /usr/local/bin/
else
  echo "No 'string_convert' binary in current directory"
fi