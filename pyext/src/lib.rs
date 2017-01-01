#[macro_use]
extern crate cpython;

use cpython::{Python, PyResult};

fn count_doubles(_py: Python, val: &str) -> PyResult<u64> {
    let mut total = 0u64;
    for (c1, c2) in val.chars().zip(val.chars().skip(1)) {
        if c1 == c2 {
            total += 1;
        }
    }
    Ok(total)
}

py_module_initializer!(rdoubles, initrdoubles, PyInit_rdoubles, |py, m| {
    m.add(py, "count_doubles", py_fn!(py, count_doubles(val: &str)))?;

    Ok(())
});
