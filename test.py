from random import uniform
from time import time_ns

import rl_ball_sym as rlbs

print(rlbs.__doc__)

gamemodes = {
    "soccar": rlbs.load_soccar,
    "dropshot": rlbs.load_dropshot,
    "hoops": rlbs.load_hoops,
    "soccar (throwback)": rlbs.load_soccar_throwback,
}

predictions = {
    "get_ball_prediction_struct": [rlbs.get_ball_prediction_struct,],
    "get_ball_prediction_struct_full": [rlbs.get_ball_prediction_struct_full,],
    "get_ball_prediction_struct_for_time": [rlbs.get_ball_prediction_struct_for_time, 12],
    "get_ball_prediction_struct_for_time_full": [rlbs.get_ball_prediction_struct_for_time_full, 12],
}

def set_random_ball():
    rlbs.set_ball({
        "location": [
            uniform(-4000, 4000),
            uniform(-5020, 5020),
            uniform(100, 1944),
        ],
        "velocity": [
            uniform(-2000, 2000),
            uniform(-2000, 2000),
            uniform(-2000, 2000),
        ],
        "angular_velocity": [
            uniform(-1, 1),
            uniform(-1, 1),
            uniform(-1, 1),
        ],
        "time": time
    })

for gamemode_name, load_func in gamemodes.items():
    load_func()

    for prediction_name, prediction_func in predictions.items():
        print()
        print(f"Testing {prediction_name} in {gamemode_name}")

        time = 0
        times = []

        for _ in range(1000):
            set_random_ball()

            start = time_ns()

            if len(prediction_func) == 1:
                prediction_func[0]()
            else:
                prediction_func[0](prediction_func[1])

            times.append(time_ns() - start)
            time += 1 / 120

        print(f"Total test time: {round(sum(times) / 1000000000, 4)}s")
        print(f"Avg. time of execution: {round(sum(times) / len(times) / 1000000, 2)}ms")
