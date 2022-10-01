mod pytypes;

use pyo3::{exceptions, prelude::*, PyErr};
use pytypes::*;
use rl_ball_sym::{
    glam::Vec3A,
    simulation::{ball::Ball, game::Game},
};
use std::sync::RwLock;

static GAME: RwLock<Option<Game>> = RwLock::new(None);
type NoGamePyErr = exceptions::PyNameError;
const NO_GAME_ERR: &str = "GAME is unset. Call a function like load_soccer first.";

static BALL: RwLock<Option<Ball>> = RwLock::new(None);
type NoBallPyErr = exceptions::PyNameError;
const NO_BALL_ERR: &str = "BALL is unset. Call a function like load_soccer first.";

#[pymodule]
/// rl_ball_sym is a Rust implementation of ball path prediction for Rocket League; Inspired by Samuel (Chip) P. Mish's C++ utils called RLUtilities
fn rl_ball_sym_pybinds(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_soccer, m)?)?;
    m.add_function(wrap_pyfunction!(load_soccar, m)?)?;
    m.add_function(wrap_pyfunction!(load_dropshot, m)?)?;
    m.add_function(wrap_pyfunction!(load_hoops, m)?)?;
    m.add_function(wrap_pyfunction!(load_soccar_throwback, m)?)?;
    m.add_function(wrap_pyfunction!(load_soccer_throwback, m)?)?;
    m.add_function(wrap_pyfunction!(tick, m)?)?;
    m.add_function(wrap_pyfunction!(step_ball, m)?)?;
    m.add_function(wrap_pyfunction!(get_ball_prediction_struct, m)?)?;
    m.add_function(wrap_pyfunction!(get_ball_prediction_struct_for_time, m)?)?;
    m.add_function(wrap_pyfunction!(get_ball_prediction_struct_full, m)?)?;
    m.add_function(wrap_pyfunction!(get_ball_prediction_struct_for_time_full, m)?)?;
    m.add_class::<BallSlice>()?;
    m.add_class::<BallPredictionStruct>()?;
    Ok(())
}

#[pyfunction]
fn load_soccer() {
    let (game, ball) = rl_ball_sym::load_soccer();
    *GAME.write().expect("GAME lock was poisoned") = Some(game);
    *BALL.write().expect("BALL lock was poisoned") = Some(ball);
}

#[pyfunction]
fn load_soccar() {
    load_soccer();
}

#[pyfunction]
fn load_dropshot() {
    let (game, ball) = rl_ball_sym::load_dropshot();
    *GAME.write().expect("GAME lock was poisoned") = Some(game);
    *BALL.write().expect("BALL lock was poisoned") = Some(ball);
}

#[pyfunction]
fn load_hoops() {
    let (game, ball) = rl_ball_sym::load_hoops();
    *GAME.write().expect("GAME lock was poisoned") = Some(game);
    *BALL.write().expect("BALL lock was poisoned") = Some(ball);
}

#[pyfunction]
fn load_soccer_throwback() {
    let (game, ball) = rl_ball_sym::load_soccar_throwback();
    *GAME.write().expect("GAME lock was poisoned") = Some(game);
    *BALL.write().expect("BALL lock was poisoned") = Some(ball);
}

#[pyfunction]
fn load_soccar_throwback() {
    load_soccer_throwback()
}

#[inline]
fn get_vec3_named(py_vec: &PyAny) -> PyResult<Vec3A> {
    Ok(Vec3A::new(
        py_vec.getattr("x")?.extract()?,
        py_vec.getattr("y")?.extract()?,
        py_vec.getattr("z")?.extract()?,
    ))
}

#[pyfunction]
fn tick(py: Python, packet: PyObject) -> PyResult<()> {
    let mut game_guard = GAME.write().expect("GAME lock was poisoned");
    let game = game_guard.as_mut().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;

    let mut ball_guard = BALL.write().expect("BALL lock was poisoned");
    let ball = ball_guard.as_mut().ok_or_else(|| PyErr::new::<NoBallPyErr, _>(NO_BALL_ERR))?;

    let packet = packet.as_ref(py);

    let py_game_info = packet.getattr("game_info")?;

    let time = py_game_info.getattr("seconds_elapsed")?.extract::<f32>()?;

    game.gravity.z = py_game_info.getattr("world_gravity_z")?.extract()?;

    let py_ball = packet.getattr("game_ball")?;
    let py_ball_physics = py_ball.getattr("physics")?;

    ball.update(
        time,
        get_vec3_named(py_ball_physics.getattr("location")?)?,
        get_vec3_named(py_ball_physics.getattr("velocity")?)?,
        get_vec3_named(py_ball_physics.getattr("angular_velocity")?)?,
    );

    let py_ball_shape = py_ball.getattr("collision_shape")?;

    let radius = py_ball_shape.getattr("sphere")?.getattr("diameter")?.extract::<f32>()? / 2.;
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

    Ok(HalfBallPredictionStruct::from_rl_ball_sym(ball.get_ball_prediction_struct_for_time(game, &time)))
}

#[pyfunction]
fn get_ball_prediction_struct_for_time_full(time: f32) -> PyResult<BallPredictionStruct> {
    let game_guard = GAME.read().expect("GAME lock was poisoned");
    let game = game_guard.as_ref().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;
    let ball = BALL.read().expect("BALL lock was poisoned").ok_or_else(|| PyErr::new::<NoBallPyErr, _>(NO_BALL_ERR))?;

    Ok(BallPredictionStruct::from_rl_ball_sym(ball.get_ball_prediction_struct_for_time(game, &time)))
}
