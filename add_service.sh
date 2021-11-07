#!/bin/sh
sudo systemctl stop cd_service
sudo systemctl disable cd_service

sudo cp ./cd_service.service /etc/systemd/system/
sudo systemctl start cd_service
sudo systemctl enable cd_service
sudo systemctl status cd_service
