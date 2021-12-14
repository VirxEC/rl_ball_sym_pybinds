extern crate cpython;
extern crate rl_ball_sym;

use cpython::{exc, py_fn, py_module_initializer, ObjectProtocol, PyDict, PyErr, PyObject, PyResult, Python, PythonObject};
use rl_ball_sym::simulation::ball::Ball;
use rl_ball_sym::simulation::game::Game;
use glam::Vec3A;

static mut GAME: Option<Game> = None;
const NO_GAME_ERR: &str = "GAME is unset. Call a function like load_soccar first.";

py_module_initializer!(rl_ball_sym, |py, m| {
    m.add(py, "__doc__", "rl_ball_sym is a Rust implementation of ball path prediction for Rocket League; Inspired by Samuel (Chip) P. Mish's C++ utils called RLUtilities")?;
    m.add(py, "load_soccar", py_fn!(py, load_soccar()))?;
    m.add(py, "load_dropshot", py_fn!(py, load_dropshot()))?;
    m.add(py, "load_hoops", py_fn!(py, load_hoops()))?;
    m.add(py, "load_soccar_throwback", py_fn!(py, load_soccar_throwback()))?;
    m.add(py, "set_gravity", py_fn!(py, set_gravity(gravity: PyDict)))?;
    m.add(py, "set_ball", py_fn!(py, set_ball(ball: PyDict)))?;
    m.add(py, "step_ball", py_fn!(py, step_ball(dt: f32)))?;
    m.add(py, "get_ball_prediction_struct", py_fn!(py, get_ball_prediction_struct()))?;
    m.add(py, "get_ball_prediction_struct_full", py_fn!(py, get_ball_prediction_struct_full()))?;
    m.add(py, "get_ball_prediction_struct_for_time", py_fn!(py, get_ball_prediction_struct_for_time(time: f32)))?;
    m.add(py, "get_ball_prediction_struct_for_time_full", py_fn!(py, get_ball_prediction_struct_for_time_full(time: f32)))?;
    Ok(())
});

fn load_soccar(py: Python) -> PyResult<PyObject> {
    unsafe {
        GAME = Some(rl_ball_sym::load_soccar());
    }

    Ok(py.None())
}

fn load_dropshot(py: Python) -> PyResult<PyObject> {
    unsafe {
        GAME = Some(rl_ball_sym::load_dropshot());
    }

    Ok(py.None())
}

fn load_hoops(py: Python) -> PyResult<PyObject> {
    unsafe {
        GAME = Some(rl_ball_sym::load_hoops());
    }

    Ok(py.None())
}

fn load_soccar_throwback(py: Python) -> PyResult<PyObject> {
    unsafe {
        GAME = Some(rl_ball_sym::load_soccar_throwback());
    }

    Ok(py.None())
}

fn set_gravity(py: Python, py_gravity: PyDict) -> PyResult<PyObject> {
    let mut game: &mut Game;

    unsafe {
        if GAME.is_none() {
            return Err(PyErr::new::<exc::NameError, _>(py, NO_GAME_ERR));
        }

        game = GAME.as_mut().unwrap();
    }

    game.gravity = Vec3A::new(
        py_gravity.get_item(py, 0).unwrap().extract(py)?,
        py_gravity.get_item(py, 1).unwrap().extract(py)?,
        py_gravity.get_item(py, 2).unwrap().extract(py)?,
    );

    Ok(py.None())
}

fn set_ball(py: Python, py_ball: PyDict) -> PyResult<PyObject> {
    let mut game: &mut Game;

    unsafe {
        if GAME.is_none() {
            return Err(PyErr::new::<exc::NameError, _>(py, NO_GAME_ERR));
        }

        game = GAME.as_mut().unwrap();
    }

    if let Some(time) = py_ball.get_item(py, "time") {
        game.ball.time = time.extract(py)?;
    }

    if let Some(location) = py_ball.get_item(py, "location") {
        game.ball.location = Vec3A::new(
            location.get_item(py, 0)?.extract(py)?,
            location.get_item(py, 1)?.extract(py)?,
            location.get_item(py, 2)?.extract(py)?,
        );
    }

    if let Some(velocity) = py_ball.get_item(py, "velocity") {
        game.ball.velocity = Vec3A::new(
            velocity.get_item(py, 0)?.extract(py)?,
            velocity.get_item(py, 1)?.extract(py)?,
            velocity.get_item(py, 2)?.extract(py)?,
        );
    }

    if let Some(angular_velocity) = py_ball.get_item(py, "angular_velocity") {
        game.ball.angular_velocity = Vec3A::new(
            angular_velocity.get_item(py, 0)?.extract(py)?,
            angular_velocity.get_item(py, 1)?.extract(py)?,
            angular_velocity.get_item(py, 2)?.extract(py)?,
        );
    }

    if let Some(radius) = py_ball.get_item(py, "radius") {
        game.ball.radius = radius.extract(py)?;
        game.ball.calculate_moi();
    }

    if let Some(collision_radius) = py_ball.get_item(py, "collision_radius") {
        game.ball.collision_radius = collision_radius.extract(py)?;
    }

    Ok(py.None())
}

fn step_ball(py: Python, dt: f32) -> PyResult<PyObject> {
    let game: &mut Game;

    unsafe {
        if GAME.is_none() {
            return Err(PyErr::new::<exc::NameError, _>(py, NO_GAME_ERR));
        }

        game = GAME.as_mut().unwrap();
    }

    Ball::step(game, dt);

    let slice = PyDict::new(py);
    slice.set_item(py, "time", game.ball.time).unwrap();

    slice.set_item(py, "location", vec![game.ball.location.x, game.ball.location.y, game.ball.location.z]).unwrap();
    slice.set_item(py, "velocity", vec![game.ball.velocity.x, game.ball.velocity.y, game.ball.velocity.z]).unwrap();
    slice.set_item(py, "angular_velocity", vec![game.ball.angular_velocity.x, game.ball.angular_velocity.y, game.ball.angular_velocity.z]).unwrap();

    Ok(slice.into_object())
}

fn get_ball_prediction_struct(py: Python) -> PyResult<PyObject> {
    let game: &mut Game;

    unsafe {
        if GAME.is_none() {
            return Err(PyErr::new::<exc::NameError, _>(py, NO_GAME_ERR));
        }

        game = GAME.as_mut().unwrap();
    }

    let raw_ball_struct = Ball::get_ball_prediction_struct(game);
    let mut slices = Vec::<PyObject>::with_capacity(raw_ball_struct.num_slices);
    let mut should_add = false;

    for raw_slice in raw_ball_struct.slices {
        should_add = !should_add;

        if should_add {
            let slice = PyDict::new(py);
            slice.set_item(py, "time", raw_slice.time).unwrap();

            slice.set_item(py, "location", vec![raw_slice.location.x, raw_slice.location.y, raw_slice.location.z]).unwrap();
            slice.set_item(py, "velocity", vec![raw_slice.velocity.x, raw_slice.velocity.y, raw_slice.velocity.z]).unwrap();

            slices.push(slice.into_object());
        }
    }

    let ball_struct = PyDict::new(py);
    ball_struct.set_item(py, "slices", &slices).unwrap();
    ball_struct.set_item(py, "num_slices", slices.len()).unwrap();

    Ok(ball_struct.into_object())
}

fn get_ball_prediction_struct_full(py: Python) -> PyResult<PyObject> {
    let game: &mut Game;

    unsafe {
        if GAME.is_none() {
            return Err(PyErr::new::<exc::NameError, _>(py, NO_GAME_ERR));
        }

        game = GAME.as_mut().unwrap();
    }

    let raw_ball_struct = Ball::get_ball_prediction_struct(game);
    let mut slices = Vec::<PyObject>::with_capacity(raw_ball_struct.num_slices);

    for raw_slice in raw_ball_struct.slices {
        let slice = PyDict::new(py);
        slice.set_item(py, "time", raw_slice.time).unwrap();

        slice.set_item(py, "location", vec![raw_slice.location.x, raw_slice.location.y, raw_slice.location.z]).unwrap();
        slice.set_item(py, "velocity", vec![raw_slice.velocity.x, raw_slice.velocity.y, raw_slice.velocity.z]).unwrap();
        slice.set_item(py, "angular_velocity", vec![raw_slice.angular_velocity.x, raw_slice.angular_velocity.y, raw_slice.angular_velocity.z]).unwrap();

        slices.push(slice.into_object());
    }

    let ball_struct = PyDict::new(py);
    ball_struct.set_item(py, "slices", slices).unwrap();
    ball_struct.set_item(py, "num_slices", raw_ball_struct.num_slices).unwrap();

    Ok(ball_struct.into_object())
}

fn get_ball_prediction_struct_for_time(py: Python, time: f32) -> PyResult<PyObject> {
    let game: &mut Game;

    unsafe {
        if GAME.is_none() {
            return Err(PyErr::new::<exc::NameError, _>(py, NO_GAME_ERR));
        }

        game = GAME.as_mut().unwrap();
    }

    let raw_ball_struct = Ball::get_ball_prediction_struct_for_time(game, &time);
    let mut slices = Vec::<PyObject>::with_capacity(raw_ball_struct.num_slices);
    let mut should_add = false;

    for raw_slice in raw_ball_struct.slices {
        should_add = !should_add;

        if should_add {
            let slice = PyDict::new(py);
            slice.set_item(py, "time", raw_slice.time).unwrap();

            slice.set_item(py, "location", vec![raw_slice.location.x, raw_slice.location.y, raw_slice.location.z]).unwrap();
            slice.set_item(py, "velocity", vec![raw_slice.velocity.x, raw_slice.velocity.y, raw_slice.velocity.z]).unwrap();

            slices.push(slice.into_object());
        }
    }

    let ball_struct = PyDict::new(py);
    ball_struct.set_item(py, "slices", &slices).unwrap();
    ball_struct.set_item(py, "num_slices", slices.len()).unwrap();

    Ok(ball_struct.into_object())
}

fn get_ball_prediction_struct_for_time_full(py: Python, time: f32) -> PyResult<PyObject> {
    let game: &mut Game;

    unsafe {
        if GAME.is_none() {
            return Err(PyErr::new::<exc::NameError, _>(py, NO_GAME_ERR));
        }

        game = GAME.as_mut().unwrap();
    }

    let raw_ball_struct = Ball::get_ball_prediction_struct_for_time(game, &time);
    let mut slices = Vec::<PyObject>::with_capacity(raw_ball_struct.num_slices);

    for raw_slice in raw_ball_struct.slices {
        let slice = PyDict::new(py);
        slice.set_item(py, "time", raw_slice.time).unwrap();

        slice.set_item(py, "location", vec![raw_slice.location.x, raw_slice.location.y, raw_slice.location.z]).unwrap();
        slice.set_item(py, "velocity", vec![raw_slice.velocity.x, raw_slice.velocity.y, raw_slice.velocity.z]).unwrap();
        slice.set_item(py, "angular_velocity", vec![raw_slice.angular_velocity.x, raw_slice.angular_velocity.y, raw_slice.angular_velocity.z]).unwrap();

        slices.push(slice.into_object());
    }

    let ball_struct = PyDict::new(py);
    ball_struct.set_item(py, "slices", slices).unwrap();
    ball_struct.set_item(py, "num_slices", raw_ball_struct.num_slices).unwrap();

    Ok(ball_struct.into_object())
}
