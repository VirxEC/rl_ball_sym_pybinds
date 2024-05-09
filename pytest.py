from random import uniform
from time import time_ns

from rlbot_flatbuffers import GameTickPacket, SphereShape

import rl_ball_sym_pybinds as rlbs

print(rlbs.__doc__)

GAME_MODES = {
    "standard": rlbs.load_standard,
    "dropshot": rlbs.load_dropshot,
    "hoops": rlbs.load_hoops,
    "standard (throwback)": rlbs.load_standard_throwback,
}

PREDICTIONS = {
    "get_ball_prediction_struct": [rlbs.get_ball_prediction_struct,],
    "get_ball_prediction_struct_full": [rlbs.get_ball_prediction_struct_full,],
    "get_ball_prediction_struct_for_time": [rlbs.get_ball_prediction_struct_for_time, 12],
    "get_ball_prediction_struct_for_time_full": [rlbs.get_ball_prediction_struct_for_time_full, 12],
}

def set_random_packet(time):
    packet = GameTickPacket()

    # set 6 cars to test the speed of tick()
    packet.ball.physics.location.x = uniform(-4000, 4000)
    packet.ball.physics.location.y = uniform(-5020, 5020)
    packet.ball.physics.location.z = uniform(100, 1944)

    packet.ball.physics.velocity.x = uniform(-2000, 2000)
    packet.ball.physics.velocity.y = uniform(-2000, 2000)
    packet.ball.physics.velocity.z = uniform(-2000, 2000)

    packet.ball.physics.angular_velocity.x = uniform(-1, 1)
    packet.ball.physics.angular_velocity.y = uniform(-1, 1)
    packet.ball.physics.angular_velocity.z = uniform(-1, 1)

    packet.ball.shape.item = SphereShape(182.5)
    packet.game_info.world_gravity_z = -650.
    packet.game_info.seconds_elapsed = time

    rlbs.tick(packet)


GAME_MODES["standard"]()
for prediction_name, prediction_func in PREDICTIONS.items():
    print()
    set_random_packet(0)

    if len(prediction_func) == 1:
        prediction = prediction_func[0]()
    else:
        prediction = prediction_func[0](prediction_func[1])

    print(prediction)
    print(repr(prediction))

    ball_slice = prediction.slices[50]
    print(ball_slice)
    print(repr(ball_slice))


for gamemode_name, load_func in GAME_MODES.items():
    load_func()

    for prediction_name, prediction_func in PREDICTIONS.items():
        print()
        print(f"Testing {prediction_name} in {gamemode_name}")

        time = 0
        times = []

        for _ in range(5000):
            set_random_packet(time)

            start = time_ns()

            if len(prediction_func) == 1:
                prediction_func[0]()
            else:
                prediction_func[0](prediction_func[1])

            times.append(time_ns() - start)
            time += 1 / 120

        print(f"Total test time: {round(sum(times) / 1000000000, 4)}s")
        print(f"Avg. time of execution: {round(sum(times) / len(times) / 1000000, 2)}ms")
