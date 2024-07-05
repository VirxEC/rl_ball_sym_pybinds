#![warn(clippy::pedantic, clippy::all)]

mod pytypes;

use pyo3::{exceptions, prelude::*, PyErr};
use pytypes::{BallPredictionStruct, BallSlice, GamePacket, HalfBallPredictionStruct, HalfBallSlice};
use rl_ball_sym::{Ball, Game};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    RwLock,
};

static GAME: RwLock<Option<Game>> = RwLock::new(None);
static BALL: RwLock<Ball> = RwLock::new(Ball::const_default());
pub static HEATSEEKER: AtomicBool = AtomicBool::new(false);

type NoGamePyErr = exceptions::PyNameError;
const NO_GAME_ERR: &str = "GAME is unset. Call a function like load_standard first.";

const TPS: f32 = 120.;
const TICK_DT: f32 = 1. / TPS;

macro_rules! pynamedmodule {
    (doc: $doc:literal, name: $name:tt, funcs: [$($func_name:path),*], classes: [$($class_name:ident),*]) => {
        #[doc = $doc]
        #[pymodule]
        fn $name(m: &Bound<PyModule>) -> PyResult<()> {
            $(m.add_function(wrap_pyfunction!($func_name, m)?)?);*;
            $(m.add_class::<$class_name>()?);*;
            Ok(())
        }
    };
}

pynamedmodule! {
    doc: "rl_ball_sym is a Rust implementation of Rocket League's ball physics",
    name: rl_ball_sym_pybinds,
    funcs: [
        load_standard,
        load_dropshot,
        load_hoops,
        load_standard_throwback,
        load_standard_heatseeker,
        tick,
        step_ball,
        get_ball_prediction_struct,
        get_ball_prediction_struct_for_time,
        get_ball_prediction_struct_full,
        get_ball_prediction_struct_for_time_full
    ],
    classes: [HalfBallSlice, HalfBallPredictionStruct, BallSlice, BallPredictionStruct]
}

#[pyfunction]
fn load_standard() {
    let (game, ball) = rl_ball_sym::load_standard();

    *GAME.write().unwrap() = Some(game);
    *BALL.write().unwrap() = ball;
    HEATSEEKER.store(false, Ordering::Relaxed);
}

#[pyfunction]
fn load_standard_heatseeker() {
    let (game, ball) = rl_ball_sym::load_standard_heatseeker();

    *GAME.write().unwrap() = Some(game);
    *BALL.write().unwrap() = ball;
    HEATSEEKER.store(true, Ordering::Relaxed);
}

#[pyfunction]
fn load_dropshot() {
    let (game, ball) = rl_ball_sym::load_dropshot();

    *GAME.write().unwrap() = Some(game);
    *BALL.write().unwrap() = ball;
    HEATSEEKER.store(false, Ordering::Relaxed);
}

#[pyfunction]
fn load_hoops() {
    let (game, ball) = rl_ball_sym::load_hoops();

    *GAME.write().unwrap() = Some(game);
    *BALL.write().unwrap() = ball;
    HEATSEEKER.store(false, Ordering::Relaxed);
}

#[pyfunction]
fn load_standard_throwback() {
    let (game, ball) = rl_ball_sym::load_standard_throwback();

    *GAME.write().unwrap() = Some(game);
    *BALL.write().unwrap() = ball;
    HEATSEEKER.store(false, Ordering::Relaxed);
}

#[pyfunction]
fn tick(packet: GamePacket) -> PyResult<()> {
    {
        let mut game_lock = GAME.write().unwrap();
        let game = game_lock.as_mut().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;
        packet.export_to_game(game);
    }

    {
        let mut ball_lock = BALL.write().unwrap();
        packet.export_to_ball(&mut ball_lock);
    }

    Ok(())
}

#[pyfunction]
fn step_ball() -> PyResult<BallSlice> {
    let game_lock = GAME.read().unwrap();
    let game = game_lock.as_ref().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;

    let mut ball = BALL.write().unwrap();

    if HEATSEEKER.load(Ordering::Relaxed) {
        ball.step_heatseeker(game, TICK_DT);
    } else {
        ball.step(game, TICK_DT);
    }

    Ok(BallSlice::from_rl_ball_sym(*ball))
}

#[pyfunction]
fn get_ball_prediction_struct() -> PyResult<HalfBallPredictionStruct> {
    let game_lock = GAME.read().unwrap();
    let game = game_lock.as_ref().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;

    let ball = BALL.read().unwrap();
    let prediction = if HEATSEEKER.load(Ordering::Relaxed) {
        ball.get_heatseeker_prediction_struct(game)
    } else {
        ball.get_ball_prediction_struct(game)
    };

    Ok(prediction.into())
}

#[pyfunction]
fn get_ball_prediction_struct_full() -> PyResult<BallPredictionStruct> {
    let game_lock = GAME.read().unwrap();
    let game = game_lock.as_ref().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;

    let ball = BALL.read().unwrap();
    let prediction = if HEATSEEKER.load(Ordering::Relaxed) {
        ball.get_heatseeker_prediction_struct(game)
    } else {
        ball.get_ball_prediction_struct(game)
    };

    Ok(prediction.into())
}

#[pyfunction]
fn get_ball_prediction_struct_for_time(time: f32) -> PyResult<HalfBallPredictionStruct> {
    let game_lock = GAME.read().unwrap();
    let game = game_lock.as_ref().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;

    let ball = BALL.read().unwrap();
    let prediction = if HEATSEEKER.load(Ordering::Relaxed) {
        ball.get_heatseeker_prediction_struct_for_time(game, time)
    } else {
        ball.get_ball_prediction_struct_for_time(game, time)
    };

    Ok(prediction.into())
}

#[pyfunction]
fn get_ball_prediction_struct_for_time_full(time: f32) -> PyResult<BallPredictionStruct> {
    let game_lock = GAME.read().unwrap();
    let game = game_lock.as_ref().ok_or_else(|| PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR))?;

    let ball = BALL.read().unwrap();
    let prediction = if HEATSEEKER.load(Ordering::Relaxed) {
        ball.get_heatseeker_prediction_struct_for_time(game, time)
    } else {
        ball.get_ball_prediction_struct_for_time(game, time)
    };

    Ok(prediction.into())
}
