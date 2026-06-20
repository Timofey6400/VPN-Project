// ! A Rust library for a VPN core that can be called from Python using PyO3.
// ! This library simulates a VPN core that runs in a background thread and sends speed updates back to a Python GUI.
use pyo3::prelude::*;
use std::process::{Command, Stdio};
use std::thread;
use std::os::windows::process::CommandExt;

#[pyfunction]
fn start_core() {
    thread::spawn(move || {
        const CREATE_NO_WINDOW: u32 = 0x08000000;

        let mut child = Command::new("./xray.exe")
            .arg("run")
            .arg("-config")
            .arg("config.json")
            .creation_flags(CREATE_NO_WINDOW) // Hide the console window
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("Failed to start xray.exe");

        let _ = child.wait();
    });
}

#[pymodule]
fn vpn_core(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(start_core, m)?)?;
    Ok(())
}