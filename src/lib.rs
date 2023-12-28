#![warn(clippy::pedantic, clippy::all)]
#![forbid(unsafe_code)]

mod pytypes;

use core::cell::RefCell;
use pyo3::{exceptions, prelude::*, PyErr};
use pytypes::{BallPredictionStruct, BallSlice, GamePacket, HalfBallPredictionStruct, HalfBallSlice};
use rl_ball_sym::{Ball, Game};

thread_local! {
    static GAME: RefCell<Option<Game>> = RefCell::new(None);
    static BALL: RefCell<Ball> = RefCell::new(Ball::const_default());
}

type NoGamePyErr = exceptions::PyNameError;
const NO_GAME_ERR: &str = "GAME is unset. Call a function like load_standard first.";

const TPS: f32 = 120.;
const TICK_DT: f32 = 1. / TPS;

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
    doc: "rl_ball_sym is a Rust implementation of Rocket League's ball physics",
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
    classes: [HalfBallSlice, HalfBallPredictionStruct, BallSlice, BallPredictionStruct]
}

#[pyfunction]
fn load_standard() {
    let (game, ball) = rl_ball_sym::load_standard();
    GAME.set(Some(game));
    BALL.set(ball);
}

#[pyfunction]
fn load_dropshot() {
    let (game, ball) = rl_ball_sym::load_dropshot();
    GAME.set(Some(game));
    BALL.set(ball);
}

#[pyfunction]
fn load_hoops() {
    let (game, ball) = rl_ball_sym::load_hoops();
    GAME.set(Some(game));
    BALL.set(ball);
}

#[pyfunction]
fn load_standard_throwback() {
    let (game, ball) = rl_ball_sym::load_standard_throwback();
    GAME.set(Some(game));
    BALL.set(ball);
}

#[pyfunction]
fn tick(packet: GamePacket) -> PyResult<()> {
    GAME.with_borrow_mut(|game| {
        let Some(game) = game.as_mut() else {
            return Err(PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR));
        };

        packet.export_to_game(game);

        Ok(())
    })?;

    BALL.with_borrow_mut(|ball| {
        packet.export_to_ball(ball);
    });

    Ok(())
}

#[pyfunction]
fn step_ball() -> PyResult<BallSlice> {
    GAME.with_borrow(|game| {
        let Some(game) = game.as_ref() else {
            return Err(PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR));
        };

        BALL.with_borrow_mut(|ball| {
            ball.step(game, TICK_DT);
            Ok(BallSlice::from_rl_ball_sym(*ball))
        })
    })
}

#[pyfunction]
fn get_ball_prediction_struct() -> PyResult<HalfBallPredictionStruct> {
    GAME.with_borrow(|game| {
        let Some(game) = game.as_ref() else {
            return Err(PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR));
        };
        let ball = BALL.with_borrow(|ball| *ball);

        Ok(ball.get_ball_prediction_struct(game).into())
    })
}

#[pyfunction]
fn get_ball_prediction_struct_full() -> PyResult<BallPredictionStruct> {
    GAME.with_borrow(|game| {
        let Some(game) = game.as_ref() else {
            return Err(PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR));
        };
        let ball = BALL.with_borrow(|ball| *ball);

        Ok(ball.get_ball_prediction_struct(game).into())
    })
}

#[pyfunction]
fn get_ball_prediction_struct_for_time(time: f32) -> PyResult<HalfBallPredictionStruct> {
    GAME.with_borrow(|game| {
        let Some(game) = game.as_ref() else {
            return Err(PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR));
        };
        let ball = BALL.with_borrow(|ball| *ball);

        Ok(ball.get_ball_prediction_struct_for_time(game, time).into())
    })
}

#[pyfunction]
fn get_ball_prediction_struct_for_time_full(time: f32) -> PyResult<BallPredictionStruct> {
    GAME.with_borrow(|game| {
        let Some(game) = game.as_ref() else {
            return Err(PyErr::new::<NoGamePyErr, _>(NO_GAME_ERR));
        };
        let ball = BALL.with_borrow(|ball| *ball);

        Ok(ball.get_ball_prediction_struct_for_time(game, time).into())
    })
}
