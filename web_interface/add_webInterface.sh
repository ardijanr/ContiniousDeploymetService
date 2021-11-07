#!/bin/sh
cargo build --release
sudo cp ./target/release/web_interface /home/deployer/cd_service/
sudo chown deployer /home/deployer/cd_service/web_interface 
