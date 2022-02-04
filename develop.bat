@echo off
set rlpy="%localappdata%\RLBotGUIX\Python37\python.exe"
maturin build --release -i %rlpy%
$rlpy -m pip install target/wheels/*.*.*-cp37-*.whl --force-reinstall