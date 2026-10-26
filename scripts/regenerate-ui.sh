#!/usr/bin/env sh
set -eu
cd "$(dirname "$0")/.."
exec blueprint-compiler compile --output data/window.ui data/window.blp
