use std::collections::HashMap;

use pyo3::wrap_pymodule;
use pyo3::exceptions::RuntimeError;
use pyo3::prelude::*;

fn runtime_error(message: String) -> PyErr {
    RuntimeError::py_err(message)
}

fn wrap_ironring_errors<T, E: std::fmt::Debug>(result: Result<T, E>) -> PyResult<T> {
    match result {
        Ok(r) => Ok(r),
        Err(e) => Err(runtime_error(format!("{:?}", e))),
    }
}

#[pymodule]
fn name_hashes(_py: Python, m: &PyModule) -> PyResult<()> {
    use ironring::name_hashes::*;

    #[pyfn(m, "hash")]
    fn py_hash(_py: Python, s: &str) -> PyResult<u32> {
        Ok(hash(s))
    }

    #[pyfn(m, "hash_as_string")]
    fn py_hash_as_string(_py: Python, h: u32) -> PyResult<String> {
        Ok(hash_as_string(h))
    }

    #[pyfn(m, "load_name_map")]
    fn py_load_name_map(_py: Python, path: &str) -> PyResult<HashMap<String, String>> {
        Ok(load_name_map(path)?)
    }

    Ok(())
}

#[pymodule]
fn unpack_bnd(_py: Python, m: &PyModule) -> PyResult<()> {
    use ironring::unpackers::bnd::*;
    use ironring::parsers::bnd::Bnd;

    #[pyfn(m, "load_bnd_file")]
    fn py_load_bnd_file(_py: Python, bnd_path: &str) -> PyResult<(Bnd, Vec<u8>)> {
        wrap_ironring_errors(load_bnd_file(bnd_path))
    }

    #[pyfn(m, "load_bnd")]
    fn py_load_bnd(_py: Python, bnd_data: &[u8]) -> PyResult<Bnd> {
        wrap_ironring_errors(load_bnd(bnd_data))
    }

    Ok(())
}

#[pymodule]
fn pyironring(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(name_hashes))?;
    m.add_wrapped(wrap_pymodule!(unpack_bnd))?;
    Ok(())
}
