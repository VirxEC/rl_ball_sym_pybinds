@echo off

set PYTHON_SYS_EXECUTABLE=%localappdata%\RLBotGUIX\Python37\python.exe

cargo build

copy .\target\debug\rl_ball_sym_pybinds.dll .\rl_ball_sym.pyd /Y

echo.

%PYTHON_SYS_EXECUTABLE% test.py

pause
