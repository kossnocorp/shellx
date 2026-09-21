#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
heading 'DEC special graphics and G0/G1 character sets' 'ASCII l q k / x / m j become box-drawing glyphs in DEC graphics mode.'
defer $'\017\e(B\e)B\e*B\e+B'
emit '\e(0'; printf 'lqqqqqqk\nx      x\nmqqqqqqj\n'; emit '\e(B'
note 'Same through G1: ESC ) 0, SO selects G1, SI selects G0.'
emit '\e)0\016'; printf 'lqqqqqqk\nx      x\nmqqqqqqj'; emit '\017\e)B'; printf '\n'
note 'Single-shift G2/G3: ESC N / ESC O affect the next character only.'
emit '\e*0\e+0\eNq \eOq'; printf ' <- two horizontal line glyphs\n'
note 'Unicode equivalent: ┌──────┐ │ └──────┘'
