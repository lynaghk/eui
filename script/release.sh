#!/bin/sh
set -euo pipefail

DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "$DIR/../"

# Kill all child processes on exit
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT

npm install --no-audit # run this to make sure local node_modules is up to date with lockfile.
rm -rf public_release/
clojure -M:shadow-cljs release frontend
