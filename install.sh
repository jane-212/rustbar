#!/bin/bash

cargo build --release

sudo cp -f ./target/release/rustbar /bin/bar

