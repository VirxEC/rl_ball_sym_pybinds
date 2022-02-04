set +v
rlpy="/usr/bin/python3.7"
maturin build --release -i $rlpy
$rlpy -m pip install target/wheels/*.*.*-cp37-*.whl --force-reinstall