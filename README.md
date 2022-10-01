# RLBot Python bindings for rl_ball_sym 1.0.7

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
        rlbs.load_soccar()

        while 1:
            try:
                self.packet: GameTickPacket = self.wait_game_tick_packet()

                rlbs.tick(packet)

                path_prediction = rlbs.get_ball_prediction_struct()

                self.renderer.begin_rendering()
                self.renderer.draw_polyline_3d(tuple((path_prediction["slices"][i]["location"][0], path_prediction["slices"][i]["location"][1], path_prediction["slices"][i]["location"][2]) for i in range(0, path_prediction["num_slices"], 4)), self.renderer.red())
                self.renderer.end_rendering()
            except Exception:
                print_exc()


if __name__ == "__main__":
    rl_ball_sym = rl_ball_sym()
    rl_ball_sym.main()

```

## Example ball prediction struct

### Normal

```python
[
    {
        "time": 0.008333,
        "location": [
            -2283.9,
            1683.8,
            323.4,
        ],
        "velocity": [
            1273.4,
            -39.7,
            757.6,
        ]
    },
    {
        "time": 0.025,
        "location": [
            -2262.6,
            1683.1,
            335.9,
        ],
        "velocity": [
            1272.7,
            -39.7,
            746.4,
        ]
    }
    ...
]
```

### Full

```python
[
    {
        "time": 0.008333,
        "location": [
            -2283.9,
            1683.8,
            323.4,
        ],
        "velocity": [
            1273.4,
            -39.7,
            757.6,
        ]
        "angular_velocity": [
            2.3,
            -0.8,
            3.8,
        }
    },
    {
        "time": 0.016666,
        "location": [
            -2273.3,
            1683.4,
            329.7,
        ],
        "velocity": [
            1273.1,
            -39.7,
            752.0,
        ],
        "angular_velocity": [
            2.3,
            -0.8,
            3.8
        ]
    }
    ...
]
```

## \_\_doc__

Returns the string `rl_ball_sym is a Rust implementation of ball path prediction for Rocket League; Inspired by Samuel (Chip) P. Mish's C++ utils called RLUtilities`

## load_soccar

Loads in the field for a standard soccar game.

## load_dropshot

Loads in the field for a standard dropshot game.

## load_hoops

Loads in the field for a standard hoops game.

## tick

Sets information related to the game.

## step_ball

Steps the ball by `1/120` seconds into the future every time it's called.

For convience, also returns the new information about the ball.

Example:

```python
{
    "time": 0.008333,
    "location": [
        -2283.9,
        1683.8,
        323.4,
    ],
    "velocity": [
        1273.4,
        -39.7,
        757.6,
    ]
    "angular_velocity": [
        2.3,
        -0.8,
        3.8,
    }
}
```

## get_ball_prediction_struct

Equivalent to calling `step_ball()` 720 times (6 seconds).

Returns a normal-type ball prediction struct.

![get_ball_prediction_struct takes 0.3ms to execute](https://raw.githubusercontent.com/VirxEC/rl_ball_sym_pybinds/master/gbps_bench.png)

## get_ball_prediction_struct_full

Equivalent to calling `step_ball()` 720 times (6 seconds).

Returns a full-type ball prediction struct.

![get_ball_prediction_struct_full takes 0.54ms to execute](https://raw.githubusercontent.com/VirxEC/rl_ball_sym_pybinds/master/gbpsft_bench.png)

## get_ball_prediction_struct_for_time

Equivalent to calling `step_ball()` 120 * `time` times.

Returns a normal-type ball prediction struct.

### time

The seconds into the future that the ball path prediction should be generated.

## get_ball_prediction_struct_full_for_time

Equivalent to calling `step_ball()` 120 * `time` times.

Returns a full-type ball prediction struct.

### time

The seconds into the future that the ball path prediction should be generated.
