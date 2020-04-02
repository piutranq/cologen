#!/usr/bin/env bash

declare -r TEMPLATE="./template"
declare -r OUTPUT="./output"
declare -r SCHEME="./scheme.yaml"

cat $TEMPLATE | cologen $SCHEME > $OUTPUT

