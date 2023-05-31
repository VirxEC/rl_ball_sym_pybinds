# RLBot Python bindings for rl_ball_sym

[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://github.com/rust-secure-code/safety-dance/)

[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)

Pre-built binaries for Python 3.7 and beyond in Windows & Linux can be found [here in the build artifacts for the latest workflow run](https://github.com/VirxEC/rl_ball_sym_pybinds/actions).

## Prerequisites:

+ [Rust & Cargo](https://www.rust-lang.org/)
+ [RLBot](https://rlbot.org) - Verify that the file `%localappdata%\RLBotGUIX\Python37\python.exe` exists. If it doesn't, please re-download and re-install from the website to update.
+ Maturin - Downloaded onto your main global Python installion, you can install via `pip install maturin`

## Steps to build the Python bindings

1. Download this repository
3. Run `develop.bat`
4. The package will be automatically installed into RLBot's Python installation
5. `import rl_ball_sym_pybinds` in your Python file

## Basic usage in an RLBot script to render the path prediction

See `script.cfg` and `script.py` for a pre-made script that renders the framework's ball path prediction in green and the rl_ball_sym's ball path prediction in red.

```python
from traceback import print_exc

from rlbot.agents.base_script import BaseScript
from rlbot.utils.structures.game_data_struct import GameTickPacket

import rl_ball_sym_pybinds as rlbs


class rl_ball_sym(BaseScript):
    def __init__(self):
        super().__init__("rl_ball_sym")

    def main(self):
        rlbs.load_soccer()

        while 1:
            try:
                self.packet: GameTickPacket = self.wait_game_tick_packet()
                rlbs.tick(self.packet)
                path_prediction = rlbs.get_ball_prediction_struct()

                self.renderer.begin_rendering()
                self.renderer.draw_polyline_3d(tuple(path_prediction.slices[i].location for i in range(0, path_prediction.num_slices, 4)), self.renderer.red())
                self.renderer.end_rendering()
            except Exception:
                print_exc()


if __name__ == "__main__":
    rl_ball_sym = rl_ball_sym()
    rl_ball_sym.main()
```

## Documentation

For documentation, see `rl_ball_sym_pybinds.pyi`.

## Benchmarks

Results of `pytest.py`:

![get_ball_prediction_struct takes 0.08ms to execute in soccer](https://raw.githubusercontent.com/VirxEC/rl_ball_sym_pybinds/master/rlbs_bench.png)
