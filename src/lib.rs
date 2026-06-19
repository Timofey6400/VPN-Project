// ! A Rust library for a VPN core that can be called from Python using PyO3.
// ! This library simulates a VPN core that runs in a background thread and sends speed updates back to a Python GUI.
use pyo3::prelude::*;
use std::thread;
use std::time::Duration;

#[pyfunction]
fn start_core(py_callback: PyObject) {
    // Start a new thread to simulate the VPN core
    thread::spawn(move || {
        let mut speed = 0;
        loop {
            thread::sleep(Duration::from_secs(1));
            speed += 100;

            // Send data back to Python
            Python::with_gil(|py| {
                let _ = py_callback.call1(py, (speed,));
            });
        }
    });
}

#[pymodule]
fn vpn_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start_core, m)?)?;
    Ok(())
}