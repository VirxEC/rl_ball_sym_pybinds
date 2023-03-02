#![forbid(unsafe_code)]

mod pytypes;

use pyo3::{exceptions, prelude::*, PyErr};
use pytypes::*;
use rl_ball_sym::simulation::{ball::Ball, game::Game};
use std::sync::RwLock;

static GAME: RwLock<Option<Game>> = RwLock::new(None);
type NoGamePyErr = exceptions::PyNameError;
const NO_GAME_ERR: &str = "GAME is unset. Call a function like load_soccer first.";

static BALL: RwLock<Ball> = RwLock::new(Ball::const_default());

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
        load_standard,
        load_dropshot,
        load_hoops,
        load_standard_throwback,
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
fn load_standard() {
    let (game, ball) = rl_ball_sym::compressed::load_standard();
    *GAME.write().expect("GAME lock was poisoned") = Some(game);
    *BALL.write().expect("BALL lock was poisoned") = ball;
}

#[pyfunction]
fn load_dropshot() {
    let (game, ball) = rl_ball_sym::compressed::load_dropshot();
    *GAME.write().expect("GAME lock was poisoned") = Some(game);
    *BALL.write().expect("BALL lock was poisoned") = ball;
}

#[pyfunction]
fn load_hoops() {
    let (game, ball) = rl_ball_sym::compressed::load_hoops();
    *GAME.write().expect("GAME lock was poisoned") = Some(game);
    *BALL.write().expect("BALL lock was poisoned") = ball;
}

#[pyfunction]
fn load_standard_throwback() {
    let (game, ball) = rl_ball_sym::compressed::load_standard_throwback();
    *GAME.write().expect("GAME lock was poisoned") = Some(game);
    *BALL.write().expect("BALL lock was poisoned") = ball;
}

#[pyfunction]
fn tick(py: Python, packet: PyObject) -> PyResult<()> {
    let packet: GamePacket = packet.as_ref(py).extract()?;

    packet.export_to_game(
        GAME.write()
            .expect("GAME lock was poisoned")
            .as_mut()
            .ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?,
    );

    packet.export_to_ball(&mut BALL.write().expect("BALL lock was poisoned"));

    Ok(())
}

#[pyfunction]
fn step_ball() -> PyResult<BallSlice> {
    let game_guard = GAME.read().expect("GAME lock was poisoned");
    let game = game_guard.as_ref().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;
    let mut ball = BALL.write().expect("BALL lock was poisoned");

    ball.step(game, 1. / 120.);
    Ok(BallSlice::from_rl_ball_sym(*ball))
}

#[pyfunction]
fn get_ball_prediction_struct() -> PyResult<HalfBallPredictionStruct> {
    let game_guard = GAME.read().expect("GAME lock was poisoned");
    let game = game_guard.as_ref().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;
    let ball = BALL.read().expect("BALL lock was poisoned");

    Ok(HalfBallPredictionStruct::from_rl_ball_sym(ball.get_ball_prediction_struct(game)))
}

#[pyfunction]
fn get_ball_prediction_struct_full() -> PyResult<BallPredictionStruct> {
    let game_guard = GAME.read().expect("GAME lock was poisoned");
    let game = game_guard.as_ref().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;
    let ball = BALL.read().expect("BALL lock was poisoned");

    Ok(BallPredictionStruct::from_rl_ball_sym(ball.get_ball_prediction_struct(game)))
}

#[pyfunction]
fn get_ball_prediction_struct_for_time(time: f32) -> PyResult<HalfBallPredictionStruct> {
    let game_guard = GAME.read().expect("GAME lock was poisoned");
    let game = game_guard.as_ref().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;
    let ball = BALL.read().expect("BALL lock was poisoned");

    Ok(HalfBallPredictionStruct::from_rl_ball_sym(ball.get_ball_prediction_struct_for_time(game, time)))
}

#[pyfunction]
fn get_ball_prediction_struct_for_time_full(time: f32) -> PyResult<BallPredictionStruct> {
    let game_guard = GAME.read().expect("GAME lock was poisoned");
    let game = game_guard.as_ref().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;
    let ball = BALL.read().expect("BALL lock was poisoned");

    Ok(BallPredictionStruct::from_rl_ball_sym(ball.get_ball_prediction_struct_for_time(game, time)))
}
