#!/bin/sh
clear
cross build --target arm-unknown-linux-gnueabihf
if [ $? -eq 0 ]; then
	scp target/arm-unknown-linux-gnueabihf/debug/robot_diff_drive debian@192.168.7.2:bin/
	if [ $? -eq 0 ]; then
		ssh debian@192.168.7.2 sudo bin/robot_diff_drive | tee run_on_bb.log
		python graph.py
	fi
fi
