#!/bin/sh
set -eux

gptee --help
gptee -h
gptee --version
gptee -V

echo Tell me a joke | gptee

gptee <(echo Tell me a joke)

echo Tell me a joke | gptee -m text-davinci-003

echo Give me just a macOS zsh command to get the free space on my hard drive \
| gptee -s "Prefix each line of output with a pound sign if it not meant to be executed"

echo "Tell me I'm pretty" | gptee -s "You only speak French"
