#!/usr/bin/env bash
source "$(dirname -- "${BASH_SOURCE[0]}")/_lib.sh"
source "$(dirname -- "${BASH_SOURCE[0]}")/_image.sh"
screen
heading 'Kitty graphics: APC = ESC _; G introduces graphics commands' 'Transmit PNG (a=t), place (a=p), delete (a=d). Expected: checkerboard.'
defer $'\e_Ga=d,d=I,i=424242,q=2\e\\'
printf '\033_Ga=t,f=100,i=424242,q=2;%s\033\\' "$image_png"
at 5 1; emit '\e_Ga=p,i=424242,c=16,r=8,q=2\e\\'
at 15 1
note 'Image is deleted when you leave this demo.'
