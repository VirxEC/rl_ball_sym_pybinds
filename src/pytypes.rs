use pyo3::prelude::*;
use rl_ball_sym::{Ball, Game, Predictions, Vec3A};

#[derive(FromPyObject, Debug)]
pub struct GameVec {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<GameVec> for Vec3A {
    #[inline]
    fn from(gv: GameVec) -> Self {
        Self::new(gv.x, gv.y, gv.z)
    }
}

#[derive(FromPyObject, Debug)]
pub struct GameSphere {
    pub diameter: f32,
}

#[derive(FromPyObject, Debug)]
pub struct GameBox {
    pub length: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(FromPyObject, Debug)]
pub struct GameCylinder {
    pub diameter: f32,
    pub height: f32,
}

#[derive(FromPyObject, Debug)]
pub struct GameCollisionShape {
    #[pyo3(attribute("type"))]
    shape_type: usize,
    #[pyo3(attribute("box"))]
    cuboid: GameBox,
    sphere: GameSphere,
    cylinder: GameCylinder,
}

impl GameCollisionShape {
    #[inline]
    pub fn get_radius(&self) -> f32 {
        match self.shape_type {
            0 => (self.cuboid.length + self.cuboid.width + self.cuboid.height) / 6.,
            1 => self.sphere.diameter / 2.,
            2 => self.cylinder.diameter / 2.,
            _ => panic!("Invalid shape type: {}", self.shape_type),
        }
    }
}

#[derive(FromPyObject, Debug)]
pub struct GamePhysics {
    pub location: GameVec,
    pub velocity: GameVec,
    pub angular_velocity: GameVec,
}

#[derive(FromPyObject, Debug)]
pub struct GameBall {
    pub physics: GamePhysics,
    pub collision_shape: GameCollisionShape,
}

#[derive(FromPyObject, Debug)]
pub struct GameInfo {
    pub seconds_elapsed: f32,
    pub world_gravity_z: f32,
}

#[derive(FromPyObject, Debug)]
pub struct GamePacket {
    pub game_info: GameInfo,
    pub game_ball: GameBall,
}

impl GamePacket {
    #[inline]
    pub fn export_to_game(&self, game: &mut Game) {
        game.gravity.z = self.game_info.world_gravity_z;
    }

    pub fn export_to_ball(self, ball: &mut Ball) {
        ball.update(
            self.game_info.seconds_elapsed,
            self.game_ball.physics.location.into(),
            self.game_ball.physics.velocity.into(),
            self.game_ball.physics.angular_velocity.into(),
        );

        let radius = self.game_ball.collision_shape.get_radius();
        if (ball.radius() - radius).abs() > f32::EPSILON {
            ball.set_radius(radius);
        }
    }
}

#[inline]
const fn vec3a_to_tuple(v: Vec3A) -> (f32, f32, f32) {
    let [x, y, z] = v.to_array();
    (x, y, z)
}

#[pyclass(frozen, get_all)]
#[derive(Clone, Copy, Debug)]
pub struct BallSlice {
    time: f32,
    location: (f32, f32, f32),
    velocity: (f32, f32, f32),
    angular_velocity: (f32, f32, f32),
}

impl BallSlice {
    #[inline]
    pub const fn from_rl_ball_sym(raw_ball: Ball) -> Self {
        Self {
            time: raw_ball.time,
            location: vec3a_to_tuple(raw_ball.location),
            velocity: vec3a_to_tuple(raw_ball.velocity),
            angular_velocity: vec3a_to_tuple(raw_ball.angular_velocity),
        }
    }
}

#[pymethods]
impl BallSlice {
    #[inline]
    fn __str__(&self) -> String {
        format!(
            "Ball @{:.2}s - location: {:?}, velocity: {:?}, angular velocity: {:?}",
            self.time, self.location, self.velocity, self.angular_velocity
        )
    }

    #[inline]
    fn __repr__(&self) -> String {
        format!(
            "BallSlice(time={}, location={:?}, velocity={:?}, angular_velocity={:?})",
            self.time, self.location, self.velocity, self.angular_velocity
        )
    }
}

#[pyclass(frozen, get_all)]
#[derive(Clone, Debug)]
pub struct BallPredictionStruct {
    num_slices: usize,
    slices: Vec<BallSlice>,
}

impl From<Predictions> for BallPredictionStruct {
    #[inline]
    fn from(raw_struct: Predictions) -> Self {
        Self {
            num_slices: raw_struct.len(),
            slices: raw_struct.into_iter().map(BallSlice::from_rl_ball_sym).collect(),
        }
    }
}

#[pymethods]
impl BallPredictionStruct {
    #[inline]
    fn __str__(&self) -> String {
        format!("Ball prediction - {} slices", self.num_slices)
    }

    #[inline]
    fn __repr__(&self) -> String {
        format!("BallPredictionStruct(num_slices={}, slices=[... {} items])", self.num_slices, self.slices.len())
    }
}

#[pyclass(frozen, get_all)]
#[derive(Clone, Copy, Debug)]
pub struct HalfBallSlice {
    time: f32,
    location: (f32, f32, f32),
    velocity: (f32, f32, f32),
}

impl HalfBallSlice {
    #[inline]
    pub const fn from_rl_ball_sym(raw_ball: Ball) -> Self {
        Self {
            time: raw_ball.time,
            location: vec3a_to_tuple(raw_ball.location),
            velocity: vec3a_to_tuple(raw_ball.velocity),
        }
    }
}

#[pymethods]
impl HalfBallSlice {
    #[inline]
    fn __str__(&self) -> String {
        format!("Ball @{:.2}s - location: {:?}, velocity: {:?}", self.time, self.location, self.velocity)
    }

    #[inline]
    fn __repr__(&self) -> String {
        format!("HalfBallSlice(time={}, location={:?}, velocity={:?})", self.time, self.location, self.velocity)
    }
}

#[pyclass(frozen, get_all)]
#[derive(Clone, Debug)]
pub struct HalfBallPredictionStruct {
    num_slices: usize,
    slices: Vec<HalfBallSlice>,
}

impl From<Predictions> for HalfBallPredictionStruct {
    #[inline]
    fn from(raw_struct: Predictions) -> Self {
        let slices = raw_struct.into_iter().step_by(2).map(HalfBallSlice::from_rl_ball_sym).collect::<Vec<_>>();

        Self { num_slices: slices.len(), slices }
    }
}

#[pymethods]
impl HalfBallPredictionStruct {
    #[inline]
    fn __str__(&self) -> String {
        format!("Ball prediction - {} slices", self.num_slices)
    }

    #[inline]
    fn __repr__(&self) -> String {
        format!("HalfBallPredictionStruct(num_slices={}, slices=[... {} items])", self.num_slices, self.slices.len())
    }
}
