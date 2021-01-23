#!/usr/bin/env bash
set -x
set -e

rm docs -rf
cargo doc --target-dir docs
echo '<html><head><meta http-equiv="refresh" content="0; url=./rp2040_pac/index.html"></head><body></body></html>' > docs/index.html