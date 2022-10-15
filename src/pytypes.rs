use pyo3::prelude::*;
use rl_ball_sym::{Ball, BallPrediction, Vec3A};

#[derive(FromPyObject, Debug)]
pub struct GameVec {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl From<GameVec> for Vec3A {
    #[inline]
    fn from(gv: GameVec) -> Self {
        Vec3A::new(gv.x, gv.y, gv.z)
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
    box_: GameBox,
    sphere: GameSphere,
    cylinder: GameCylinder,
}

impl GameCollisionShape {
    #[inline]
    pub fn get_radius(&self) -> f32 {
        match self.shape_type {
            0 => (self.box_.length + self.box_.width + self.box_.height) / 6.,
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

#[inline]
const fn vec3a_to_tuple(v: Vec3A) -> (f32, f32, f32) {
    let [x, y, z] = v.to_array();
    (x, y, z)
}

#[pyclass(frozen)]
#[derive(Clone, Copy, Debug)]
pub struct BallSlice {
    #[pyo3(get)]
    time: f32,
    #[pyo3(get)]
    location: (f32, f32, f32),
    #[pyo3(get)]
    velocity: (f32, f32, f32),
    #[pyo3(get)]
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
    fn __str__(&self) -> String {
        format!(
            "Ball @{:.2}s - location: {:?}, velocity: {:?}, angular velocity: {:?}",
            self.time, self.location, self.velocity, self.angular_velocity
        )
    }

    fn __repr__(&self) -> String {
        format!(
            "BallSlice(time={}, location={:?}, velocity={:?}, angular_velocity={:?})",
            self.time, self.location, self.velocity, self.angular_velocity
        )
    }
}

#[pyclass(frozen)]
#[derive(Clone, Debug)]
pub struct BallPredictionStruct {
    #[pyo3(get)]
    num_slices: usize,
    #[pyo3(get)]
    slices: Vec<BallSlice>,
}

impl BallPredictionStruct {
    #[inline]
    pub fn from_rl_ball_sym(raw_struct: BallPrediction) -> Self {
        Self {
            num_slices: raw_struct.len(),
            slices: raw_struct.into_iter().map(BallSlice::from_rl_ball_sym).collect(),
        }
    }
}

#[pymethods]
impl BallPredictionStruct {
    fn __str__(&self) -> String {
        format!("Ball prediction - {} slices", self.num_slices)
    }

    fn __repr__(&self) -> String {
        format!("BallPredictionStruct(num_slices={}, slices=[... {} items])", self.num_slices, self.slices.len())
    }
}

#[pyclass(frozen)]
#[derive(Clone, Copy, Debug)]
pub struct HalfBallSlice {
    #[pyo3(get)]
    time: f32,
    #[pyo3(get)]
    location: (f32, f32, f32),
    #[pyo3(get)]
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
    fn __str__(&self) -> String {
        format!("Ball @{:.2}s - location: {:?}, velocity: {:?}", self.time, self.location, self.velocity)
    }

    fn __repr__(&self) -> String {
        format!("HalfBallSlice(time={}, location={:?}, velocity={:?})", self.time, self.location, self.velocity)
    }
}

#[pyclass(frozen)]
#[derive(Clone, Debug)]
pub struct HalfBallPredictionStruct {
    #[pyo3(get)]
    num_slices: usize,
    #[pyo3(get)]
    slices: Vec<HalfBallSlice>,
}

impl HalfBallPredictionStruct {
    pub fn from_rl_ball_sym(raw_struct: BallPrediction) -> Self {
        let slices = raw_struct.into_iter().step_by(2).map(HalfBallSlice::from_rl_ball_sym).collect::<Vec<_>>();

        Self { num_slices: slices.len(), slices }
    }
}

#[pymethods]
impl HalfBallPredictionStruct {
    fn __str__(&self) -> String {
        format!("Ball prediction - {} slices", self.num_slices)
    }

    fn __repr__(&self) -> String {
        format!("HalfBallPredictionStruct(num_slices={}, slices=[... {} items])", self.num_slices, self.slices.len())
    }
}
