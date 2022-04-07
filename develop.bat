@echo off
set rlpy="%localappdata%\RLBotGUIX\Python37\python.exe"
maturin build --release -i %rlpy%
%rlpy% -m pip install rl_ball_sym_pybinds --find-links=target\wheels --force-reinstall