use std::ffi::CString;
use pyo3::ffi::c_str;
use pyo3::prelude::{PyAnyMethods, PyModule};
use pyo3::Python;

static code: &'static str = include_str!("u43.py");

pub fn main() {
	Python::with_gil(|py| {
		let c = CString::new(code.as_bytes()).unwrap();
		let activator =
			PyModule::from_code(py, c.as_c_str(), c_str!("main.py"), c_str!("main")).unwrap();
		activator.call_method("main", (), None).unwrap();
	})
}