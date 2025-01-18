@echo off
powershell -Command "Start-Process cmd.exe -ArgumentList '/c cd /d C:\S2O\s2o_net_lib && cargo run -- admin && exit' -Verb RunAs"