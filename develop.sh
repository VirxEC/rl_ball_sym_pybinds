set +v
maturin build --release -i "/usr/bin/python3.7"
source ~/.RLBotGUI/env/bin/activate
pip install target/wheels/*.*.*-cp37-*.whl --force-reinstall