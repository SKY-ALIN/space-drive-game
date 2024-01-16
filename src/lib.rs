use pyo3::prelude::*;

pub fn get_version() -> String {
    let version = env!("CARGO_PKG_VERSION").to_string();
    version.replace("-alpha", "a").replace("-beta", "b")
}

pub fn get_authors() -> Vec<String> {
    let authors = env!("CARGO_PKG_AUTHORS");
    authors.split(':').map(str::to_string).collect()
}

#[pymodule]
fn space_drive_game(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add("__version__", get_version())?;
    m.add("__authors__", get_authors())?;
    Ok(())
}
