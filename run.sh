#!/bin/sh

# To execute this you need: CAP_NETADM

cargo b
sudo setcap cap_net_admin=eip ./target/debug/Tcp
./target/debug/Tcp
