#!/bin/bash

#  http://stackoverflow.com/a/3232082/4496839
read -r -p "delete puzzle_42? [y/N] " response
response=${response,,}    # tolower
if [[ "$response" =~ ^(yes|y)$ ]]
then
	rm -r target/debug/puzzle_42/
fi

cd target/debug/

cargo run
