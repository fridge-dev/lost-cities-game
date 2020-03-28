#!/bin/sh

###
# Run any **script** asynchronously with built in logging! Runs the async command in a new process group, 
# so it won't be terminated if your SSH session ends. How cool!
#
# EXAMPLE: async.sh my-script.sh -arg1 foo -arg2 bar
#
# Can't successfully run arbitrary shell code nor aliases asynchronously. Probably won't be motivated to
# figure it out. Example failure: `async.sh ll`
###

if [ $# -lt 1 ]; then
    echo "Provide some script (with optional args) to run"
    exit 1
fi

LOG=/tmp/async-helper-$(date '+%Y%m%d-%s').log

# setsid is really cool.
setsid async-log-wrapper.sh "$@" &> $LOG < /dev/null &

echo -e "\n\t> tail -f $LOG\n"
tail -f $LOG
