#!/bin/bash
set -euo pipefail

DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
cd "$DIR/../"

# Kill all child processes on exit
trap "trap - SIGTERM && kill -- -$$" SIGINT SIGTERM EXIT


export EUI_DEV=true


# Start watching rust
fd ".rs" | entr -crs "cargo run --example demo" &

cd cljs_frontend/

# Start watching CSS
mkdir -p public_dev/js/ public_dev/css/
sass --watch src/style.sass public_dev/style.css &


npm install --no-audit # run this to make sure local node_modules is up to date with lockfile.


clojure -M:shadow-cljs watch frontend
