#!/usr/bin/env bash
set -x
set -e

rm docs -rf
cargo doc
mv target/doc docs
echo '<html><head><meta http-equiv="refresh" content="0; url=./rp2040_pac/index.html"></head><body></body></html>' > docs/index.html
