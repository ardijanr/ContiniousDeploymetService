#!/bin/sh
cargo build --release
sudo cp ./target/release/deployment_worker /bin/deploy_worker
sudo chmod 700 /bin/deploy_worker
