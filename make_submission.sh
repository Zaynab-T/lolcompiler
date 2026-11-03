#!/bin/bash
# Builds the Mac executable + rustdoc and assembles the Blackboard zip.
set -e
cd "$(dirname "$0")"

cargo test --quiet
cargo build --release
cargo doc --no-deps --document-private-items

rm -rf submission
mkdir -p submission/design submission/src submission/bin submission/test submission/docs
cp readme.txt submission/
cp -R design/. submission/design/
cp -R src/. submission/src/
cp target/release/lolcompiler submission/bin/
cp -R test/. submission/test/
rm -f submission/test/*.html
cp -R target/doc/. submission/docs/

rm -f COSC455_Project1_Tabassi.zip
(cd submission && zip -qr ../COSC455_Project1_Tabassi.zip .)
echo "Done: COSC455_Project1_Tabassi.zip"
