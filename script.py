from traceback import print_exc
from typing import Sequence

from rlbot import flat

# import RocketSim as rs
from rlbot.managers.script import Script

import rl_ball_sym_pybinds as rlbs


class rl_ball_sym(Script):
    def __init__(self):
        super().__init__("rl_ball_sym")

        # rlbs.load_standard()
        rlbs.load_standard_heatseeker()

        # self.arena = rs.Arena(rs.GameMode.HEATSEEKER)
        self.framework_prediction: Sequence[flat.PredictionSlice] = []

    def handle_ball_prediction(self, ball_prediction: flat.BallPrediction):
        self.framework_prediction = ball_prediction.slices

    def handle_packet(self, packet: flat.GameTickPacket):
        try:
            if len(packet.balls) == 0:
                return

            rlbs.tick(packet)

            # rl_ball = packet.balls[0].physics
            # ball = self.arena.ball.get_state()
            # ball.pos.x = rl_ball.location.x
            # ball.pos.y = rl_ball.location.y
            # ball.pos.z = rl_ball.location.z
            # ball.vel.x = rl_ball.velocity.x
            # ball.vel.y = rl_ball.velocity.y
            # ball.vel.z = rl_ball.velocity.z
            # ball.ang_vel.x = rl_ball.angular_velocity.x
            # ball.ang_vel.y = rl_ball.angular_velocity.y
            # ball.ang_vel.z = rl_ball.angular_velocity.z
            # self.arena.ball.set_state(ball)

            # self.renderer.begin_rendering("rocketsim")

            # rocketsim_prediction = []
            # for i in range(0, 120 * 6):
            #     self.arena.step()
            #     pos = self.arena.ball.get_state().pos
            #     if i % 8 == 0:
            #         rocketsim_prediction.append(flat.Vector3(pos.x, pos.y, pos.z))

            # self.renderer.draw_polyline_3d(
            #     rocketsim_prediction,
            #     self.renderer.yellow,
            # )

            # self.renderer.end_rendering()

            self.renderer.begin_rendering("rl_ball_sym")

            custom_prediction = rlbs.get_ball_prediction_struct()
            self.renderer.draw_polyline_3d(
                [
                    flat.Vector3(*custom_prediction.slices[i].location)
                    for i in range(0, custom_prediction.num_slices, 4)
                ],
                self.renderer.red,
            )

            self.renderer.end_rendering()

            if len(self.framework_prediction) > 2:
                self.renderer.begin_rendering("rlbot")

                self.renderer.draw_polyline_3d(
                    tuple(
                        slice.physics.location
                        for slice in self.framework_prediction[::4]
                    ),
                    self.renderer.green,
                )

                self.renderer.end_rendering()
        except Exception:
            print_exc()


if __name__ == "__main__":
    rl_ball_sym().run(False)
