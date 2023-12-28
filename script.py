from traceback import print_exc

import RocketSim as rs
from rlbot.agents.base_script import BaseScript
from rlbot.utils.structures.game_data_struct import GameTickPacket

import rl_ball_sym_pybinds as rlbs


class rl_ball_sym(BaseScript):
    def __init__(self):
        super().__init__("rl_ball_sym")

    def main(self):
        rlbs.load_standard()
        arena = rs.Arena(rs.GameMode.SOCCAR)

        while 1:
            try:
                self.packet: GameTickPacket = self.wait_game_tick_packet()

                rlbs.tick(self.packet)
                ball = arena.ball.get_state()
                ball.pos.x = self.packet.game_ball.physics.location.x
                ball.pos.y = self.packet.game_ball.physics.location.y
                ball.pos.z = self.packet.game_ball.physics.location.z
                ball.vel.x = self.packet.game_ball.physics.velocity.x
                ball.vel.y = self.packet.game_ball.physics.velocity.y
                ball.vel.z = self.packet.game_ball.physics.velocity.z
                ball.ang_vel.x = self.packet.game_ball.physics.angular_velocity.x
                ball.ang_vel.y = self.packet.game_ball.physics.angular_velocity.y
                ball.ang_vel.z = self.packet.game_ball.physics.angular_velocity.z
                arena.ball.set_state(ball)

                self.renderer.begin_rendering("rocketsim")

                rocketsim_prediction = []
                for _ in range(0, 120 * 6):
                    arena.step()
                    pos = arena.ball.get_state().pos
                    rocketsim_prediction.append((pos.x, pos.y, pos.z))

                self.renderer.draw_polyline_3d(tuple(rocketsim_prediction[i] for i in range(0, len(rocketsim_prediction), 2)), self.renderer.yellow())

                self.renderer.end_rendering()
                self.renderer.begin_rendering("rl_ball_sym")

                custom_prediction = rlbs.get_ball_prediction_struct()
                self.renderer.draw_polyline_3d(tuple(custom_prediction.slices[i].location for i in range(0, custom_prediction.num_slices, 2)), self.renderer.red())

                self.renderer.end_rendering()
                self.renderer.begin_rendering("rlbot")

                framework_prediction = self.get_ball_prediction_struct()
                if framework_prediction is not None and framework_prediction.num_slices > 2:
                    self.renderer.draw_polyline_3d(tuple((framework_prediction.slices[i].physics.location.x, framework_prediction.slices[i].physics.location.y, framework_prediction.slices[i].physics.location.z) for i in range(0, framework_prediction.num_slices, 2)), self.renderer.green())

                self.renderer.end_rendering()
            except Exception:
                print_exc()


if __name__ == "__main__":
    # set the CWD to the directory of this script
    import os
    os.chdir(os.path.dirname(os.path.realpath(__file__)))

    rl_ball_sym().main()
