use pyo3::prelude::*;
use pyo3::types::{PyDict, PyModule};

use grieg_parser::parse_expr;
use grieg_engine::Evaluator;

// --- internal impl shared by both Python entry points ---
fn eval_impl(py: Python, expr: &str, mem: bool) -> PyResult<PyObject> {
    let parsed = parse_expr(expr)
        .map_err(|e| PyErr::new::<pyo3::exceptions::PyValueError, _>(format!("{e}")))?;
    let mut ev = Evaluator::new(mem);
    let res = ev.eval(&parsed, None);

    let out = PyDict::new_bound(py);
    out.set_item("expr", expr)?;
    out.set_item("mem", mem)?;
    out.set_item("phase", format!("{:?}", res.phase))?;
    out.set_item("value", format!("{:?}", res.value))?;
    Ok(out.into_py(py))
}

// Preferred API: expr(...)
#[pyfunction(name = "expr")]
fn expr_py(py: Python, expr: &str, mem: bool) -> PyResult<PyObject> {
    eval_impl(py, expr, mem)
}

// Back-compat alias: eval(...)
#[pyfunction(name = "eval")]
fn eval_py(py: Python, expr: &str, mem: bool) -> PyResult<PyObject> {
    eval_impl(py, expr, mem)
}

#[pymodule]
fn grieg(_py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(expr_py, m)?)?;
    m.add_function(wrap_pyfunction!(eval_py, m)?)?;
    Ok(())
}
