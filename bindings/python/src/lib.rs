#[pyo3::pymodule]
mod alleged {
    use pyo3::prelude::*;

    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> String {
        (a + b).to_string()
    }
}
