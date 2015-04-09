#!/bin/bash
cargo doc
scp -r -P 9140 target/doc/* sothr@sothr.com:/srv/www/sothr.com/projects/rust
