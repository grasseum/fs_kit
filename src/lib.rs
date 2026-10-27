#[cfg(feature = "python")]
use pyo3::prelude::*;

pub mod core_lib;
pub mod core_dir;
pub mod envs;
pub mod mapper;
pub mod enums;
pub mod structs;
pub mod ln_py;
pub mod ln_js;

#[cfg(feature = "python")]
#[pyfunction]
pub fn glob(patterns: &Bound<'_, PyAny>) -> PyResult<Vec<structs::FileInfo>> {
	let paths = match patterns.extract::<String>() {
		Ok(pattern) => vec![pattern],
		Err(_) => patterns.extract::<Vec<String>>()?,
	};
	Ok(core_lib::get_validate_dir(paths))
}

#[cfg(feature = "python")]
#[pymodule]
fn _fs_kit(module: &Bound<'_, PyModule>) -> PyResult<()> {
	module.add_class::<structs::FileInfo>()?;
	module.add_function(wrap_pyfunction!(glob, module)?)?;
	module.add_function(wrap_pyfunction!(core_lib::get_validate_dir, module)?)?;
	Ok(())
}
