#!/bin/sh

###
# NOT TO BE USED DIRECTLY. SEE async.sh.
#
# This script exists so we can run setsid on commands with automated logging statements around them.
# In async.sh, I couldn't get something like `setsid {logging...} $@ {logging...}` to work, so here we are.
###

echo "---------------------------------------------------------------------------"
echo "--- Executing asynchronously '$@'"
echo "--- Run 'kill -9 $$' to stop"
echo "---------------------------------------------------------------------------"
echo

# Delegation at its finest (don't laugh at me)
$@

echo
echo "---------------------------------------------------------------------------"
echo "--- Done executing '$@'"
echo "---------------------------------------------------------------------------"
