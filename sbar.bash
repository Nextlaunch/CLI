#!/bin/bash

# Uses SIXELs to print a completion bar on standard out. The second parameter
# (FULL) is the total number of pixels on the inside of the bar. The first
# parameter (PART) is the number of pixels that are filled in. It is not a
# percentage, don't fall into the trap of thinking that it is.
#
# To use this script, you'll need to be executing the command on a terminal or
# terminal emulator that supports SIXEL graphics. If you're on a *nix system
# with X11, `xterm -ti vt340` should work. Or you can add this line to your
# ~/.Xresources file:
#
#    XTerm*decTerminalID: vt340

PART=${1:-20}
FULL=${2:-24}

if [ "$PART" -lt 0 -o "$FULL" -lt 1 ] ; then
  echo "`basename $0` $PART $FULL : non-positive values? are you high?" 1>&2
  exit 1
fi

if [ "$PART" -gt "$FULL" ] ; then
  PART=$FULL
fi

# Undocumented code to stop XTerm from adding a CRLF after rendering
echo -n -e "\033[?8452h"

# Start SIXEL mode
echo -n -e "\033Pq"

# Set color registers to WHITE and GREEN
echo -n -e "#0;2;99;99;99#1;2;0;99;0"

# Render 3 lines, 6 pixels high of graph data
echo -n -e "#0?o!${FULL}Oo\$#1??!${PART}_-"
echo -n -e "#0?~!${FULL}?~\$#1??!${PART}~-"
echo -n -e "#0?B!${FULL}AB\$#1??!${PART}@-"

# Exit SIXEL mode
echo -n -e "\033\\"
