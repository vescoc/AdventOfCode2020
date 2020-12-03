#!/bin/bash

for d in day[0-2][0-9]; do (cd $d; cargo $*); done
