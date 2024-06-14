from traceback import print_exc
from typing import Sequence

# import RocketSim as rs
from rlbot.managers.script import Script
from rlbot_flatbuffers import (
    BallPrediction,
    GameTickPacket,
    PolyLine3D,
    PredictionSlice,
    Vector3,
)

# import rl_ball_sym_pybinds as rlbs


class rl_ball_sym(Script):
    def __init__(self):
        super().__init__("rl_ball_sym")

        # rlbs.load_standard()
        # rlbs.load_standard_heatseeker()

        # self.arena = rs.Arena(rs.GameMode.HEATSEEKER)
        self.framework_prediction: Sequence[PredictionSlice] = []

    def handle_ball_prediction(self, ball_prediction: BallPrediction):
        self.framework_prediction = ball_prediction.slices

    def handle_packet(self, packet: GameTickPacket):
        try:
            # rlbs.tick(packet)

            # ball = self.arena.ball.get_state()
            # ball.pos.x = packet.ball.physics.location.x
            # ball.pos.y = packet.ball.physics.location.y
            # ball.pos.z = packet.ball.physics.location.z
            # ball.vel.x = packet.ball.physics.velocity.x
            # ball.vel.y = packet.ball.physics.velocity.y
            # ball.vel.z = packet.ball.physics.velocity.z
            # ball.ang_vel.x = packet.ball.physics.angular_velocity.x
            # ball.ang_vel.y = packet.ball.physics.angular_velocity.y
            # ball.ang_vel.z = packet.ball.physics.angular_velocity.z
            # self.arena.ball.set_state(ball)

            # self.renderer.begin_rendering("rocketsim")

            # rocketsim_prediction = []
            # for i in range(0, 120 * 6):
            #     self.arena.step()
            #     pos = self.arena.ball.get_state().pos
            #     if i % 8 == 0:
            #         rocketsim_prediction.append(Vector3(pos.x, pos.y, pos.z))

            # self.renderer.draw(
            #     PolyLine3D(
            #         rocketsim_prediction,
            #         self.renderer.yellow,
            #     )
            # )

            # self.renderer.end_rendering()

            # self.renderer.begin_rendering("rl_ball_sym")

            # custom_prediction = rlbs.get_ball_prediction_struct()
            # self.renderer.draw(
            #     PolyLine3D(
            #         [
            #             Vector3(*custom_prediction.slices[i].location)
            #             for i in range(0, custom_prediction.num_slices, 4)
            #         ],
            #         self.renderer.red,
            #     )
            # )

            # self.renderer.end_rendering()

            if len(self.framework_prediction) > 2:
                self.renderer.begin_rendering("rlbot")

                self.renderer.draw(
                    PolyLine3D(
                        tuple(
                            slice.physics.location
                            for slice in self.framework_prediction[::4]
                        ),
                        self.renderer.green,
                    )
                )

                self.renderer.end_rendering()
        except Exception:
            print_exc()


if __name__ == "__main__":
    rl_ball_sym().run()
