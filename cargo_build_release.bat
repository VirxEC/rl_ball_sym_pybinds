@echo off

set PYTHON_SYS_EXECUTABLE=%localappdata%\RLBotGUIX\Python37\python.exe

cargo build --release

copy .\target\release\rl_ball_sym_pybinds.dll .\rl_ball_sym.pyd /Y

echo.

%PYTHON_SYS_EXECUTABLE% test.py

pause
