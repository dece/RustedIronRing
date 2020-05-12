use std::collections::HashMap;

use pyo3::wrap_pymodule;
use pyo3::prelude::*;

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
fn pyironring(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pymodule!(name_hashes))?;
    Ok(())
}
