from traceback import print_exc

from rlbot.agents.base_script import BaseScript
from rlbot.utils.structures.game_data_struct import GameTickPacket

import rl_ball_sym as rlbs


class rl_ball_sym(BaseScript):
    def __init__(self):
        super().__init__("rl_ball_sym")

    def main(self):
        rlbs.load_soccar()

        while 1:
            try:
                self.packet: GameTickPacket = self.wait_game_tick_packet()

                framework_prediction = self.get_ball_prediction_struct()
                framework_location = self.packet.game_ball.physics.location
                framework_velocity = self.packet.game_ball.physics.velocity
                framework_angular_velocity = self.packet.game_ball.physics.angular_velocity

                rlbs.set_ball({
                    "time": self.packet.game_info.seconds_elapsed,
                    "location": [framework_location.x, framework_location.y, framework_location.z],
                    "velocity": [framework_velocity.x, framework_velocity.y, framework_velocity.z],
                    "angular_velocity": [framework_angular_velocity.x, framework_angular_velocity.y, framework_angular_velocity.z],
                })

                custom_prediction = rlbs.get_ball_prediction_struct()

                self.renderer.begin_rendering()
                self.renderer.draw_polyline_3d(tuple((framework_prediction.slices[i].physics.location.x, framework_prediction.slices[i].physics.location.y, framework_prediction.slices[i].physics.location.z) for i in range(0, framework_prediction.num_slices, 4)), self.renderer.green())
                self.renderer.draw_polyline_3d(tuple((custom_prediction["slices"][i]["location"][0], custom_prediction["slices"][i]["location"][1], custom_prediction["slices"][i]["location"][2]) for i in range(0, custom_prediction["num_slices"], 4)), self.renderer.red())
                self.renderer.end_rendering()
            except Exception:
                print_exc()


if __name__ == "__main__":
    rl_ball_sym = rl_ball_sym()
    rl_ball_sym.main()
