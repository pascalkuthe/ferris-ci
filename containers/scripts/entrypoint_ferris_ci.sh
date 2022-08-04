#!/bin/bash
set -e
export MANPATH=""
source /opt/rh/rh-python38/enable
exec "$@"
