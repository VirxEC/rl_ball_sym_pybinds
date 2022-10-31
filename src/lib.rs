#![forbid(unsafe_code)]

mod pytypes;

use pyo3::{exceptions, prelude::*, PyErr};
use pytypes::*;
use rl_ball_sym::simulation::{ball::Ball, game::Game};
use std::sync::RwLock;

static GAME: RwLock<Option<Game>> = RwLock::new(None);
type NoGamePyErr = exceptions::PyNameError;
const NO_GAME_ERR: &str = "GAME is unset. Call a function like load_soccer first.";

static BALL: RwLock<Option<Ball>> = RwLock::new(None);
type NoBallPyErr = exceptions::PyNameError;
const NO_BALL_ERR: &str = "BALL is unset. Call a function like load_soccer first.";

macro_rules! pynamedmodule {
    (doc: $doc:literal, name: $name:tt, funcs: [$($func_name:path),*], classes: [$($class_name:ident),*]) => {
        #[doc = $doc]
        #[pymodule]
        fn $name(_py: Python, m: &PyModule) -> PyResult<()> {
            $(m.add_function(wrap_pyfunction!($func_name, m)?)?);*;
            $(m.add_class::<$class_name>()?);*;
            Ok(())
        }
    };
}

pynamedmodule! {
    doc: "rl_ball_sym is a Rust implementation of ball path prediction for Rocket League; Inspired by Samuel (Chip) P. Mish's C++ utils called RLUtilities",
    name: rl_ball_sym_pybinds,
    funcs: [
        load_soccer,
        load_soccar,
        load_dropshot,
        load_hoops,
        load_soccar_throwback,
        load_soccer_throwback,
        tick,
        step_ball,
        get_ball_prediction_struct,
        get_ball_prediction_struct_for_time,
        get_ball_prediction_struct_full,
        get_ball_prediction_struct_for_time_full
    ],
    classes: [BallSlice, BallPredictionStruct]
}

#[pyfunction]
fn load_soccer() {
    let (game, ball) = rl_ball_sym::compressed::load_soccer();
    *GAME.write().expect("GAME lock was poisoned") = Some(game);
    *BALL.write().expect("BALL lock was poisoned") = Some(ball);
}

#[pyfunction]
fn load_soccar() {
    load_soccer();
}

#[pyfunction]
fn load_dropshot() {
    let (game, ball) = rl_ball_sym::compressed::load_dropshot();
    *GAME.write().expect("GAME lock was poisoned") = Some(game);
    *BALL.write().expect("BALL lock was poisoned") = Some(ball);
}

#[pyfunction]
fn load_hoops() {
    let (game, ball) = rl_ball_sym::compressed::load_hoops();
    *GAME.write().expect("GAME lock was poisoned") = Some(game);
    *BALL.write().expect("BALL lock was poisoned") = Some(ball);
}

#[pyfunction]
fn load_soccer_throwback() {
    let (game, ball) = rl_ball_sym::compressed::load_soccar_throwback();
    *GAME.write().expect("GAME lock was poisoned") = Some(game);
    *BALL.write().expect("BALL lock was poisoned") = Some(ball);
}

#[pyfunction]
fn load_soccar_throwback() {
    load_soccer_throwback();
}

#[pyfunction]
fn tick(py: Python, packet: PyObject) -> PyResult<()> {
    let mut game_guard = GAME.write().expect("GAME lock was poisoned");
    let game = game_guard.as_mut().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;

    let mut ball_guard = BALL.write().expect("BALL lock was poisoned");
    let ball = ball_guard.as_mut().ok_or_else(|| PyErr::new::<NoBallPyErr, _>(NO_BALL_ERR))?;

    let packet = packet.as_ref(py).extract::<GamePacket>()?;

    let time = packet.game_info.seconds_elapsed;
    game.gravity.z = packet.game_info.world_gravity_z;

    ball.update(
        time,
        packet.game_ball.physics.location.into(),
        packet.game_ball.physics.velocity.into(),
        packet.game_ball.physics.angular_velocity.into(),
    );

    let radius = packet.game_ball.collision_shape.get_radius();
    ball.set_radius(radius, radius + 1.9);

    Ok(())
}

#[pyfunction]
fn step_ball() -> PyResult<BallSlice> {
    let game_guard = GAME.read().expect("GAME lock was poisoned");
    let game = game_guard.as_ref().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;
    let mut ball_guard = BALL.write().expect("BALL lock was poisoned");
    let ball = ball_guard.as_mut().ok_or_else(|| PyErr::new::<NoBallPyErr, _>(NO_BALL_ERR)).unwrap();

    ball.step(game, 1. / 120.);
    Ok(BallSlice::from_rl_ball_sym(*ball))
}

#[pyfunction]
fn get_ball_prediction_struct() -> PyResult<HalfBallPredictionStruct> {
    let game_guard = GAME.read().expect("GAME lock was poisoned");
    let game = game_guard.as_ref().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;
    let ball = BALL.read().expect("BALL lock was poisoned").ok_or_else(|| PyErr::new::<NoBallPyErr, _>(NO_BALL_ERR))?;

    Ok(HalfBallPredictionStruct::from_rl_ball_sym(ball.get_ball_prediction_struct(game)))
}

#[pyfunction]
fn get_ball_prediction_struct_full() -> PyResult<BallPredictionStruct> {
    let game_guard = GAME.read().expect("GAME lock was poisoned");
    let game = game_guard.as_ref().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;
    let ball = BALL.read().expect("BALL lock was poisoned").ok_or_else(|| PyErr::new::<NoBallPyErr, _>(NO_BALL_ERR))?;

    Ok(BallPredictionStruct::from_rl_ball_sym(ball.get_ball_prediction_struct(game)))
}

#[pyfunction]
fn get_ball_prediction_struct_for_time(time: f32) -> PyResult<HalfBallPredictionStruct> {
    let game_guard = GAME.read().expect("GAME lock was poisoned");
    let game = game_guard.as_ref().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;
    let ball = BALL.read().expect("BALL lock was poisoned").ok_or_else(|| PyErr::new::<NoBallPyErr, _>(NO_BALL_ERR))?;

    Ok(HalfBallPredictionStruct::from_rl_ball_sym(ball.get_ball_prediction_struct_for_time(game, time)))
}

#[pyfunction]
fn get_ball_prediction_struct_for_time_full(time: f32) -> PyResult<BallPredictionStruct> {
    let game_guard = GAME.read().expect("GAME lock was poisoned");
    let game = game_guard.as_ref().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;
    let ball = BALL.read().expect("BALL lock was poisoned").ok_or_else(|| PyErr::new::<NoBallPyErr, _>(NO_BALL_ERR))?;

    Ok(BallPredictionStruct::from_rl_ball_sym(ball.get_ball_prediction_struct_for_time(game, time)))
}
