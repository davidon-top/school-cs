use std::ffi::{CStr, CString};
use pyo3::ffi::c_str;
use pyo3::prelude::*;

static code: &'static str = include_str!("u2.py");

pub fn main() {
    Python::with_gil(|py| {
        let c = CString::new(code.as_bytes()).unwrap();
        let activator = PyModule::from_code(py, c.as_c_str(), c_str!("main.py"), c_str!("main")).unwrap();
        activator.call_method("main", (), None).unwrap();
    })
}
