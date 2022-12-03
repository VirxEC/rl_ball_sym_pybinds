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

                self.renderer.begin_rendering()

                custom_prediction = rlbs.get_ball_prediction_struct()
                self.renderer.draw_polyline_3d(tuple(custom_prediction.slices[i].location for i in range(0, custom_prediction.num_slices, 4)), self.renderer.red())

                framework_prediction = self.get_ball_prediction_struct()
                if framework_prediction is not None and framework_prediction.num_slices > 8:
                    self.renderer.draw_polyline_3d(tuple((framework_prediction.slices[i].physics.location.x, framework_prediction.slices[i].physics.location.y, framework_prediction.slices[i].physics.location.z) for i in range(0, framework_prediction.num_slices, 4)), self.renderer.green())
                
                self.renderer.end_rendering()
            except Exception:
                print_exc()


if __name__ == "__main__":
    rl_ball_sym().main()
