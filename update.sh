#!/usr/bin/env bash
set -x
set -e

# NOTE: Last executed using Rust 1.40.0

cargo install --force --version 0.17.0 svd2rust
cargo install --force --version 0.7.0  form
rustup component add rustfmt

rm -rf src
mkdir src
svd2rust -i svd/rp2040.svd 
form -i lib.rs -o src
rm lib.rs
# Original svd has \n (two chars) in it, whch gets converted to "\n" and "\\n" by svd2rust
# If we convert them to newline characters in the SVD, they don't turn up in markdown so online docs suffer
# So, convert both \\n and \n to [spc] [spc] [newline], then strip the spaces out if there are consecutive [newlines]
# This means that by the time we're in docs.rs markdown, \n\n becomes new paragraph, and \n becomes a new line
find src -name '*.rs' -exec sed -i -e 's/\\\\n/\\n/g' -e 's/\\n/  \n/g' -e 's/  \n  \n/\n\n/g' {} \;
cargo fmt
rustfmt build.rs
