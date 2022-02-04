use glam::Vec3A;
use lazy_static::lazy_static;
use pyo3::types::PyDict;
use pyo3::{exceptions, prelude::*, PyErr};
use rl_ball_sym::simulation::ball::Ball;
use rl_ball_sym::simulation::game::Game;
use std::sync::Mutex;

lazy_static! {
    static ref GAME: Mutex<Option<Game>> = Mutex::new(None);
}

const NO_GAME_ERR: &str = "GAME is unset. Call a function like load_soccar first.";

#[pymodule]
fn rl_ball_sym_pybinds(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_soccar, m)?)?;
    m.add_function(wrap_pyfunction!(load_dropshot, m)?)?;
    m.add_function(wrap_pyfunction!(load_hoops, m)?)?;
    m.add_function(wrap_pyfunction!(load_soccar_throwback, m)?)?;
    m.add_function(wrap_pyfunction!(tick, m)?)?;
    m.add_function(wrap_pyfunction!(get_ball_prediction_struct, m)?)?;
    m.add_function(wrap_pyfunction!(get_ball_prediction_struct_for_time, m)?)?;
    m.add_function(wrap_pyfunction!(get_ball_prediction_struct_full, m)?)?;
    m.add_function(wrap_pyfunction!(get_ball_prediction_struct_for_time_full, m)?)?;
    Ok(())
}

#[pyfunction]
fn load_soccar() {
    let mut game_guard = GAME.lock().unwrap();
    *game_guard = Some(rl_ball_sym::load_soccar());
}

#[pyfunction]
fn load_dropshot() {
    let mut game_guard = GAME.lock().unwrap();
    *game_guard = Some(rl_ball_sym::load_dropshot());
}

#[pyfunction]
fn load_hoops() {
    let mut game_guard = GAME.lock().unwrap();
    *game_guard = Some(rl_ball_sym::load_hoops());
}

#[pyfunction]
fn load_soccar_throwback() {
    let mut game_guard = GAME.lock().unwrap();
    *game_guard = Some(rl_ball_sym::load_soccar_throwback());
}

fn get_vec3_named(py_vec: &PyAny) -> PyResult<Vec3A> {
    Ok(Vec3A::new(
        py_vec.getattr("x")?.extract()?,
        py_vec.getattr("y")?.extract()?,
        py_vec.getattr("z")?.extract()?,
    ))
}

#[pyfunction]
fn tick(py: Python, packet: PyObject) -> PyResult<()> {
    let mut game_guard = GAME.lock().unwrap();
    let game = game_guard.as_mut().ok_or_else(|| PyErr::new::<exceptions::PyNameError, _>(NO_GAME_ERR))?;

    let packet = packet.as_ref(py);

    let py_game_info = packet.getattr("game_info")?;

    let time = py_game_info.getattr("seconds_elapsed")?.extract::<f32>()?;

    game.gravity.z = py_game_info.getattr("world_gravity_z")?.extract()?;

    let py_ball = packet.getattr("game_ball")?;
    let py_ball_physics = py_ball.getattr("physics")?;

    game.ball.update(
        time,
        get_vec3_named(py_ball_physics.getattr("location")?)?,
        get_vec3_named(py_ball_physics.getattr("velocity")?)?,
        get_vec3_named(py_ball_physics.getattr("angular_velocity")?)?,
    );

    let py_ball_shape = py_ball.getattr("collision_shape")?;

    game.ball.radius = py_ball_shape.getattr("sphere")?.getattr("diameter")?.extract::<f32>()? / 2.;
    game.ball.collision_radius = game.ball.radius + 1.9;
    game.ball.calculate_moi();

    Ok(())
}

#[pyfunction]
fn get_ball_prediction_struct(py: Python) -> PyResult<&PyDict> {
    let mut game_guard = GAME.lock().unwrap();
    let game = game_guard.as_mut().ok_or_else(|| PyErr::new::<exceptions::PyNameError, _>(NO_GAME_ERR))?;

    let raw_ball_struct = Ball::get_ball_prediction_struct(game);
    let mut slices = Vec::with_capacity(raw_ball_struct.num_slices);
    let mut should_add = false;

    for raw_slice in raw_ball_struct.slices {
        should_add = !should_add;

        if should_add {
            let slice = PyDict::new(py);
            slice.set_item("time", raw_slice.time).unwrap();

            slice.set_item("location", vec![raw_slice.location.x, raw_slice.location.y, raw_slice.location.z]).unwrap();
            slice.set_item("velocity", vec![raw_slice.velocity.x, raw_slice.velocity.y, raw_slice.velocity.z]).unwrap();

            slices.push(slice);
        }
    }

    let ball_struct = PyDict::new(py);
    ball_struct.set_item("num_slices", slices.len()).unwrap();
    ball_struct.set_item("slices", slices).unwrap();

    Ok(ball_struct)
}

#[pyfunction]
fn get_ball_prediction_struct_full(py: Python) -> PyResult<&PyDict> {
    let mut game_guard = GAME.lock().unwrap();
    let game = game_guard.as_mut().ok_or_else(|| PyErr::new::<exceptions::PyNameError, _>(NO_GAME_ERR))?;

    let raw_ball_struct = Ball::get_ball_prediction_struct(game);
    let mut slices = Vec::with_capacity(raw_ball_struct.num_slices);

    for raw_slice in raw_ball_struct.slices {
        let slice = PyDict::new(py);
        slice.set_item("time", raw_slice.time).unwrap();

        slice.set_item("location", vec![raw_slice.location.x, raw_slice.location.y, raw_slice.location.z]).unwrap();
        slice.set_item("velocity", vec![raw_slice.velocity.x, raw_slice.velocity.y, raw_slice.velocity.z]).unwrap();
        slice
            .set_item(
                "angular_velocity",
                vec![raw_slice.angular_velocity.x, raw_slice.angular_velocity.y, raw_slice.angular_velocity.z],
            )
            .unwrap();

        slices.push(slice);
    }

    let ball_struct = PyDict::new(py);
    ball_struct.set_item("slices", slices).unwrap();
    ball_struct.set_item("num_slices", raw_ball_struct.num_slices).unwrap();

    Ok(ball_struct)
}

#[pyfunction]
fn get_ball_prediction_struct_for_time(py: Python, time: f32) -> PyResult<&PyDict> {
    let mut game_guard = GAME.lock().unwrap();
    let game = game_guard.as_mut().ok_or_else(|| PyErr::new::<exceptions::PyNameError, _>(NO_GAME_ERR))?;

    let raw_ball_struct = Ball::get_ball_prediction_struct_for_time(game, &time);
    let mut slices = Vec::with_capacity(raw_ball_struct.num_slices);
    let mut should_add = false;

    for raw_slice in raw_ball_struct.slices {
        should_add = !should_add;

        if should_add {
            let slice = PyDict::new(py);
            slice.set_item("time", raw_slice.time).unwrap();

            slice.set_item("location", vec![raw_slice.location.x, raw_slice.location.y, raw_slice.location.z]).unwrap();
            slice.set_item("velocity", vec![raw_slice.velocity.x, raw_slice.velocity.y, raw_slice.velocity.z]).unwrap();

            slices.push(slice);
        }
    }

    let ball_struct = PyDict::new(py);
    ball_struct.set_item("slices", &slices).unwrap();
    ball_struct.set_item("num_slices", slices.len()).unwrap();

    Ok(ball_struct)
}

#[pyfunction]
fn get_ball_prediction_struct_for_time_full(py: Python, time: f32) -> PyResult<&PyDict> {
    let mut game_guard = GAME.lock().unwrap();
    let game = game_guard.as_mut().ok_or_else(|| PyErr::new::<exceptions::PyNameError, _>(NO_GAME_ERR))?;

    let raw_ball_struct = Ball::get_ball_prediction_struct_for_time(game, &time);
    let mut slices = Vec::with_capacity(raw_ball_struct.num_slices);

    for raw_slice in raw_ball_struct.slices {
        let slice = PyDict::new(py);
        slice.set_item("time", raw_slice.time).unwrap();

        slice.set_item("location", vec![raw_slice.location.x, raw_slice.location.y, raw_slice.location.z]).unwrap();
        slice.set_item("velocity", vec![raw_slice.velocity.x, raw_slice.velocity.y, raw_slice.velocity.z]).unwrap();
        slice
            .set_item(
                "angular_velocity",
                vec![raw_slice.angular_velocity.x, raw_slice.angular_velocity.y, raw_slice.angular_velocity.z],
            )
            .unwrap();

        slices.push(slice);
    }

    let ball_struct = PyDict::new(py);
    ball_struct.set_item("slices", slices).unwrap();
    ball_struct.set_item("num_slices", raw_ball_struct.num_slices).unwrap();

    Ok(ball_struct)
}
