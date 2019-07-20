#!/bin/bash
if [ $# -ne 1 ]; then
    echo "usage: ${0} contest_name" 1>&2
    exit 1
fi

CONTEST=$1
cp -R template $CONTEST
pushd $CONTEST
#cp a.rs b.rs
