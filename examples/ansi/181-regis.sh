#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
screen
heading 'ReGIS vector graphics: DCS 0 p ... ST' 'Legacy DEC graphics, rarely supported. Expected: rectangle with a diagonal.'
emit '\eP0pS(E)P[100,100]V[300,100][300,250][100,250][100,100][300,250]\e\\'
